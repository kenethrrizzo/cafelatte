use crate::{
    core::entities::user::User as UserCore,
    core::{errors::user_errors::UserError, ports::user_port::IUserRepository},
    infrastructure::data::models::user::User as UserModel,
};

#[derive(Clone)]
pub struct UserRepository {
    pub conn: sqlx::MySqlPool,
}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn get_users(&self) -> core::result::Result<Vec<UserCore>, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user")
            .fetch_all(&self.conn)
            .await;

        match result {
            Ok(rows) => {
                let mut users: Vec<UserCore> = Vec::new();
                for row in &rows {
                    users.push(UserCore::from(row.clone()));
                }

                Ok(users)
            }
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn get_user_by_id(&self, id: u8) -> core::result::Result<UserCore, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(&self.conn)
            .await;

        match result {
            Ok(row) => Ok(UserCore::from(row)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn create_user(&self, user: UserCore) -> core::result::Result<(), UserError> {
        let user_model = UserModel::from(user);

        let result = sqlx::query("INSERT INTO user (name, surname) VALUES (?, ?)")
            .bind(&user_model.name)
            .bind(&user_model.surname)
            .execute(&self.conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("SQLx error: {:?}", err);
                Err(UserError::Unexpected)
            }
        }
    }

    async fn update_user(&self, user_id: i32, user: UserCore) -> core::result::Result<(), UserError> {
        let user_model = UserModel::from(user);

        let result = sqlx::query("UPDATE user SET name=?, surname=? WHERE id=?")
            .bind(&user_model.name)
            .bind(&user_model.surname.unwrap_or_default())
            .bind(user_id)
            .execute(&self.conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn delete_user(&self, user_id: i32) -> core::result::Result<(), UserError> {
        let result = sqlx::query("DELETE FROM user WHERE id=?")
            .bind(user_id)
            .execute(&self.conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }
}

impl UserRepository {
    pub fn new(conn: sqlx::MySqlPool) -> Self {
        UserRepository { conn }
    }
}
