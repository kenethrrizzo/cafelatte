use crate::core::entities::user::User;
use async_trait::async_trait;
use core::result::Result;
use std::error::Error;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, Box<dyn Error>>;
    async fn create_user(&self, user: User) -> Result<(), Box<dyn Error>>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), Box<dyn Error>>;
    async fn delete_user(&self, user_id: i32) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, Box<dyn Error>>;
    async fn create_user(&self, user: User) -> Result<(), Box<dyn Error>>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), Box<dyn Error>>;
    async fn delete_user(&self, user_id: i32) -> Result<(), Box<dyn Error>>;
}
