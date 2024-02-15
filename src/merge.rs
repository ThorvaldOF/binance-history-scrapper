use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use crate::input::Settings;
use crate::extract::EXTRACTS_PATH;

const RESULTS_PATH: &str = "./results/";

pub fn merge_files(settings: &Settings) -> Result<bool, std::io::Error> {
    fs::create_dir_all(RESULTS_PATH)?;

    let target_file = format!("{}{}{}.csv", RESULTS_PATH, settings.symbol, settings.granularity);

    let source_dir = format!("{}{}/", EXTRACTS_PATH, settings.symbol);

    let output_file = File::create(target_file)?;
    let mut csv_writer = WriterBuilder::new().from_writer(output_file);

    let mut csv_files: Vec<String> = fs::read_dir(source_dir.clone())?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().into_string().ok())
                .filter(|name| name.ends_with(".csv"))
        })
        .collect();

    csv_files.sort();

    for file_name in csv_files {
        let input_path = Path::new(&source_dir).join(file_name);

        let  input_file = File::open(&input_path)?;

        let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(input_file);
        for result in csv_reader.records() {
            let record = result?;
            let collected_record:Vec<&str>  = record.iter().collect();

            let mut processed_record:StringRecord = StringRecord::new();
            for i in 1..=5{
                processed_record.push_field(collected_record[i]);
            }

            csv_writer.write_record(processed_record.iter())?;
        }
        if settings.clear_extracts {
            fs::remove_file(input_path)?;
        }
    }
    Ok(true)
}