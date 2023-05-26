use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Debug, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: Option<String>,
}
