use std::fs::{File, create_dir_all, metadata, read_to_string, remove_file};
use std::io::{copy, Read};
use sha2::{Sha256, Digest};
use crate::asset_file::{AssetFile};

pub fn download_file(asset_file: &AssetFile) -> Result<bool, std::io::Error> {
    let file_path = asset_file.get_download_directory() + &asset_file.get_full_file_name(".zip");

    match check_integrity(&file_path) {
        Ok(true) => return Ok(true),
        _ => {}
    }
    if metadata(&file_path).is_ok() {
        remove_file(&file_path)?;
    }
    if metadata(file_path.clone() + ".CHECKSUM").is_ok() {
        remove_file(file_path.clone() + ".CHECKSUM")?;
    }

    download(&asset_file, ".zip")?;
    download(&asset_file, ".zip.CHECKSUM")?;

    check_integrity(&file_path)
}

fn download(asset_file: &AssetFile, extension: &str) -> Result<bool, std::io::Error> {
    let response = ureq::get(&asset_file.get_download_url(extension)).call();

    if !response.is_ok() {
        return Ok(false);
    }

    create_dir_all(&asset_file.get_download_directory())?;

    let mut file = File::create(format!("{}{}", asset_file.get_download_directory(), asset_file.get_full_file_name(extension)))?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;
    Ok(true)
}

fn check_integrity(file_path: &str) -> Result<bool, std::io::Error> {
    let checksum_path = format!("{}{}", file_path, ".CHECKSUM");
    if metadata(file_path).is_err() || metadata(&checksum_path).is_err() {
        return Ok(false);
    }

    let expected_checksum = match read_to_string(&checksum_path) {
        Ok(content) => {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if let Some(expected_checksum) = parts.get(0).cloned() {
                Some(expected_checksum.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    };

    if let Some(expected_checksum) = expected_checksum {
        let actual_checksum = calculate_checksum(file_path)?;

        return if expected_checksum == actual_checksum {
            Ok(true)
        } else {
            Ok(false)
        };
    }
    Ok(false)
}

fn calculate_checksum(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    let checksum = format!("{:x}", hash_result);
    Ok(checksum)
}