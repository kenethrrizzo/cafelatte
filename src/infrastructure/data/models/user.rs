use crate::core::entities::user::User as UserCore;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Debug, FromRow, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub surname: String,
    pub phone_number: Option<String>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn from_user_core(user_core: UserCore) -> Self {
        User {
            id: user_core.id,
            name: user_core.name,
            surname: user_core.surname,
            phone_number: user_core.phone_number,
            email: user_core.email,
            password: user_core.password,
        }
    }
}
