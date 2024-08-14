mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::{thread};
use std::sync::{Arc, Mutex};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ureq::{Agent, AgentBuilder};
use crate::download::{download_asset};
use crate::extract::{extract_asset};
use crate::input::Settings;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::{Manifest, TimePeriod};
use crate::utils::process_data::ProcessData;

const BINANCE_BIRTH: i32 = 2017;

//TODO: Modulariser un peu le bordel
//TODO: utiliser des channels
//TODO: Pourquoi ça pète avant la fin ?
struct FailedProcess {
    asset: String,
    error: ScrapperError,
}

//TODO: check all the project and rename stuff
fn main() {
    let settings = input::process_input();
    handle_processes(settings);
    println!("Scrapping completed, you can find your output in 'results' directory");
}

fn handle_processes(settings: Settings) {
    let multi_progress = MultiProgress::new();

    let mut processes_vec: Vec<ProcessData> = vec![];
    for asset in settings.assets {
        let process_data = ProcessData::new(&settings.granularity, &asset, multi_progress.clone());
        processes_vec.push(process_data);
    }
    let mut processes_size = processes_vec.len();
    let processes = Arc::new(Mutex::new(processes_vec));
    let manifest = Arc::new(Mutex::new(Manifest::new(&settings.granularity.clone())));
    let master_bar = Arc::new(Mutex::new(multi_progress.add(ProgressBar::new(processes_size as u64))));
    let failed_processes = Arc::new(Mutex::new(vec![]));

    master_bar.lock().unwrap().set_style(ProgressStyle::with_template(
        "[TOTAL] {bar:75.white/white} {pos:>4}/{len:7}",
    )
        .unwrap()
        .progress_chars("█░"));

    if processes_size < 4 {
        processes_size = 1;
    }

    let mut handles = vec![];
    for _ in 0..processes_size {
        let processes_clone = Arc::clone(&processes);
        let manifest_clone = Arc::clone(&manifest);
        let master_bar_clone = Arc::clone(&master_bar);
        let failed_processes_clone = Arc::clone(&failed_processes);
        let handle = thread::spawn(move || process_worker(processes_clone, manifest_clone, master_bar_clone, failed_processes_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let fails = failed_processes.lock().unwrap();
    if !fails.is_empty() {
        println!("{} assets failed, here's the list:", fails.len());
        for fail in fails.iter() {
            println!("[{}] => {}", fail.asset, fail.error);
        }
        println!("Fix the problems then restart the program");
    }
    drop(fails);
    let manifest = manifest.lock().unwrap();
    manifest.save().expect("Couldn't save manifest");
    drop(manifest);
}

fn process_worker(processes: Arc<Mutex<Vec<ProcessData>>>, manifest: Arc<Mutex<Manifest>>, master_bar: Arc<Mutex<ProgressBar>>, failed_processes_res: Arc<Mutex<Vec<FailedProcess>>>) {
    let agent: Agent = AgentBuilder::new()
        .build();
    let mut failed_processes: Vec<FailedProcess> = vec![];
    loop {
        let mut processes = processes.lock().unwrap();
        if processes.is_empty() {
            break;
        }

        let process_data = processes.remove(0);
        drop(processes);
        match process(process_data.clone(), agent.clone()) {
            Err(err) => {
                failed_processes.push(FailedProcess { asset: process_data.get_asset(), error: err });
                continue;
            }
            Ok(res) => {
                match res {
                    None => continue,
                    Some((down_times, date_period)) => {
                        let mut manifest = manifest.lock().unwrap();
                        manifest.add_asset(&process_data.get_asset(), date_period);
                        for down_time in down_times {
                            manifest.add_down_time(down_time);
                        }
                        drop(manifest);
                    }
                }
            }
        }
        master_bar.lock().unwrap().inc(1);
    }
    failed_processes_res.lock().unwrap().extend(failed_processes);
}

fn process(mut process: ProcessData, agent: Agent) -> Result<Option<(Vec<TimePeriod>, TimePeriod)>, ScrapperError> {
    process.init_progress_bar();

    let result = (|| {
        if let Some(start_time) = download_asset(&mut process, agent)? {
            let extraction_results = extract_asset(&mut process, start_time)?;
            return Ok(Some(extraction_results));
        }
        Err(ScrapperError::NoOnlineData)
    })();
    process.finish_progress_bar();

    result
}