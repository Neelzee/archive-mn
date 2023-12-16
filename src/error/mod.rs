use std::fmt::Display;
use scraper::error::SelectorErrorKind;

#[derive(Debug, Clone)]
pub enum ArchiveError {
    ScraperError(String),
    XlError(String),
    XlSaveError(String, String),
    ParserError(String),
    UrlParseError(String),
    MissingTitle,
    InvalidURL,
    IOError(String),
    RequestError(String),
    ResponseError(String),
}

unsafe impl Sync for ArchiveError {
    
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::ScraperError(err) => write!(f, "Scraper Error: {}", err),
            ArchiveError::XlError(err) => write!(f, "Xl Error: {}", err),
            ArchiveError::XlSaveError(err, path) => write!(f, "Xl Save Error: {}, path: {}", err, path),
            ArchiveError::ParserError(str) => write!(f, "Parser Error: {}", str),
            ArchiveError::UrlParseError(err) => write!(f, "Id Parse Error: {}", err),
            ArchiveError::MissingTitle => write!(f, "Missing Title Error"),
            ArchiveError::InvalidURL => write!(f, "Invalid URL Error"),
            ArchiveError::IOError(e) => write!(f, "IOError: {}", e),
            ArchiveError::RequestError(err) => write!(f, "Reqwest Error: {}", err),
            ArchiveError::ResponseError(err) => write!(f, "Response Error: {}", err),
        }
    }
}

impl From<reqwest::Error> for ArchiveError {
    fn from(value: reqwest::Error) -> Self {
        ArchiveError::ScraperError(value.to_string())
    }
}

impl From<rust_xlsxwriter::XlsxError> for ArchiveError {
    fn from(value: rust_xlsxwriter::XlsxError) -> Self {
        ArchiveError::XlError(value.to_string())
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for ArchiveError {
    fn from(value: SelectorErrorKind) -> Self {
        ArchiveError::ParserError(value.to_string())
    }
}

impl From<std::num::ParseIntError> for ArchiveError {
    fn from(value: std::num::ParseIntError) -> Self {
        ArchiveError::UrlParseError(value.to_string())
    }
}


impl From<std::io::Error> for ArchiveError {
    fn from(value: std::io::Error) -> Self {
        ArchiveError::IOError(value.to_string())
    }
}