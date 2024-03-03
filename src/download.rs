use std::fs::{File, create_dir_all, metadata};
use std::io::copy;
use crate::{LOCAL_PATH, STABLE_COIN};

pub const DOWNLOADS_PATH: &str = "downloads/";

pub fn download_file(asset: &str, granularity: &str, file_name: &str) -> Result<bool, std::io::Error> {
    let file_directory = format!("{}{}{}{}/{}/", LOCAL_PATH, DOWNLOADS_PATH, asset, STABLE_COIN, granularity);
    if metadata(format!("{}{}.zip", file_directory, file_name)).is_ok() {
        return Ok(true);
    }
    let url = format!("https://data.binance.vision/data/spot/monthly/klines/{}{}/{}/{}.zip", asset, STABLE_COIN, granularity, file_name);

    create_dir_all(file_directory.clone())?;

    let response = ureq::get(&url).call();

    if !response.is_ok() {
        return Ok(false);
    }

    let mut file = File::create(format!("{}{}.zip", file_directory, file_name))?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;

    Ok(true)
}
