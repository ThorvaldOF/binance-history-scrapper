use std::fs::{File, metadata, read_to_string};
use std::io::Read;
use csv::ReaderBuilder;
use sha2::{Digest, Sha256};
use crate::ScrapperError;

pub fn check_zip_integrity(file_path: &str) -> Result<(), ScrapperError> {
    let checksum_path = format!("{}{}", file_path, ".CHECKSUM");
    if metadata(file_path).is_err() || metadata(&checksum_path).is_err() {
        return Err(ScrapperError::IntegrityError);
    }

    let checksum_read = read_to_string(&checksum_path)?;
    let checksum_content: Vec<&str> = checksum_read.split_whitespace().collect();
    let expected_checksum = {
        if let Some(expected_checksum) = checksum_content.get(0).cloned() {
            expected_checksum.to_string()
        } else {
            return Err(ScrapperError::IntegrityError);
        }
    };

    let actual_checksum = calculate_checksum(file_path)?;

    return if expected_checksum == actual_checksum {
        Ok(())
    } else {
        Err(ScrapperError::IntegrityError)
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


pub fn get_minutes_in_month(month: u32, year: u32) -> Option<usize> {
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            if leap_year {
                29
            } else {
                28
            }
        }
        _ => return None,
    };
    Some(days * 24 * 60)
}