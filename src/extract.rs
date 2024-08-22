use std::fs::{File, create_dir_all, remove_file, metadata};
use std::io::{Read, Write};
use csv::{ReaderBuilder, StringRecord};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;
use crate::utils::asset_file::AssetFile;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::TimePeriod;
use crate::utils::month_year::MonthYear;
use crate::utils::process_data::ProcessData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedData {
    open_time: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl PartialEq for ExtractedData {
    fn eq(&self, other: &Self) -> bool {
        self.open_time == other.open_time &&
            self.open == other.open &&
            self.high == other.high &&
            self.low == other.low &&
            self.close == other.close &&
            self.volume == other.volume
    }
}

pub fn extract_asset(process: &mut ProcessData, start_time: MonthYear) -> Result<(Vec<TimePeriod>, TimePeriod), ScrapperError> {
    let end_time = process.get_end();

    let global_asset_file = AssetFile::new(&process.get_asset(), &process.get_granularity(), start_time.clone());

    init_result_file(&global_asset_file)?;
    let mut extracted_data: Vec<ExtractedData> = vec![];

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
            extracted_data.extend(extract_file(&asset_file)?);
        }
    }
    let asset_data = post_treatment(&global_asset_file, &mut extracted_data)?;
    save_file(&global_asset_file, &extracted_data)?;
    check_file_integrity(&global_asset_file.get_result_file_path(), extracted_data)?;
    Ok(asset_data)
}

pub fn extract_file(asset_file: &AssetFile) -> Result<Vec<ExtractedData>, ScrapperError> {
    let source_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");
    let source_file = File::open(source_path.clone())?;

    let mut archive = ZipArchive::new(source_file)?;

    let mut entry = archive.by_index(0)?;

    let mut csv_content = String::new();
    entry.read_to_string(&mut csv_content)?;

    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(csv_content.as_bytes());

    let mut extracted_records: Vec<ExtractedData> = vec![];

    for csv_record in csv_reader.records() {
        let record = csv_record?;
        extracted_records.push(extract_record(record)?);
    }
    Ok(extracted_records)
}


fn extract_record(record: StringRecord) -> Result<ExtractedData, ScrapperError> {
    let collected_record: Vec<&str> = record.iter().collect();

    //Format based on https://github.com/binance/binance-public-data/
    let open_time = collected_record[0].parse::<u64>()?;
    let open = collected_record[1].parse::<f64>()?;
    let high = collected_record[2].parse::<f64>()?;
    let low = collected_record[3].parse::<f64>()?;
    let close = collected_record[4].parse::<f64>()?;
    let volume = collected_record[5].parse::<f64>()?;
    Ok(ExtractedData { open_time, open, high, low, close, volume })
}

pub fn init_result_file(asset_file: &AssetFile) -> Result<(), ScrapperError> {
    let path = asset_file.get_result_file_path();
    if metadata(&path).is_ok() {
        remove_file(path)?;
    }
    create_dir_all(asset_file.get_extract_directory())?;
    Ok(())
}

pub fn post_treatment(asset_file: &AssetFile, extracted_data: &mut Vec<ExtractedData>) -> Result<(Vec<TimePeriod>, TimePeriod), ScrapperError> {
    extracted_data.sort_by(|a, b| a.open_time.cmp(&b.open_time));
    let start_ts = extracted_data.get(0)
        .ok_or(ScrapperError::IntegrityError("No data found in extracted_data".to_string()))?
        .open_time;
    let mut last_ts = 0;
    let mut down_periods: Vec<TimePeriod> = vec![];

    for entry in extracted_data {
        let ts = entry.open_time;

        //TODO: checker si les timestamps sont bien un multiple du facteur de granularité, sinon on considère que c'est des données corrompues
        if last_ts == 0 {
            last_ts = ts;
        }
        if ts < last_ts {
            return Err(ScrapperError::IntegrityError("Timestamps are not in the wright order:".to_string()));
        }
        if ts - last_ts > asset_file.get_ts_factor() || !is_multiple_of_granularity(ts, asset_file.get_ts_factor()) {
            let down_period = TimePeriod::new(last_ts, ts);
            down_periods.push(down_period);
        }
        last_ts = ts;
    }
    Ok((down_periods, TimePeriod::new(start_ts, last_ts)))
}

fn save_file(asset_file: &AssetFile, extracted_data: &Vec<ExtractedData>) -> Result<(), ScrapperError> {
    let encoded_data = bincode::serialize(&extracted_data)?;

    let path = asset_file.get_result_file_path();
    let mut file = File::create(path)?;
    file.write_all(&encoded_data)?;
    file.flush()?;
    Ok(())
}

fn is_multiple_of_granularity(timestamp: u64, factor: u64) -> bool {
    timestamp % factor == 0
}

fn check_file_integrity(path: &str, reference_data: Vec<ExtractedData>) -> Result<(), ScrapperError> {
    let mut file = File::open(path)?;

    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    let data: Vec<ExtractedData> = bincode::deserialize(&encoded_data)?;

    for (i, entry) in data.iter().enumerate() {
        if entry != &reference_data[i] {
            return Err(ScrapperError::IntegrityError("Data integrity check failed".to_string()));
        }
    }
    Ok(())
}
