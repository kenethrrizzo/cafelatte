use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use salvo_skeleton::{
    core::{ports::user_port::IUserService, services::user_service::UserService},
    infrastructure::{
        api::controllers::user_controller::{create_user, get_user_by_id, get_users},
        data::{mysql, repositories::user_repository::UserRepository},
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conn = mysql::connect_to_database().await.unwrap();
    let user_repo = UserRepository::new(conn);
    let user_service: Arc<dyn IUserService> = Arc::new(UserService::new(user_repo));

    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users/{user_id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
            .app_data(web::Data::new(user_service.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
