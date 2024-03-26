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

    if metadata(&file_path).is_ok() {
        remove_file(&file_path)?;
    }
    if metadata(file_path.clone() + ".CHECKSUM").is_ok() {
        remove_file(file_path.clone() + ".CHECKSUM")?;
    }

    download(&asset_file, ".zip")?;
    download(&asset_file, ".zip.CHECKSUM")?;

    check_zip_integrity(&file_path)?;
    Ok(())
}

fn download(asset_file: &AssetFile, extension: &str) -> Result<(), ScrapperError> {
    let response = ureq::get(&asset_file.get_download_url(extension)).call();

    if !response.is_ok() {
        return Err(ScrapperError::NoOnlineData);
    }

    create_dir_all(&asset_file.get_download_directory())?;

    let mut file = File::create(format!("{}{}", asset_file.get_download_directory(), asset_file.get_full_file_name(extension)))?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;
    Ok(())
}
