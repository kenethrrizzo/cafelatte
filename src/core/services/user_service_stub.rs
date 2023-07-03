use crate::core::{
    entities::{login::Login, user::User},
    errors::user_errors::UserError,
    ports::user_port::IUserService,
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserServiceStub {
    pub status_code: i32,
}

#[async_trait]
impl IUserService for UserServiceStub {
    async fn register(&self, _user: User) -> Result<Login, UserError> {
        if self.status_code == 200 {
            Ok(Login::new())
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn login(&self, _email: String, _password: String) -> Result<Login, UserError> {
        if self.status_code == 200 {
            Ok(Login::new())
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn get_users(&self) -> Result<Vec<User>, UserError> {
        if self.status_code == 200 {
            Ok(vec![User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
                phone_number: Some("0988059308".to_string()),
                email: "kenethriera@gmail.com".to_string(),
                password: "password".to_string(),
            }])
        } else if self.status_code == 404 {
            Err(UserError::NotFound)
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn get_user_by_id(&self, _id: u8) -> Result<User, UserError> {
        if self.status_code == 200 {
            Ok(User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
                phone_number: Some("0988059308".to_string()),
                email: "kenethriera@gmail.com".to_string(),
                password: "password".to_string(),
            })
        } else if self.status_code == 404 {
            Err(UserError::NotFound)
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn update_user(&self, _user_id: i32, _user: User) -> Result<(), UserError> {
        if self.status_code == 200 {
            Ok(())
        } else if self.status_code == 404 {
            Err(UserError::NotFound)
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn delete_user(&self, _user_id: i32) -> Result<(), UserError> {
        if self.status_code == 200 {
            Ok(())
        } else if self.status_code == 404 {
            Err(UserError::NotFound)
        } else {
            Err(UserError::Unexpected)
        }
    }
}
