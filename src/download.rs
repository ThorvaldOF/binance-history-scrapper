use std::fs::{File, create_dir_all, metadata, remove_file};
use std::io::{copy};
use crate::utils::integrity::check_zip_integrity;
use crate::utils::asset_file::{AssetFile};
use crate::utils::errors::ScrapperError;

pub fn download_file(asset_file: &AssetFile) -> Result<(), ScrapperError> {
    let file_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");

    if check_zip_integrity(&file_path).is_ok() {
        return Ok(());
    }
    download(&asset_file, ".zip", false)?;

    download(&asset_file, ".zip.CHECKSUM", false)?;
    if check_zip_integrity(&file_path).is_err() {
        download(&asset_file, ".zip", true)?;

        download(&asset_file, ".zip.CHECKSUM", true)?;
    }
    check_zip_integrity(&file_path)?;
    Ok(())
}

fn download(asset_file: &AssetFile, extension: &str, overwrite: bool) -> Result<(), ScrapperError> {
    let file_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(extension);
    if check_file(&file_path) && overwrite {
        remove_file(&file_path)?;
    }
    let response = asset_file.agent.get(&asset_file.get_download_url(extension)).call();

    //TODO: improve that error management
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
