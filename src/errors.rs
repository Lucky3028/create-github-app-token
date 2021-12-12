use std::result;
use thiserror::Error as ThisError;

/// A type alias for `Result<T, create_github_app_token::Error>`
pub type Result<T> = result::Result<T, Error>;

/// The specific type of errors
#[derive(Debug, ThisError)]
pub enum Error {
    /// failed to fetch GitHub App Iinstalltation id
    #[error("failed to fetch GitHub App Iinstalltation id")]
    InstallationIdNotFound,
    /// failed to authorize GitHub token
    #[error("failed to authorize GitHub token")]
    UnAuthorized,
    /// failed to fetch GitHub App with the id
    #[error("failed to fetch GitHub App with the id")]
    ResourceNotFound,
    /// received unexpected http status code
    #[error("received unexpected http status code: {0:?}")]
    UnknownStatusCode(surf::StatusCode),

    /// failed to read the file
    #[error("failed to read the file: {0:?}")]
    Io(#[from] std::io::Error),
    /// failed to encode by JsonWebToken
    #[error("failed to encode by JsonWebToken: {0:?}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    /// failed to parse url from str
    #[error("failed to parse url from str: {0:?}")]
    UrlParse(#[from] surf::http::url::ParseError),
    /// failed to communicate with GitHub
    #[error("failed to communicate with GitHub: {0:?}")]
    Http(surf::Error),
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        Error::Http(err)
    }
}
