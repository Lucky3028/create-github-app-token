use crate::errors::{Error, Result};
use surf::StatusCode;

type SurfResult = std::result::Result<surf::Response, surf::Error>;

pub(crate) trait ResponseExt {
    /// Converts from `surf` response result to `crate::Error` by the response's status code.
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
            StatusCode::NotFound => Error::ResourceNotFound,
            StatusCode::Unauthorized => Error::UnAuthorized,
            _ => Error::UnknownStatusCode(status),
        })
    }
}
