mod input;
mod download;
mod extract;
mod utils;
mod tests;

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ureq::{Agent};
use crate::download::{download_asset};
use crate::extract::{extract_asset};
use crate::input::Settings;
use crate::utils::errors::ScrapperError;
use crate::utils::manifest::{Manifest, TimePeriod};
use crate::utils::process_data::ProcessData;
use tokio::task;
use tokio::sync::Semaphore;

const BINANCE_BIRTH: i32 = 2017;

#[tokio::main]
async fn main() {
    let settings = input::process_input();
    handle_processes(settings).await;
    println!("Scrapping completed, you can find your output in 'results' directory");
}

async fn handle_processes(settings: Settings) {
    let multi_progress = MultiProgress::new();
    let semaphore = Arc::new(Semaphore::new(4));

    let mut processes_vec: Vec<ProcessData> = vec![];
    for asset in &settings.assets {
        let process_data = ProcessData::new(&settings.granularity, &asset);
        processes_vec.push(process_data);
    }
    let master_bar = Arc::new(Mutex::new(multi_progress.add(ProgressBar::new(processes_vec.len() as u64))));

    master_bar.lock().unwrap().set_style(ProgressStyle::with_template(
        "[TOTAL] {bar:75.white/white} {pos:>4}/{len:7}",
    )
        .unwrap()
        .progress_chars("█░"));


    let agent = Agent::new();
    let (tx, rx) = mpsc::channel::<(String, Result<(Vec<TimePeriod>, TimePeriod), ScrapperError>)>();

    let mut handles = vec![];

    for process in processes_vec {
        let process_clone = process.clone();
        let master_bar_clone = Arc::clone(&master_bar);
        let multi_progress_clone = multi_progress.clone();
        let agent_clone = agent.clone();
        let tx_clone = tx.clone();
        let semaphore_clone = Arc::clone(&semaphore);

        let handle = task::spawn(async move {
            let _permit = semaphore_clone.acquire().await.unwrap();
            new_process(process_clone, agent_clone, master_bar_clone, multi_progress_clone, tx_clone).await;
        });

        handles.push(handle);
    }
    drop(tx);
    for handle in handles {
        handle.await.unwrap();
    }
    post_process(rx, settings);
}

async fn new_process(mut process_data: ProcessData, agent: Agent, master_bar: Arc<Mutex<ProgressBar>>, multi_progress: MultiProgress, tx: Sender<(String, Result<(Vec<TimePeriod>, TimePeriod), ScrapperError>)>) {
    process_data.init_progress_bar(&multi_progress);
    let res = process(&mut process_data, agent);
    process_data.finish_progress_bar(&multi_progress);
    master_bar.lock().unwrap().inc(1);
    tx.send((process_data.get_asset(), res)).unwrap();
}

fn post_process(rx: Receiver<(String, Result<(Vec<TimePeriod>, TimePeriod), ScrapperError>)>, settings: Settings) {
    let mut manifest = Manifest::new(&settings.granularity.clone());

    while let Ok(result) = rx.recv() {
        match result.1 {
            Err(err) => {
                //TODO: automatic retry on fail, if it's not a "No data error"
                println!("Asset {} failed with error: {}", result.0, err);
                continue;
            }
            Ok(res) => {
                manifest.add_asset(&result.0, res.1);
                for down_time in res.0 {
                    manifest.add_down_time(down_time);
                }
            }
        }
    }
    manifest.save().unwrap();
}

fn process(process: &mut ProcessData, agent: Agent) -> Result<(Vec<TimePeriod>, TimePeriod), ScrapperError> {
    let result = (|| {
        if let Some(start_time) = download_asset(process, agent)? {
            let extraction_results = extract_asset(process, start_time)?;
            return Ok(extraction_results);
        }
        Err(ScrapperError::NoOnlineData)
    })();

    result
}