use crate::core::ports::user_port::IUserService;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/users")]
pub async fn get_users(user_service: web::Data<Arc<dyn IUserService>>) -> impl Responder {
    let users = user_service.get_users().await.unwrap();

    for user in users {
        println!("User name: {}", user.name);
    }

    HttpResponse::Ok().body("get-users")
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("get-user-by-id")
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Created().body("create-user")
}
