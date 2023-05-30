use crate::core::entities::user::User as UserCore;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Debug, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: Option<String>,
}

impl User {
    pub fn from(user_core: UserCore) -> Self {
        User {
            id: user_core.id.unwrap_or_default(),
            name: user_core.name,
            surname: Some(user_core.surname),
        }
    }
}
