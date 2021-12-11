use surf::StatusCode;

type SurfResult = std::result::Result<surf::Response, surf::Error>;

pub(crate) trait ResponseExt {
    fn convert(self) -> crate::errors::Result<surf::Response>;
}

impl ResponseExt for SurfResult {
    fn convert(self) -> crate::errors::Result<surf::Response> {
        let res = match self {
            Ok(res) => res,
            Err(e) => {
                return Err(crate::errors::Error::from(e));
            }
        };
        let status = res.status();
        if status.is_success() {
            return Ok(res);
        }

        Err(match status {
            StatusCode::NotFound => {
                crate::errors::new_error(crate::errors::ErrorKind::ResourceNotFound)
            }
            StatusCode::Unauthorized => {
                crate::errors::new_error(crate::errors::ErrorKind::UnAuthorized)
            }
            _ => crate::errors::new_error(crate::errors::ErrorKind::UnknownStatusCode(status)),
        })
    }
}
