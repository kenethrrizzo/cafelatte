use crate::core::{entities::user::User, errors::user_errors::UserError};
use async_trait::async_trait;
use core::result::Result;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn register(&self, user: User) -> Result<(), UserError>;

    async fn get_users(&self) -> Result<Vec<User>, UserError>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, UserError>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), UserError>;
    async fn delete_user(&self, user_id: i32) -> Result<(), UserError>;
}

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, user: User) -> Result<(), UserError>;

    async fn get_users(&self) -> Result<Vec<User>, UserError>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, UserError>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), UserError>;
    async fn delete_user(&self, user_id: i32) -> Result<(), UserError>;
}
