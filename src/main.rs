mod input;
mod download;
mod extract;
mod asset_file;

use std::{fs, io};
use chrono::Datelike;
use chrono::prelude::Local;
use crate::download::{download_file, DOWNLOADS_PATH};
use crate::extract::{extract_file};

const BINANCE_BIRTH: i32 = 2017;
const LOCAL_PATH: &str = "./binance_data/";

const STABLE_COIN: &str = "USDT";

fn main() {
    let settings = input::process_input();

    for asset in settings.assets {
        process(&asset, &settings.granularity, settings.clear_cache);
    }
    if settings.clear_cache {
        fs::remove_dir_all(format!("{}{}", LOCAL_PATH, DOWNLOADS_PATH)).expect("Couldn't clear downloads directory");
    }
    println!("Scrapping completed, you can find your output in 'results' directory");
    println!("Press enter to quit...");
    let mut useless_input = String::new();
    io::stdin().read_line(&mut useless_input).expect("Couldn't retrieve your input, please try again");
}

fn process(asset: &str, granularity: &str, clear_cache: bool) {
    let today = Local::now();
    'process: for year in (BINANCE_BIRTH..today.year()).rev() {
        let mut max_month = 12;
        if year == today.year() {
            max_month = today.month();
        }
        for month in (1..=max_month).rev() {
            let asset_file = asset_file::AssetFile::new(asset, granularity, year, month);
            let display_name = asset_file.get_display_name();
            println!("Processing {} ", display_name);
            let file_name = asset_file.get_file_name();

            match download_file(&asset_file) {
                Ok(false) => {
                    println!("Download of [{}] finished, no data available before {}/{} (included)", asset, month, year);
                    break 'process;
                }
                Err(err) => {
                    println!("An error occured while downloading {}, details: {}", display_name, err);
                    break 'process;
                }
                Ok(true) => {}
            }
            match extract_file(&asset_file, clear_cache) {
                Err(err) => {
                    println!("An error occurred while extracting {}, details: {}", display_name, err);
                    break 'process;
                }
                _ => {}
            }
        }
    }
}