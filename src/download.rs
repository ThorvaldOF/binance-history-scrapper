use std::fs::{File, create_dir_all, metadata, read_to_string, remove_file};
use std::io::{BufReader, copy, Read};
use crate::{LOCAL_PATH, STABLE_COIN};
use sha2::{Sha256, Digest};

pub const DOWNLOADS_PATH: &str = "downloads/";

pub fn download_file(asset: &str, granularity: &str, file_name: &str) -> Result<bool, std::io::Error> {
    let file_directory = format!("{}{}{}{}/{}/", LOCAL_PATH, DOWNLOADS_PATH, asset, STABLE_COIN, granularity);
    let file_full_name = format!("{}.zip", file_name);
    let file_path = &format!("{}{}", &file_directory, &file_full_name);
    //TODO: all formatting in a struct and implementation
    if check_integrity(&file_path).is_ok() {
        return Ok(true);
    }
    if metadata(&file_path).is_ok() {
        remove_file(&file_path)?;
    }
    if metadata(&format!("{}.CHECKSUM", file_path)).is_ok() {
        remove_file(&format!("{}.CHECKSUM", file_path))?;
    }

    let url = format!("https://data.binance.vision/data/spot/monthly/klines/{}{}/{}/{}", asset, STABLE_COIN, granularity, &file_full_name);
    download(&file_directory, &file_full_name, &url)?;
    download(&file_directory, &format!("{}{}", &file_full_name, ".CHECKSUM"), &format!("{}{}", url, ".CHECKSUM"))?;

    Ok(true)
}

fn download(file_directory: &str, file_name: &str, url: &str) -> Result<bool, std::io::Error> {
    let response = ureq::get(&url).call();

    if !response.is_ok() {
        return Ok(false);
    }

    create_dir_all(file_directory.clone())?;

    let mut file = File::create(format!("{}{}", file_directory, file_name))?;

    copy(&mut response.unwrap().into_reader(), &mut file)?;
    Ok(true)
}

fn check_integrity(file_path: &str) -> Result<bool, std::io::Error> {
    let checksum_path = format!("{}{}", file_path, ".CHECKSUM");
    let file_metadata = metadata(file_path)?;
    let checksum_metadata = metadata(&checksum_path)?;

    if !file_metadata.is_file() || !checksum_metadata.is_file() {
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