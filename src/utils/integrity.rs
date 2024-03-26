use std::fs::{File, metadata, read_to_string};
use std::io::Read;
use sha2::{Digest, Sha256};

pub fn check_integrity(file_path: &str) -> Result<bool, std::io::Error> {
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

pub fn get_minutes_in_month(month: u32, year: i32) -> Option<usize> {
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