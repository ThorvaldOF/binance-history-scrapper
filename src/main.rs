mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::{fs, io, thread};
use std::sync::{Arc, Mutex};
use chrono::Datelike;
use chrono::prelude::Local;
use ureq::{Agent, AgentBuilder};
use crate::utils::asset_file::AssetFile;
use crate::download::{download_file};
use crate::extract::{extract_file};
use crate::input::Settings;
use crate::utils::errors::ScrapperError;

const BINANCE_BIRTH: i32 = 2017;

pub struct ProcessData {
    pub granularity: String,
    pub asset: String,
    pub clear_cache: bool,
}

fn main() {
    let settings = input::process_input();
    let clear_cache = settings.clear_cache;

    //TODO: utiliser un mutex, et chaque thread va piocher dans ce mutex pour savoir quel process exécuter, quand le mutex.size() = 0, go finir le programme
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

    let mut handles = vec![];
    for _ in 0..4 {
        let processes_clone = Arc::clone(&processes);
        let handle = thread::spawn(move || process_worker(processes_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn process_worker(processes: Arc<Mutex<Vec<ProcessData>>>) {
    //TODO: more parameters
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
        process(process_data, agent.clone());
    }
}

fn process(process: ProcessData, agent: Agent) {
    let today = Local::now();
    let mut first_iter = true;
    'process: for year in (BINANCE_BIRTH..today.year()).rev() {
        let mut max_month = 12;
        if year == today.year() {
            max_month = today.month();
        }
        for month in (1..=max_month).rev() {
            let asset_file = AssetFile::new(&process.asset, &process.granularity, year, month, agent.clone());
            let display_name = asset_file.get_display_name();

            match download_file(&asset_file) {
                Ok(_) => {}
                Err(ScrapperError::NoOnlineData) => {
                    if first_iter {
                        println!("No data available for [{}]", &process.asset);
                    } else {
                        println!("Process of [{}] finished, no data available before {}/{} (included)", &process.asset, month, year);
                    }
                    break 'process;
                }
                Err(err) => {
                    //TODO: Pause on error, ask the user to fix the problem, then press enter to continue
                    println!("An error occured while downloading {}, details: {}", display_name, err);
                    break 'process;
                }
            }
            match extract_file(&asset_file, process.clear_cache) {
                Err(err) => {
                    println!("An error occurred while extracting {}, details: {}", display_name, err);
                    break 'process;
                }
                Ok(()) => {}
            }
            if first_iter {
                first_iter = false;
            }
        }
    }
}