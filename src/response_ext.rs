use crate::errors::{new_error, Error, ErrorKind, Result};
use surf::StatusCode;

type SurfResult = std::result::Result<surf::Response, surf::Error>;

pub(crate) trait ResponseExt {
    fn convert(self) -> Result<surf::Response>;
}

impl ResponseExt for SurfResult {
    fn convert(self) -> Result<surf::Response> {
        let res = match self {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::from(e));
            }
        };
        let status = res.status();
        if status.is_success() {
            return Ok(res);
        }

        Err(match status {
            StatusCode::NotFound => new_error(ErrorKind::ResourceNotFound),
            StatusCode::Unauthorized => new_error(ErrorKind::UnAuthorized),
            _ => new_error(ErrorKind::UnknownStatusCode(status)),
        })
    }
}
