use crate::core::{
    entities::user::User,
    ports::user_port::{IUserRepository, IUserService},
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserService<R>
where
    R: IUserRepository,
{
    user_repository: R,
}

#[async_trait]
impl<R> IUserService for UserService<R>
where
    R: IUserRepository,
{
    async fn get_users(&self) -> Result<Vec<User>, anyhow::Error> {
        self.user_repository.get_users().await
    }

    async fn get_user_by_id(&self, id: u8) -> Result<User, anyhow::Error> {
        self.user_repository.get_user_by_id(id).await
    }

    async fn create_user(&self, user: User) -> Result<(), anyhow::Error> {
        self.user_repository.create_user(user).await
    }

    async fn update_user(&self, user_id: i32, user: User) -> Result<(), anyhow::Error> {
        self.user_repository.update_user(user_id, user).await
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), anyhow::Error> {
        self.user_repository.delete_user(user_id).await
    }
}

impl<R> UserService<R>
where
    R: IUserRepository,
{
    pub fn new(user_repository: R) -> Self {
        UserService { user_repository }
    }
}
