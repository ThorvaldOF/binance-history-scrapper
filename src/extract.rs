use std::fs;
use std::fs::File;
use std::io::copy;
use zip::ZipArchive;
use crate::download::DOWNLOADS_PATH;
use crate::input::Settings;

pub const EXTRACTS_PATH: &str = "./extracts/";

pub fn extract_file(settings: &Settings, file_name: &str) -> Result<bool, std::io::Error> {
    let file_directory = format!("{}{}/", EXTRACTS_PATH, settings.symbol);
    if fs::metadata(format!("{}{}.csv", file_directory, file_name)).is_ok() {
        return Ok(true);
    }
    let source_path = format!("{}{}/{}.zip", DOWNLOADS_PATH, settings.symbol, file_name);
    let source_file = File::open(source_path.clone())?;

    let mut archive = ZipArchive::new(source_file)?;

    if archive.len() != 1 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid ZIP file structure"));
    }
    fs::create_dir_all(file_directory.clone())?;

    let mut entry = archive.by_index(0)?;
    let target_file = format!("{}{}.csv", file_directory, file_name);
    let mut dest_file = File::create(&target_file)?;

    copy(&mut entry, &mut dest_file)?;
    if settings.clear_downloads {
        fs::remove_file(source_path)?;
    }
    Ok(true)
}
