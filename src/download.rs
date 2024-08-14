use std::fs::{File, create_dir_all, metadata, remove_file};
use std::io::{copy};
use ureq::Agent;
use crate::BINANCE_BIRTH;
use crate::utils::integrity::check_zip_integrity;
use crate::utils::asset_file::{AssetFile};
use crate::utils::errors::ScrapperError;
use crate::utils::month_year::MonthYear;
use crate::utils::process_data::ProcessData;

pub fn download_asset(process: &mut ProcessData, agent: Agent) -> Result<Option<MonthYear>, ScrapperError> {
    let end_time = process.get_end();
    let mut start_time = end_time.clone();
    'downloads: for year in (BINANCE_BIRTH..=end_time.get_year()).rev() {
        let mut max_month = 12;
        if year == end_time.get_year() {
            max_month = end_time.get_month();
        }
        for month in (1..=max_month).rev() {
            let month_year = MonthYear::new(month, year);
            let asset_file = AssetFile::new(&process.get_asset(), &process.get_granularity(), month_year.clone());
            if let Err(err) = download_file(&asset_file, agent.clone()) {
                match err {
                    ScrapperError::NoOnlineData => {
                        break 'downloads;
                    }
                    _ => {
                        process.finish_progress_bar();
                        return Err(err);
                    }
                };
            }

            start_time = month_year;
            process.increment_progress_bar();
        }
    }
    if start_time.get_year() == end_time.get_year() && start_time.get_month() == end_time.get_month() {
        Ok(None)
    } else {
        Ok(Some(start_time))
    }
}

pub fn download_file(asset_file: &AssetFile, agent: Agent) -> Result<(), ScrapperError> {
    let file_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");

    if check_zip_integrity(&file_path).is_ok() {
        return Ok(());
    }
    download(&asset_file, ".zip", false, agent.clone())?;
    download(&asset_file, ".zip.CHECKSUM", false, agent.clone())?;

    if check_zip_integrity(&file_path).is_err() {
        download(&asset_file, ".zip", true, agent.clone())?;
        download(&asset_file, ".zip.CHECKSUM", true, agent.clone())?;
    }
    check_zip_integrity(&file_path)?;
    Ok(())
}

fn download(asset_file: &AssetFile, extension: &str, overwrite: bool, agent: Agent) -> Result<(), ScrapperError> {
    let file_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(extension);
    if check_file(&file_path) && overwrite {
        remove_file(&file_path)?;
    }
    let response = agent.get(&asset_file.get_download_url(extension)).call();

    match response {
        Ok(_) => {}
        Err(error) => {
            return match error {
                ureq::Error::Status(code, _) => {
                    if code == 404 {
                        Err(ScrapperError::NoOnlineData)
                    } else {
                        Err(ScrapperError::NetworkError(error))
                    }
                }
                ureq::Error::Transport(_) => {
                    Err(ScrapperError::NetworkError(error))
                }
            };
        }
    }

    create_dir_all(&asset_file.get_download_directory())?;

    let mut file = File::create(file_path)?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;
    Ok(())
}

fn check_file(path: &str) -> bool {
    if let Ok(metadata) = metadata(path) {
        return metadata.is_file();
    }
    false
}
