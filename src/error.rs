use thiserror::Error;

/// Represents all possible errors in this library
#[derive(Error, Debug)]
pub enum Error {
    /// Represents an error in the underlying HTTP request
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// Represents an error in parsing a URL
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Represents an error caused by an unknown git provider in a url
    #[error("Unknown Git provider")]
    UnknownGitProvider,

    /// Represents an error caused when a url is missing its line numbers
    #[error("Missing line numbers in url")]
    MissingLineNumbers,

    /// Represents an error in parsing an integer
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
