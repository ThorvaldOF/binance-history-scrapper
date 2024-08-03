mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::{fs, io, thread};
use std::fmt::format;
use std::sync::{Arc, Mutex};
use chrono::Datelike;
use chrono::prelude::Local;
use clap::builder::Str;
use serde::{Deserialize, Serialize};
use sha2::digest::typenum::NInt;
use ureq::{Agent, AgentBuilder};
use crate::utils::asset_file::AssetFile;
use crate::download::{download_file};
use crate::extract::{extract_file};
use crate::input::Settings;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::{DatePeriod, Manifest, TimePeriod};

const BINANCE_BIRTH: i32 = 2017;

//TODO: check all the project and rename stuff
#[derive(Clone)]
pub struct ProcessData {
    pub granularity: String,
    pub asset: String,
    pub clear_cache: bool,
}

fn main() {
    let settings = input::process_input();
    let clear_cache = settings.clear_cache;

    handle_processes(settings);

    if clear_cache {
        fs::remove_dir_all(AssetFile::get_cache_directory()).expect("Couldn't clear downloads directory");
    }
    println!("Scrapping completed, you can find your output in 'results' directory");
    println!("Press enter to quit...");
    let mut useless_input = String::new();
    io::stdin().read_line(&mut useless_input).expect("Couldn't retrieve your input, please try again");
}

fn handle_processes(settings: Settings) {
    let mut processes_vec: Vec<ProcessData> = vec![];
    for asset in settings.assets {
        let process_data = ProcessData { asset, granularity: settings.granularity.clone(), clear_cache: settings.clear_cache };
        processes_vec.push(process_data);
    }
    let processes = Arc::new(Mutex::new(processes_vec));
    let manifest = Arc::new(Mutex::new(Manifest::new()));

    let mut handles = vec![];
    for _ in 0..4 {
        let processes_clone = Arc::clone(&processes);
        let manifest_clone = Arc::clone(&manifest);
        let handle = thread::spawn(move || process_worker(processes_clone, manifest_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    match manifest.lock() {
        Ok(man) => {
            if man.save().is_err() {
                //TODO: maybe a better error handling
                println!("Couldn't save manifest");
            }
        }
        Err(_) => {
            //TODO: maybe a better error handling
            println!("Couldn't save manifest");
        }
    };
}

//START END DATE
fn process_worker(processes: Arc<Mutex<Vec<ProcessData>>>, manifest: Arc<Mutex<Manifest>>) {
    let agent: Agent = AgentBuilder::new()
        .build();
    loop {
        let mut processes = match processes.lock() {
            Ok(data) => data,
            Err(_) => {
                //TODO: maybe a better error handling
                break;
            }
        };
        if processes.is_empty() {
            //No process remaining
            break;
        }

        let process_data = processes.remove(0);
        drop(processes);
        println!("[{}] Processing...", process_data.asset);
        let results = process(process_data.clone(), agent.clone());
        if let Some((_, date_period)) = results {
            let mut manifest = match manifest.lock() {
                Ok(man) => man,
                Err(_) => {
                    //TODO: maybe a better error handling
                    continue;
                }
            };
            manifest.add_asset(&process_data.granularity, &process_data.asset, date_period)
        }
    }
}

fn process(process: ProcessData, agent: Agent) -> Option<(Vec<TimePeriod>, DatePeriod)> {
    let today = Local::now();
    let mut first_iter = true;
    let mut start_date = String::new();
    let end_date = format!("{}-{}", month_to_string(today.month()), today.year());
    'process: for year in (BINANCE_BIRTH..today.year()).rev() {
        let mut max_month = 12;
        if year == today.year() {
            max_month = today.month();
        }
        for month in (1..=max_month).rev() {
            let asset_file = AssetFile::new(&process.asset, &process.granularity, year, month, agent.clone());

            if let Err(err) = download_file(&asset_file) {
                match err {
                    ScrapperError::NoOnlineData => {
                        if first_iter {
                            println!("[{}] No data available", &process.asset);
                            return None;
                        } else {
                            println!("[{}] Finished, no data available before {}/{} (included)", &process.asset, month, year);
                            start_date = format!("{}-{}", month_to_string(month), year);
                            break 'process;
                        }
                    }
                    _ => {
                        //TODO: Pause on error, ask the user to fix the problem, then press enter to continue
                        println!("[{}] Download error, details: {}", &process.asset, err);
                        return None;
                    }
                };
            }
            if let Err(err) = extract_file(&asset_file, process.clear_cache) {
                //TODO: Pause on error, ask the user to fix the problem, then press enter to continue
                println!("[{}] Extraction error, details: {}", &process.asset, err);
                return None;
            }
            if first_iter {
                first_iter = false;
            }
        }
    }
    Some((vec![], DatePeriod::new(&start_date, &end_date)))
}

fn month_to_string(month: u32) -> String {
    let prefix = if month < 10 {
        "0"
    } else {
        ""
    };
    format!("{}{}", prefix, month)
}