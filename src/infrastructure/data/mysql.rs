use std::env;

use sqlx::MySqlPool;

pub async fn connect_to_database() -> Result<MySqlPool, sqlx::Error> {
    let conn = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;
    Ok(conn)
}
