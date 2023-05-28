use crate::core::entities::user::User;
use async_trait::async_trait;
use core::result::Result;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>, anyhow::Error>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, anyhow::Error>;
    async fn create_user(&self, user: User) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait IUserService {
    async fn get_users(&self) -> Result<Vec<User>, anyhow::Error>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, anyhow::Error>;
    async fn create_user(&self, user: User) -> Result<(), anyhow::Error>;
}
