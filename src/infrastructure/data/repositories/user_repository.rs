use std::error::Error;

use crate::core::entities::user::User;
use crate::core::ports::user_port::Repository;
use crate::infrastructure::data::models::user::User as UserModel;
use async_trait::async_trait;
use sqlx::MySqlPool;

pub struct UserRepository {
    pub conn: MySqlPool,
}

#[async_trait]
impl Repository for UserRepository {
    async fn get(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = sqlx::query_as::<_, UserModel>("SELECT id, name, surname FROM user")
            .fetch_all(&self.conn)
            .await?;

        let mut users: Vec<User> = Vec::new();
        for row in &rows {
            users.push(User {
                id: row.id,
                name: row.clone().name,
                surname: row.clone().surname.unwrap(),
            });
        }

        Ok(users)
    }
}
