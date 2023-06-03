use crate::core::{entities::user::User, ports::user_port::IUserService};
use anyhow::{anyhow, Ok};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserServiceStub {
    pub success: bool,
}

#[async_trait]
impl IUserService for UserServiceStub {
    async fn get_users(&self) -> Result<Vec<User>, anyhow::Error> {
        if self.success {
            Ok(vec![User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            }])
        } else {
            Err(anyhow!("Error en get_users"))
        }
    }

    async fn get_user_by_id(&self, _id: u8) -> Result<User, anyhow::Error> {
        if self.success {
            Ok(User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            })
        } else {
            Err(anyhow!("Error en get_user_by_id"))
        }
    }

    async fn create_user(&self, _user: User) -> Result<(), anyhow::Error> {
        if self.success {
            Ok(())
        } else {
            Err(anyhow!("Error en create_user"))
        }
    }

    async fn update_user(&self, _user_id: i32, _user: User) -> Result<(), anyhow::Error> {
        if self.success {
            Ok(())
        } else {
            Err(anyhow!("Error en update_user"))
        }
    }

    async fn delete_user(&self, _user_id: i32) -> Result<(), anyhow::Error> {
        if self.success {
            Ok(())
        } else {
            Err(anyhow!("Error en delete_user"))
        }
    }
}
