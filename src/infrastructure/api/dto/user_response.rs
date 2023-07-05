use crate::core::entities::{login::Login, user::User as UserCore};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub complete_name: String,
}

impl UserResponse {
    pub fn from_user_core(user: UserCore) -> Self {
        let mut complete_name = user.name;
        complete_name.push_str(" ");
        complete_name.push_str(user.surname.as_str());

        UserResponse {
            id: user.id.unwrap_or_default(),
            complete_name,
        }
    }

    pub fn from_user_core_vec(users: Vec<UserCore>) -> Vec<Self> {
        let mut response: Vec<UserResponse> = vec![];
        for user in users {
            response.push(UserResponse::from_user_core(user));
        }

        response
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub complete_name: String,
    pub token: String,
}

impl LoginResponse {
    pub fn from_login(login: Login) -> Self {
        let user = login.get_user();

        let mut complete_name = user.name;
        complete_name.push_str(" ");
        complete_name.push_str(user.surname.as_str());

        LoginResponse {
            id: user.id.unwrap_or_default(),
            complete_name,
            token: login.get_token(),
        }
    }
}
