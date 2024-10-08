use std::fs::{File, metadata, read_to_string};
use std::io::Read;
use sha2::{Digest, Sha256};
use crate::ScrapperError;

pub fn check_zip_integrity(file_path: &str) -> Result<(), ScrapperError> {
    let checksum_path = format!("{}{}", file_path, ".CHECKSUM");
    if metadata(file_path).is_err() || metadata(&checksum_path).is_err() {
        return Err(ScrapperError::IntegrityError("TODOA:".to_string()));
    }

    let checksum_read = read_to_string(&checksum_path)?;
    let checksum_content: Vec<&str> = checksum_read.split_whitespace().collect();
    let expected_checksum = {
        if let Some(expected_checksum) = checksum_content.get(0).cloned() {
            expected_checksum.to_string()
        } else {
            return Err(ScrapperError::IntegrityError("TODOB:".to_string()));
        }
    };

    let actual_checksum = calculate_checksum(file_path)?;

    return if expected_checksum == actual_checksum {
        Ok(())
    } else {
        Err(ScrapperError::IntegrityError("TODOC:".to_string()))
    };
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

