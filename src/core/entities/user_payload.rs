use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPayload {
    id: i32,
    name: String,
    surname: String,
    issued_at: i64,
    exp: i64,
}

impl UserPayload {
    pub fn new(id: i32, name: String, surname: String, duration_minutes: i64) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::minutes(duration_minutes);

        Self {
            id,
            name,
            surname,
            issued_at: now.timestamp(),
            exp: expiration.timestamp(),
        }
    }
}
