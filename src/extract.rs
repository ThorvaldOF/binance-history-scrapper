use std::fs::{File, create_dir_all, remove_file, metadata, OpenOptions};
use std::io::{Read};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use zip::ZipArchive;
use crate::utils::asset_file::AssetFile;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::TimePeriod;
use crate::utils::month_year::MonthYear;
use crate::utils::process_data::ProcessData;

pub fn extract_asset(process: &mut ProcessData, start_time: MonthYear) -> Result<(Vec<TimePeriod>, TimePeriod), ScrapperError> {
    let end_time = process.get_end();

    let global_asset_file = AssetFile::new(&process.get_asset(), &process.get_granularity(), start_time.clone());

    init_result_file(&global_asset_file)?;

    for year in start_time.get_year()..=end_time.get_year() {
        let max_month = if year == end_time.get_year() {
            end_time.get_month()
        } else {
            12
        };
        let min_month = if year == start_time.get_year() {
            start_time.get_month()
        } else { 1 };
        for month in min_month..=max_month {
            let month_year = MonthYear::new(month, year);
            let asset_file = AssetFile::new(&process.get_asset(), &process.get_granularity(), month_year.clone());
            extract_file(&asset_file)?;
        }
    }
    let asset_data = post_treatment(&global_asset_file)?;
    Ok(asset_data)
}

pub fn extract_file(asset_file: &AssetFile) -> Result<(), ScrapperError> {
    let output_file_path = asset_file.get_result_file_path();

    let source_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");
    let source_file = File::open(source_path.clone())?;

    let mut archive = ZipArchive::new(source_file)?;


    let mut entry = archive.by_index(0)?;
    let output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_file_path)?;

    let mut csv_content = String::new();
    entry.read_to_string(&mut csv_content)?;

    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(csv_content.as_bytes());

    let mut csv_writer = WriterBuilder::new().from_writer(output_file);

    for result in csv_reader.records() {
        let record = result?;
        csv_writer.write_record(filter_record(record).iter())?;
    }
    Ok(())
}


fn filter_record(record: StringRecord) -> StringRecord {
    let collected_record: Vec<&str> = record.iter().collect();

    //Format based on https://github.com/binance/binance-public-data/
    let mut processed_record: StringRecord = StringRecord::new();
    for i in 0..=5 {
        processed_record.push_field(collected_record[i]);
    }
    processed_record
}

pub fn init_result_file(asset_file: &AssetFile) -> Result<(), ScrapperError> {
    let path = asset_file.get_result_file_path();
    if metadata(&path).is_ok() {
        remove_file(path)?;
    }
    create_dir_all(asset_file.get_extract_directory())?;
    Ok(())
}

pub fn post_treatment(asset_file: &AssetFile) -> Result<(Vec<TimePeriod>, TimePeriod), ScrapperError> {
    let path = asset_file.get_result_file_path();
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut start_ts = 0;
    let mut last_ts = 0;
    let mut down_periods: Vec<TimePeriod> = vec![];

    let mut i = 0;
    for result in reader.records() {
        let record = result?;
        let ts_str = record.get(0).ok_or(ScrapperError::ParseError("Couldn't get csv record:".to_string()))?;
        let ts: u64 = ts_str.parse().ok().ok_or(ScrapperError::ParseError("Couldn't parse csv record:".to_string()))?;

        if last_ts == 0 {
            last_ts = ts;
        }
        if start_ts == 0 {
            start_ts = ts;
        }
        if ts < last_ts {
            return Err(ScrapperError::IntegrityError("Timestamps are not in the wright order:".to_string()));
        }
        if ts - last_ts > asset_file.get_ts_factor() {
            let down_period = TimePeriod::new(last_ts, ts);
            down_periods.push(down_period);
        }
        last_ts = ts;
        i = i + 1;
    }
    Ok((down_periods, TimePeriod::new(start_ts, last_ts)))
}

