pub enum ArchiveError<'a> {
    ScraperError(reqwest::Error),
    XlError(rust_xlsxwriter::XlsxError),
    ParserError(scraper::error::SelectorErrorKind<'a>),
    UrlParseError(std::num::ParseIntError)
}

impl From<reqwest::Error> for ArchiveError<'static> {
    fn from(value: reqwest::Error) -> Self {
        ArchiveError::ScraperError(value)
    }
}

impl From<rust_xlsxwriter::XlsxError> for ArchiveError<'static> {
    fn from(value: rust_xlsxwriter::XlsxError) -> Self {
        ArchiveError::XlError(value)
    }
}

impl From<scraper::error::SelectorErrorKind<'static>> for ArchiveError<'static> {
    fn from(value: scraper::error::SelectorErrorKind<'static>) -> Self {
        ArchiveError::ParserError(value)
    }
}

impl From<std::num::ParseIntError> for ArchiveError<'static> {
    fn from(value: std::num::ParseIntError) -> Self {
        ArchiveError::UrlParseError(value)
    }
}