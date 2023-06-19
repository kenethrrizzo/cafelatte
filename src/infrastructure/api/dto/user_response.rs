use crate::core::entities::user::User as UserCore;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub surname: String,
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
