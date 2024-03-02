use std::fs::{File, create_dir_all, remove_file};
use std::io::{Read};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use zip::ZipArchive;
use crate::download::DOWNLOADS_PATH;
use crate::input::Settings;
use crate::LOCAL_PATH;

const RESULTS_PATH: &str = "results/";

pub fn extract_file(settings: &Settings, file_name: &str) -> Result<bool, std::io::Error> {
    let file_directory = format!("{}{}{}/{}/", LOCAL_PATH, RESULTS_PATH, settings.symbol, settings.granularity);

    let source_path = format!("{}{}{}/{}/{}.zip", LOCAL_PATH, DOWNLOADS_PATH, settings.symbol, settings.granularity, file_name);
    let source_file = File::open(source_path.clone())?;

    let mut archive = ZipArchive::new(source_file)?;

    if archive.len() != 1 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid ZIP file structure"));
    }
    create_dir_all(file_directory.clone())?;

    let mut entry = archive.by_index(0)?;
    let output_file_path = format!("{}{}.csv", file_directory, file_name);
    let output_file = File::create(&output_file_path)?;


    let mut csv_content = String::new();
    entry.read_to_string(&mut csv_content)?;

    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(csv_content.as_bytes());

    let mut csv_writer = WriterBuilder::new().from_writer(output_file);

    for result in csv_reader.records() {
        let record = result?;
        let collected_record: Vec<&str> = record.iter().collect();

        //Format based on https://github.com/binance/binance-public-data/
        let mut processed_record: StringRecord = StringRecord::new();
        for i in 0..=6 {
            processed_record.push_field(collected_record[i]);
        }

        csv_writer.write_record(processed_record.iter())?;
    }
    if settings.clear_cache {
        remove_file(source_path)?;
    }
    Ok(true)
}
