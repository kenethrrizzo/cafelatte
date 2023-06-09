use crate::core::{entities::user::User, errors::user_errors::UserErrors, ports::user_port::IUserService};
use async_trait::async_trait;
use std::error::Error;

#[derive(Clone)]
pub struct UserServiceStub {
    pub success: bool,
}

#[async_trait]
impl IUserService for UserServiceStub {
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        if self.success {
            Ok(vec![User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            }])
        } else {
            Err(UserErrors::Unknown.into())
        }
    }

    async fn get_user_by_id(&self, _id: u8) -> Result<User, Box<dyn Error>> {
        if self.success {
            Ok(User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            })
        } else {
            Err(UserErrors::Unknown.into())
        }
    }

    async fn create_user(&self, _user: User) -> Result<(), Box<dyn Error>> {
        if self.success {
            Ok(())
        } else {
            Err(UserErrors::Unknown.into())
        }
    }

    async fn update_user(&self, _user_id: i32, _user: User) -> Result<(), Box<dyn Error>> {
        if self.success {
            Ok(())
        } else {
            Err(UserErrors::Unknown.into())
        }
    }

    async fn delete_user(&self, _user_id: i32) -> Result<(), Box<dyn Error>> {
        if self.success {
            Ok(())
        } else {
            Err(UserErrors::Unknown.into())
        }
    }
}
