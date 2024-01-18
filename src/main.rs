mod input;
mod download;
mod extract;
mod merge;

use std::{fs, io};
use chrono::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::download::{download_file, DOWNLOADS_PATH};
use crate::extract::{extract_file, EXTRACTS_PATH};
use crate::input::Settings;
use crate::merge::merge_files;

const BINANCE_BIRTH: i32 = 2017;

//TODO: implementation of interval
fn main() {
    let settings = input::process_input();
    let today = Local::now();
    'process: for year in (BINANCE_BIRTH..today.year()).rev() {
        let mut max_month = 12;
        if year == today.year() {
            max_month = today.month();
        }
        for month in (1..=max_month).rev() {
            let mut month_prefix = "";
            if month < 10 {
                month_prefix = "0";
            }
            let pb = ProgressBar::new(1);
            let style = ProgressStyle::with_template("{prefix:.bold.dim}: {wide_msg}")
                .unwrap();
            pb.set_style(style);
            pb.set_prefix(format!("[{} {} -> {}/{}]", settings.symbol, settings.granularity, month, year));
            pb.set_message("Downloading...");
            let file_name = format!("{}-{}-{}-{}{}", settings.symbol, settings.granularity, year, month_prefix, month);

            match download_file(&settings, &file_name) {
                Ok(false) => {
                    pb.set_message(format!("Download of [{}] finished, no data available before {}/{} (included)", settings.asset, month, year));
                    pb.finish();
                    break 'process;
                }
                Err(err) => {
                    pb.set_message(format!("An error occured while downloading, details: {}", err));
                    pb.finish();
                    break 'process;
                }
                Ok(true) => {}
            }
            pb.set_message("Extracting...");
            match extract_file(&settings, &file_name) {
                Err(err) => {
                    pb.set_message(format!("An error occurred while extracting, details: {}", err));
                    pb.finish();
                    break 'process;
                }
                Ok(..) => {
                    pb.set_message("Finished");
                    pb.finish();
                }
            }
        }
    }
    println!("Finished downloads and extracts, merging result files");
    match merge_files(&settings) {
        Err(err) => {
            println!("An error occurred while merging resulting files, details: {}", err);
        }
        Ok(..) => {
            clear_cache(&settings);
            println!("Scrapping completed, you can find your output in 'results' directory");
            println!("Press enter to quit...");
            let mut useless_input = String::new();
            io::stdin().read_line(&mut useless_input).expect("Couldn't retrieve your input, please try again");
        }
    }
}

fn clear_cache(settings: &Settings) {
    if settings.clear_downloads {
        fs::remove_dir_all(DOWNLOADS_PATH).expect("Couldn't clear downloads directory");
    }
    if settings.clear_extracts {
        fs::remove_dir_all(EXTRACTS_PATH).expect("Couldn't clear extracts directory");
    }
}
