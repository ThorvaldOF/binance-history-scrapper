use std::{fmt, io};

pub enum ScrapperError {
    IOError(io::Error),
    ZipError(zip::result::ZipError),
    CsvError(csv::Error),
    NetworkError(ureq::Error),
    IntegrityError,
    NoOnlineData,
}

impl fmt::Display for ScrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapperError::IOError(e) => { write!(f, "IO error: {}", e) }
            ScrapperError::ZipError(e) => { write!(f, "Zip error {}", e) }
            ScrapperError::CsvError(e) => { write!(f, "Csv error: {}", e) }
            ScrapperError::NetworkError(e) => { write!(f, "Network error: {}", e) }
            ScrapperError::IntegrityError => { write!(f, "Integrity couldn't be checked") }
            ScrapperError::NoOnlineData => { write!(f, "No data available on Binance servers") }
        }
    }
}

impl From<io::Error> for ScrapperError {
    fn from(error: io::Error) -> Self {
        ScrapperError::IOError(error)
    }
}

impl From<zip::result::ZipError> for ScrapperError {
    fn from(error: zip::result::ZipError) -> Self {
        ScrapperError::ZipError(error)
    }
}

impl From<csv::Error> for ScrapperError {
    fn from(error: csv::Error) -> Self {
        ScrapperError::CsvError(error)
    }
}
