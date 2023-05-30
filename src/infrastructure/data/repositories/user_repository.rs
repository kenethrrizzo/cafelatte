use crate::core::entities::user::User as UserCore;
use crate::core::ports::user_port::IUserRepository;
use crate::infrastructure::data::models::user::User as UserModel;
use async_trait::async_trait;
use core::result::Result;

#[derive(Clone)]
pub struct UserRepository {
    pub conn: sqlx::MySqlPool,
}

impl UserRepository {
    pub fn new(conn: sqlx::MySqlPool) -> Self {
        UserRepository { conn }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn get_users(&self) -> Result<Vec<UserCore>, anyhow::Error> {
        let rows = sqlx::query_as::<_, UserModel>("SELECT id, name, surname FROM user")
            .fetch_all(&self.conn)
            .await?;

        let mut users: Vec<UserCore> = Vec::new();
        for row in &rows {
            users.push(UserCore::from(row.clone()));
        }

        Ok(users)
    }

    async fn get_user_by_id(&self, id: u8) -> Result<UserCore, anyhow::Error> {
        let row = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(&self.conn)
            .await?;

        Ok(UserCore::from(row))
    }

    async fn create_user(&self, _user: UserCore) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}
