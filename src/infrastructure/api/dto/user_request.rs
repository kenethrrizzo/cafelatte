use crate::core::entities::user::User as UserCore;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub name: Option<String>,
    pub surname: Option<String>,
}

impl UserRequest {
    pub fn to_user_core(&self) -> UserCore {
        UserCore {
            id: None,
            name: self.name.clone().unwrap(),
            surname: self.surname.clone().unwrap(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.name.is_some() && self.surname.is_some()
    }

    pub fn dummy() -> Self {
        UserRequest {
            name: Some("Maximiliano".to_string()),
            surname: Some("Riera".to_string()),
        }
    }
}
