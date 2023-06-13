use crate::core::entities::user::User as UserCore;
use serde::{Deserialize, Serialize};

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

    pub fn get_extra_fields(&self) -> Option<Vec<String>> {
        let allowed_fields = vec!["name".to_string(), "surname".to_string()];

        let mut extra_fields = vec![];

        for (field, _) in serde_json::to_value(self).ok()?.as_object()?.iter() {
            if !allowed_fields.contains(field) {
                extra_fields.push(field.to_string());
            }
        }

        if extra_fields.is_empty() {
            None
        } else {
            Some(extra_fields)
        }
    }

    pub fn dummy() -> Self {
        UserRequest {
            name: Some("Maximiliano".to_string()),
            surname: Some("Riera".to_string()),
        }
    }
}
