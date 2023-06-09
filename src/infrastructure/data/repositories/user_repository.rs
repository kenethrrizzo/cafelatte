use crate::core::entities::user::User as UserCore;
use crate::core::ports::user_port::IUserRepository;
use crate::infrastructure::data::models::user::User as UserModel;
use async_trait::async_trait;
use core::result::Result;
use std::error::Error;

#[derive(Clone)]
pub struct UserRepository {
    pub conn: sqlx::MySqlPool,
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn get_users(&self) -> Result<Vec<UserCore>, Box<dyn Error>> {
        let rows = sqlx::query_as::<_, UserModel>("SELECT * FROM user")
            .fetch_all(&self.conn)
            .await?;

        let mut users: Vec<UserCore> = Vec::new();
        for row in &rows {
            users.push(UserCore::from(row.clone()));
        }

        Ok(users)
    }

    async fn get_user_by_id(&self, id: u8) -> Result<UserCore, Box<dyn Error>> {
        let row = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(&self.conn)
            .await?;

        Ok(UserCore::from(row))
    }

    async fn create_user(&self, user: UserCore) -> Result<(), Box<dyn Error>> {
        let user_model = UserModel::from(user);

        sqlx::query("INSERT INTO user (name, surname) VALUES (?, ?)")
            .bind(&user_model.name)
            .bind(&user_model.surname)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    async fn update_user(&self, user_id: i32, user: UserCore) -> Result<(), Box<dyn Error>> {
        let user_model = UserModel::from(user);

        sqlx::query("UPDATE user SET name=?, surname=? WHERE id=?")
            .bind(&user_model.name)
            .bind(&user_model.surname.unwrap_or_default())
            .bind(user_id)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM user WHERE id=?")
            .bind(user_id)
            .execute(&self.conn)
            .await?;

        Ok(())
    }
}

impl UserRepository {
    pub fn new(conn: sqlx::MySqlPool) -> Self {
        UserRepository { conn }
    }
}
