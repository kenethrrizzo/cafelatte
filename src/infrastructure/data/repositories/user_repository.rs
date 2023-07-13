use crate::core::{
    entities::user::User as UserCore, errors::user_errors::UserError,
    ports::user_port::IUserRepository,
};
use crate::infrastructure::data::models::user::User as UserModel;

#[derive(Clone)]
pub struct UserRepository {
    pub conn: sqlx::MySqlPool,
}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn insert_user(&self, user: UserCore) -> Result<UserCore, UserError> {
        let mut user_model = UserModel::from_user_core(user);

        let result = sqlx::query(
            "INSERT INTO user (name, surname, phone_number, email, password) VALUES (?,?,?,?,?)",
        )
        .bind(&user_model.name)
        .bind(&user_model.surname)
        .bind(&user_model.clone().phone_number.unwrap_or_default())
        .bind(&user_model.email)
        .bind(&user_model.password)
        .execute(&self.conn)
        .await;

        match result {
            Ok(r) => {
                user_model.set_id(r.last_insert_id() as i32);

                Ok(UserCore::from_user_model(user_model))
            }
            Err(err) => {
                log::error!("{:?}", err);
                Err(UserError::Unexpected)
            }
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<UserCore, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE email=?")
            .bind(email)
            .fetch_one(&self.conn)
            .await;

        match result {
            Ok(row) => Ok(UserCore::from_user_model(row)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("{:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn get_users(&self) -> Result<Vec<UserCore>, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user")
            .fetch_all(&self.conn)
            .await;

        match result {
            Ok(rows) => Ok(UserCore::from_user_model_vec(rows)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("{:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn get_user_by_id(&self, id: u8) -> Result<UserCore, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(&self.conn)
            .await;

        match result {
            Ok(row) => Ok(UserCore::from_user_model(row)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("{:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn update_user(&self, user_id: i32, user: UserCore) -> Result<(), UserError> {
        let user_model = UserModel::from_user_core(user);

        let result =
            sqlx::query("UPDATE user SET name=?, surname=?, phone_number=?, email=? WHERE id=?")
                .bind(&user_model.name)
                .bind(&user_model.surname)
                .bind(&user_model.phone_number.unwrap_or_default())
                .bind(&user_model.email)
                .bind(user_id)
                .execute(&self.conn)
                .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("{:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), UserError> {
        let result = sqlx::query("DELETE FROM user WHERE id=?")
            .bind(user_id)
            .execute(&self.conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("{:?}", err);
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
