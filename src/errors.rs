use std::result;

pub type Result<T> = result::Result<T, Error>;

pub(crate) fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

#[derive(Debug)]
pub enum ErrorKind {
    InstallationIdNotFound,
    UnAuthorized,
    ResourceNotFound,

    UnknownStatusCode(surf::StatusCode),
    Io(std::io::Error),
    Jwt(jsonwebtoken::errors::Error),
    UrlParse(surf::http::url::ParseError),
    Http(surf::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        new_error(ErrorKind::Io(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        new_error(ErrorKind::Jwt(err))
    }
}

impl From<surf::http::url::ParseError> for Error {
    fn from(err: surf::http::url::ParseError) -> Self {
        new_error(ErrorKind::UrlParse(err))
    }
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        new_error(ErrorKind::Http(err))
    }
}
