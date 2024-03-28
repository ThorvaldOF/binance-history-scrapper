use std::{fmt, io};

pub enum ScrapperError {
    IOError(io::Error),
    ZipError(zip::result::ZipError),
    CsvError(csv::Error),
    NetworkError(ureq::Error),
    IntegrityError,
    NoOnlineData,
    OtherError,
}

impl fmt::Display for ScrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapperError::IOError(e) => { write!(f, "TODO:Implement display") }
            ScrapperError::ZipError(e) => { write!(f, "TODO:Implement display") }
            ScrapperError::CsvError(e) => { write!(f, "TODO:Implement display") }
            ScrapperError::NetworkError(e) => { write!(f, "TODO:Implement display") }
            ScrapperError::IntegrityError => { write!(f, "TODO:Implement display") }
            ScrapperError::NoOnlineData => { write!(f, "TODO:Implement display") }
            ScrapperError::OtherError => { write!(f, "TODO:Implement display") }
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