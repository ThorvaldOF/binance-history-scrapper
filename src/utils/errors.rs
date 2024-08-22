use std::{fmt, io};
use std::num::{ParseFloatError, ParseIntError};
use bincode::ErrorKind;

pub enum ScrapperError {
    IOError(io::Error),
    ZipError(zip::result::ZipError),
    CsvError(csv::Error),
    NetworkError(ureq::Error),
    ParseError(String),
    IntegrityError(String),
    NoOnlineData,
}

impl fmt::Display for ScrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapperError::IOError(e) => { write!(f, "IO error: {}", e) }
            ScrapperError::ZipError(e) => { write!(f, "Zip error {}", e) }
            ScrapperError::CsvError(e) => { write!(f, "Csv error: {}", e) }
            ScrapperError::NetworkError(e) => { write!(f, "Network error: {}", e) }
            ScrapperError::ParseError(msg) => { write!(f, "Parse error: {}", msg) }
            ScrapperError::IntegrityError(msg) => { write!(f, "Integrity error: {}", msg) }
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

impl From<ParseIntError> for ScrapperError {
    fn from(error: ParseIntError) -> Self {
        ScrapperError::ParseError(error.to_string())
    }
}

impl From<ParseFloatError> for ScrapperError {
    fn from(error: ParseFloatError) -> Self {
        ScrapperError::ParseError(error.to_string())
    }
}

impl From<bincode::Error> for ScrapperError {
    fn from(error: bincode::Error) -> Self {
        ScrapperError::ParseError(error.to_string())
    }
}

