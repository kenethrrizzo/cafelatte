use crate::core::entities::user::User as UserCore;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub name: String,
    pub surname: String,
    pub phone_number: Option<String>,
    pub email: String,
    pub password: String,
}

impl UserRequest {
    pub fn to_user_core(&self) -> UserCore {
        UserCore {
            id: None,
            name: self.name.clone(),
            surname: self.surname.clone(),
            phone_number: self.phone_number.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.email.contains("@")
    }

    pub fn dummy() -> Self {
        UserRequest {
            name: "Maximiliano".to_string(),
            surname: "Riera".to_string(),
            phone_number: Some("0988059308".to_string()),
            email: "maximiliano@gmail.com".to_string(),
            password: "oas98y8dn".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

impl LoginRequest {
    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.email.contains("@")
    }
}
