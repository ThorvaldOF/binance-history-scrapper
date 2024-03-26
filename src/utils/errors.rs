use std::io;

pub enum ScrapperError {
    IOError(io::Error),
    IntegrityError(String),
    NoOnlineData(),
    OtherError(String),
}

impl From<io::Error> for ScrapperError {
    fn from(error: io::Error) -> Self {
        ScrapperError::IOError(error)
    }
}

impl From<&str> for ScrapperError {
    fn from(message: &str) -> Self {
        ScrapperError::IntegrityError(message.to_string())
    }
}

impl From<&str> for ScrapperError {
    fn from(message: &str) -> Self {
        ScrapperError::OtherError(message.to_string())
    }
}