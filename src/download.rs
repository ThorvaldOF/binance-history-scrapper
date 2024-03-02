use crate::input::Settings;
use std::fs::{File, create_dir_all, metadata};
use std::io::copy;
use crate::LOCAL_PATH;

pub const DOWNLOADS_PATH: &str = "downloads/";

//TODO: Optimize, refactor and better error management
pub fn download_file(settings: &Settings, file_name: &str) -> Result<bool, std::io::Error> {
    let file_directory = format!("{}{}{}/{}/", LOCAL_PATH, DOWNLOADS_PATH, settings.symbol, settings.granularity);
    if metadata(format!("{}{}.zip", file_directory, file_name)).is_ok() {
        return Ok(true);
    }
    let url = format!("https://data.binance.vision/data/spot/monthly/klines/{}/{}/{}.zip", settings.symbol, settings.granularity, file_name);

    create_dir_all(file_directory.clone())?;

    let response = ureq::get(&url).call();

    if !response.is_ok() {
        return Ok(false);
    }

    let mut file = File::create(format!("{}{}.zip", file_directory, file_name))?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;

    Ok(true)
}
