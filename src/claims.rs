use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

/// `Claims` of `JsonWebToken`
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Claims {
    iat: i64,
    exp: i64,
    iss: usize,
}

impl Claims {
    pub(crate) fn new(issued_at: DateTime<Utc>, expired_at: DateTime<Utc>, issuer: usize) -> Self {
        // マイクロ秒を切り捨て
        let issued_at = issued_at
            .date()
            .and_hms_milli(issued_at.hour(), issued_at.minute(), issued_at.second(), 0)
            .timestamp();
        let expired_at = expired_at
            .date()
            .and_hms_milli(
                expired_at.hour(),
                expired_at.minute(),
                expired_at.second(),
                0,
            )
            .timestamp();
        Self {
            iat: issued_at,
            exp: expired_at,
            iss: issuer,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Claims;
    use chrono::{TimeZone, Utc};
    use test_case::test_case;

    const FORMATTER: &str = "%Y/%m/%d %H:%M";
    const FORMATTER_WITH_SEC: &str = "%Y/%m/%d %H:%M:%S";
    const FORMATTER_WITH_MILL_SEC: &str = "%Y/%m/%d %H:%M:%S%.3f";

    #[test_case("2021/01/01 00:00", "2021/01/01 23:59", 0, FORMATTER
        => Claims{ iat: 1609459200, exp: 1609545540, iss: 0 }; "format to date time")]
    #[test_case("2021/01/01 00:00:00", "2021/01/01 23:59:00", 0, FORMATTER_WITH_SEC
        => Claims{ iat: 1609459200, exp: 1609545540, iss: 0 }; "format to date time with seconds")]
    #[test_case("2021/01/01 00:00:00.000", "2021/01/01 23:59:00.000", 0, FORMATTER_WITH_MILL_SEC
        => Claims{ iat: 1609459200, exp: 1609545540, iss: 0 }; "format to date time with milliseconds")]
    #[test_case("2021/01/01 00:00:00.123", "2021/01/01 23:59:00.456", 0, FORMATTER_WITH_MILL_SEC
        => Claims{ iat: 1609459200, exp: 1609545540, iss: 0 }; "milliseconds are forced to be 0")]
    fn gen_claims(issued_at: &str, expired_at: &str, issuer: usize, formatter: &str) -> Claims {
        let issued_at = Utc.datetime_from_str(issued_at, formatter).unwrap();
        let expired_at = Utc.datetime_from_str(expired_at, formatter).unwrap();

        Claims::new(issued_at, expired_at, issuer)
    }
}
