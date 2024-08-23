mod input;
mod download;
mod extract;
mod utils;

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
use crate::utils::month_year::MonthYear;
use crate::utils::start_dates::StartDates;

const BINANCE_BIRTH: i32 = 2017;

struct ProcessResult {
    down_times: Vec<TimePeriod>,
    time_period: TimePeriod,
    start_date: Option<MonthYear>,
}

#[tokio::main]
async fn main() {
    let settings = input::process_input();
    handle_processes(settings).await;
    println!("Scrapping completed, you can find your output in 'results' directory");
}

async fn handle_processes(settings: Settings) {
    let multi_progress = MultiProgress::new();
    let semaphore = Arc::new(Semaphore::new(4));

    let start_dates = StartDates::load();

    let mut processes_vec: Vec<ProcessData> = vec![];
    for asset in &settings.assets {
        let start = start_dates.get_start_date(&asset);
        let process_data = ProcessData::new(&settings.granularity, &asset, start);
        processes_vec.push(process_data);
    }
    let master_bar = Arc::new(Mutex::new(multi_progress.add(ProgressBar::new(processes_vec.len() as u64))));

    master_bar.lock().unwrap().set_style(ProgressStyle::with_template(
        "[TOTAL] {bar:75.white/white} {pos:>4}/{len:7}",
    )
        .unwrap()
        .progress_chars("█░"));


    let agent = Agent::new();
    let (tx, rx) = mpsc::channel::<(String, Result<ProcessResult, ScrapperError>)>();

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

async fn new_process(mut process_data: ProcessData, agent: Agent, master_bar: Arc<Mutex<ProgressBar>>, multi_progress: MultiProgress, tx: Sender<(String, Result<ProcessResult, ScrapperError>)>) {
    process_data.init_progress_bar(&multi_progress);
    let res = process(&mut process_data, agent);
    process_data.finish_progress_bar(&multi_progress);
    master_bar.lock().unwrap().inc(1);
    tx.send((process_data.get_asset(), res)).unwrap();
}

fn post_process(rx: Receiver<(String, Result<ProcessResult, ScrapperError>)>, settings: Settings) {
    let mut manifest = Manifest::new(&settings.granularity.clone());
    let mut start_dates = StartDates::load();

    while let Ok(result) = rx.recv() {
        match result.1 {
            Err(err) => {
                //TODO: automatic retry on fail, if it's not a "No data error"
                println!("Asset {} failed with error: {}", result.0, err);
                continue;
            }
            Ok(res) => {
                manifest.add_asset(&result.0, res.time_period);
                if let Some(start_date) = res.start_date {
                    start_dates.set_start_date(&result.0, start_date);
                }
                for down_time in res.down_times {
                    manifest.add_down_time(down_time);
                }
            }
        }
    }
    start_dates.save();
    manifest.save().unwrap();
}

//(Vec<TimePeriod>, TimePeriod)
fn process(process: &mut ProcessData, agent: Agent) -> Result<ProcessResult, ScrapperError> {
    let result = (|| {
        if let Some(start_time) = download_asset(process, agent)? {
            let extraction_results = extract_asset(process, start_time)?;
            return Ok(extraction_results);
        }
        Err(ScrapperError::NoOnlineData)
    })();
    let extracted_result = result?;
    Ok(ProcessResult { down_times: extracted_result.0, time_period: extracted_result.1, start_date: process.get_start() })
}