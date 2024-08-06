mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::{fs, thread};
use std::sync::{Arc, Mutex};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ureq::{Agent, AgentBuilder};
use crate::utils::asset_file::AssetFile;
use crate::download::{download_asset};
use crate::extract::{extract_asset};
use crate::input::Settings;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::{Manifest, TimePeriod};
use crate::utils::process_data::ProcessData;

const BINANCE_BIRTH: i32 = 2017;

//TODO: check all the project and rename stuff
fn main() {
    let settings = input::process_input();
    let clear_cache = settings.clear_cache;
    handle_processes(settings);

    if clear_cache {
        fs::remove_dir_all(AssetFile::get_cache_directory()).expect("Couldn't clear downloads directory");
    }
    println!("Scrapping completed, you can find your output in 'results' directory");
}

//TODO: check to either keep all bars for display, or simplify and delete them when done
fn handle_processes(settings: Settings) {
    let multi_progress = MultiProgress::new();

    let mut processes_vec: Vec<ProcessData> = vec![];
    for asset in settings.assets {
        let process_data = ProcessData::new(&settings.granularity, &asset, settings.clear_cache, multi_progress.clone());
        processes_vec.push(process_data);
    }
    let processes_size = processes_vec.len();
    let processes = Arc::new(Mutex::new(processes_vec));
    let manifest = Arc::new(Mutex::new(Manifest::new(&settings.granularity.clone())));
    let master_bar = Arc::new(Mutex::new(multi_progress.add(ProgressBar::new(processes_size as u64))));


    master_bar.lock().unwrap().set_style(ProgressStyle::with_template(
        "[TOTAL] {bar:75.white/white} {pos:>4}/{len:7}",
    )
        .unwrap()
        .progress_chars("█░"));

    let mut handles = vec![];
    for _ in 0..4 {
        let processes_clone = Arc::clone(&processes);
        let manifest_clone = Arc::clone(&manifest);
        let master_bar_clone = Arc::clone(&master_bar);
        let handle = thread::spawn(move || process_worker(processes_clone, manifest_clone, master_bar_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    match manifest.lock() {
        Ok(man) => {
            if let Err(err) = man.save() {
                //TODO: maybe a better error handling
                println!("Couldn't save manifest, cause: {}", err);
            }
        }
        Err(err) => {
            //TODO: maybe a better error handling
            println!("Couldn't save manifest, cause: {}", err);
        }
    };
}

fn process_worker(processes: Arc<Mutex<Vec<ProcessData>>>, manifest: Arc<Mutex<Manifest>>, master_bar: Arc<Mutex<ProgressBar>>) {
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
        let results = process(process_data.clone(), agent.clone());
        if let Some((down_times, date_period)) = results {
            let mut manifest = match manifest.lock() {
                Ok(man) => man,
                Err(_) => {
                    //TODO: maybe a better error handling
                    continue;
                }
            };
            manifest.add_asset(&process_data.asset, date_period);
            for down_time in down_times {
                manifest.add_down_time(down_time);
            }
            drop(manifest);
        }
        if let Ok(master_bar) = master_bar.lock() {
            master_bar.inc(1);
            drop(master_bar);
        }
    }
}

fn process(mut process: ProcessData, agent: Agent) -> Option<(Vec<TimePeriod>, TimePeriod)> {
    process.init_progress_bar();

    if let Some(start_time) = download_asset(&mut process, agent) {
        return extract_asset(&mut process, start_time);
    }
    None
}
