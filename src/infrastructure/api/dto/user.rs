use crate::core::entities::user::User as UserCore;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    name: String,
    surname: String,
}

impl UserResponse {
    pub fn from(user_core: UserCore) -> Self {
        UserResponse {
            id: user_core.id.unwrap_or_default(),
            name: user_core.name,
            surname: user_core.surname,
        }
    }
}

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
    surname: String,
}

impl UserRequest {
    pub fn to_user_core(&self) -> UserCore {
        UserCore {
            id: None,
            name: self.name.clone(),
            surname: self.surname.clone(),
        }
    }
}
