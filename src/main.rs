mod input;
mod download;
mod extract;
mod utils;

use std::{fs, io, thread};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use chrono::Datelike;
use chrono::prelude::Local;
use crate::utils::asset_file::AssetFile;
use crate::download::{download_file};
use crate::extract::{extract_file};
use crate::input::Settings;

const BINANCE_BIRTH: i32 = 2017;

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
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    for _ in 0..4 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || process_worker(rx_clone));
        handles.push(handle);
    }

    for asset in settings.assets {
        let process_data = ProcessData { asset, granularity: settings.granularity.clone(), clear_cache: settings.clear_cache };
        tx.send(process_data).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn process_worker(rx: Arc<Mutex<Receiver<ProcessData>>>) {
    loop {
        let process_data = match rx.lock().unwrap().recv() {
            Ok(process_data) => process_data,
            Err(_) => break,
        };
        process(process_data);
    }
}

fn process(process: ProcessData) {
    let today = Local::now();
    let mut first_iter = true;
    'process: for year in (BINANCE_BIRTH..today.year()).rev() {
        let mut max_month = 12;
        if year == today.year() {
            max_month = today.month();
        }
        for month in (1..=max_month).rev() {
            let asset_file = AssetFile::new(&process.asset, &process.granularity, year, month);
            let display_name = asset_file.get_display_name();
            println!("Processing {} ", display_name);

            match download_file(&asset_file) {
                Ok(false) => {
                    if first_iter {
                        println!("No data available for [{}] finished", &process.asset);
                    } else {
                        println!("Download of [{}] finished, no data available before {}/{} (included)", &process.asset, month, year);
                    }
                    break 'process;
                }
                Err(err) => {
                    println!("An error occured while downloading {}, details: {}", display_name, err);
                    break 'process;
                }
                Ok(true) => {}
            }
            match extract_file(&asset_file, process.clear_cache) {
                Err(err) => {
                    println!("An error occurred while extracting {}, details: {}", display_name, err);
                    break 'process;
                }
                _ => {}
            }
            if first_iter {
                first_iter = false;
            }
        }
    }
}