use async_trait::async_trait;

use crate::core::{
    entities::user::User,
    ports::user_port::{IUserRepository, IUserService},
};

#[derive(Clone)]
pub struct UserService<R>
where
    R: IUserRepository,
{
    user_repository: R,
}

impl<R> UserService<R>
where
    R: IUserRepository,
{
    pub fn new(user_repository: R) -> Self {
        UserService { user_repository }
    }
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
        unimplemented!()
    }
}
