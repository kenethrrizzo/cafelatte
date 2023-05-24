use dotenv::dotenv;
use serde::Serialize;
use std::env;

use salvo::prelude::*;
use sqlx::{FromRow, MySqlPool};

/// Pinta un string estático.
#[handler]
async fn hello() -> &'static str {
    "Hello, World!"
}

/// Recibe un request y un response, con el response se envía el mensaje.
#[handler]
async fn hello_world(_req: &mut Request, res: &mut Response) {
    res.status_code(StatusCode::OK).render("Hello, World!");
}

#[handler]
pub async fn get_all_users(res: &mut Response) {
    let mut users: Vec<User> = Vec::new();
    match get_users_from_db().await {
        Ok(u) => users = u,
        Err(e) => println!("Error retrieving users: {}", e),
    }
    res.status_code(StatusCode::OK).render(Json(users));
}

#[derive(Serialize, Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    surname: Option<String>,
}

async fn get_users_from_db() -> Result<Vec<User>, sqlx::Error> {
    dotenv().ok();
    let pool = connect_to_database().await?;

    let rows = sqlx::query_as::<_, User>("SELECT id, name, surname FROM user")
        .fetch_all(&pool)
        .await?;

    for row in &rows {
        println!(
            "ID: {}, Name: {}, Email: {}",
            row.id,
            row.name,
            row.surname.clone().unwrap()
        );
    }

    Ok(rows)
}

async fn connect_to_database() -> Result<MySqlPool, sqlx::Error> {
    Ok(MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?)
}

pub fn router() -> Router {
    Router::with_path("tests").get(get_all_users)
}
