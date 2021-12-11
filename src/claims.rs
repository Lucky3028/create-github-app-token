use serde::{Deserialize, Serialize};
use chrono::{DateTime, Timelike, Utc};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    exp: i64,
    iat: i64,
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
