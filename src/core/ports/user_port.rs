use crate::core::{
    entities::{login::Login, user::User},
    errors::user_errors::UserError,
};
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn insert_user(&self, user: User) -> Result<User, UserError>;
    async fn get_user_by_email(&self, email: String) -> Result<User, UserError>;
    async fn get_users(&self) -> Result<Vec<User>, UserError>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, UserError>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), UserError>;
    async fn delete_user(&self, user_id: i32) -> Result<(), UserError>;
}

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, user: User) -> Result<Login, UserError>;
    async fn login(&self, email: String, password: String) -> Result<Login, UserError>;

    async fn get_users(&self) -> Result<Vec<User>, UserError>;
    async fn get_user_by_id(&self, id: u8) -> Result<User, UserError>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<(), UserError>;
    async fn delete_user(&self, user_id: i32) -> Result<(), UserError>;
}
