mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::{fs, thread};
use std::fmt::format;
use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::{Datelike, TimeZone};
use chrono::prelude::Local;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
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

//TODO: Some kind of progress bar
fn main() {
    let settings = input::process_input();
    let clear_cache = settings.clear_cache;
    handle_processes(settings);

    if clear_cache {
        fs::remove_dir_all(AssetFile::get_cache_directory()).expect("Couldn't clear downloads directory");
    }
    println!("Scrapping completed, you can find your output in 'results' directory");
}

fn handle_processes(settings: Settings) {
    let m = MultiProgress::new();

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
        let multi_progress = m.clone();
        let handle = thread::spawn(move || process_worker(processes_clone, manifest_clone, multi_progress));
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

fn process_worker(processes: Arc<Mutex<Vec<ProcessData>>>, manifest: Arc<Mutex<Manifest>>, multi_progress: MultiProgress) {
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
        let results = process(process_data.clone(), agent.clone(), multi_progress.clone());
        if let Some((down_times, date_period)) = results {
            let mut manifest = match manifest.lock() {
                Ok(man) => man,
                Err(_) => {
                    //TODO: maybe a better error handling
                    continue;
                }
            };
            manifest.add_asset(&process_data.granularity, &process_data.asset, date_period);
            for down_time in down_times {
                manifest.add_down_time(&process_data.granularity, down_time);
            }
        }
    }
}

//TODO: refactoring
fn process(process: ProcessData, agent: Agent, multi_progress: MultiProgress) -> Option<(Vec<TimePeriod>, DatePeriod)> {
    //TODO: regrouper tout dans un struct, et calculer en amont les dates (thread principal)
    let today = Local::now();
    let start_time: (i32, u32) = if today.month() <= 2 {
        let new_month = if today.month() == 2 {
            12
        } else {
            11
        };
        (today.year() - 1, new_month)
    } else {
        (today.year(), today.month() - 2)
    };

    let mut first_iter = true;
    let mut start_date = String::new();
    let mut last_ts: u64 = 0;
    let end_date = format!("{}-{}", month_to_string(start_time.1), start_time.0);
    let mut down_times: Vec<TimePeriod> = vec![];

    let full_years = start_time.0 - BINANCE_BIRTH - 1;
    let bar_size = full_years as u32 * 12 + start_time.1;

    //TODO: calculate length of bar
    let sty = ProgressStyle::with_template(
        "[{prefix}] {bar:40.green/blue} {pos:>7}/{len:10} {msg}",
    )
        .unwrap()
        .progress_chars("##-");
    let err_sty = ProgressStyle::with_template(
        "[{prefix}] {bar:40.red/white} {pos:>7}/{len:10} {msg}",
    )
        .unwrap()
        .progress_chars("##-");

    let pb = multi_progress.add(ProgressBar::new(bar_size as u64));
    pb.set_style(sty);
    pb.set_prefix(process.asset.clone());

    'process: for year in (BINANCE_BIRTH..=start_time.0).rev() {
        let mut max_month = 12;
        if year == start_time.0 {
            max_month = start_time.1;
        }
        for month in (1..=max_month).rev() {
            let asset_file = AssetFile::new(&process.asset, &process.granularity, year, month, agent.clone());

            if let Err(err) = download_file(&asset_file) {
                match err {
                    ScrapperError::NoOnlineData => {
                        if first_iter {
                            pb.finish_with_message("no data available");
                            return None;
                        } else {
                            println!("[{}] Finished, no data available before {}/{} (included)", &process.asset, month, year);
                            start_date = format!("{}-{}", month_to_string(month), year);
                            break 'process;
                        }
                    }
                    _ => {
                        //TODO: Pause on error, ask the user to fix the problem, then press enter to continue
                        pb.set_style(err_sty);
                        pb.finish_with_message(format!("download error: {}", err));
                        return None;
                    }
                };
            }
            match extract_file(&asset_file, process.clear_cache, last_ts) {
                Ok(mut res) => {
                    if !res.0.is_empty() {
                        down_times.append(&mut res.0);
                    }
                    last_ts = res.1;
                }
                Err(err) => {
                    //TODO: Pause on error, ask the user to fix the problem, then press enter to continue
                    pb.set_style(err_sty);
                    pb.finish_with_message(format!("extraction error: {}", err));
                    return None;
                }
            }
            if first_iter {
                first_iter = false;
            }
            pb.inc(1);
        }
    }
    pb.finish_with_message(format!("last data {}", start_date));
    Some((down_times, DatePeriod::new(&start_date, &end_date)))
}

fn month_to_string(month: u32) -> String {
    let prefix = if month < 10 {
        "0"
    } else {
        ""
    };
    format!("{}{}", prefix, month)
}