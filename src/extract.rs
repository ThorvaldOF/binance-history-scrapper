use std::fs::{File, create_dir_all, remove_file, metadata};
use std::io::{Read};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use zip::ZipArchive;
use crate::utils::asset_file::AssetFile;
use crate::utils::errors::ScrapperError;

pub fn extract_file(asset_file: &AssetFile, clear_cache: bool) -> Result<(), ScrapperError> {
    let output_file_path = asset_file.get_extract_directory() + &asset_file.get_full_file_name(".csv");
    if metadata(&output_file_path).is_ok() {
        remove_file(output_file_path.clone())?;
    }

    let source_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");
    let source_file = File::open(source_path.clone())?;

    let mut archive = ZipArchive::new(source_file)?;

    create_dir_all(asset_file.get_extract_directory())?;

    let mut entry = archive.by_index(0)?;
    let output_file = File::create(&output_file_path)?;

    let mut csv_content = String::new();
    entry.read_to_string(&mut csv_content)?;

    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(csv_content.as_bytes());

    let mut csv_writer = WriterBuilder::new().from_writer(output_file);

    for result in csv_reader.records() {
        let record = result?;
        csv_writer.write_record(filter_record(record).iter())?;
    }
    if clear_cache {
        remove_file(source_path)?;
    }
    Ok(())
}

fn filter_record(record: StringRecord) -> StringRecord {
    let collected_record: Vec<&str> = record.iter().collect();

    //Format based on https://github.com/binance/binance-public-data/
    let mut processed_record: StringRecord = StringRecord::new();
    for i in 0..=6 {
        processed_record.push_field(collected_record[i]);
    }
    processed_record
}
