use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use salvo_skeleton::{
    core::{ports::user_port::IUserService, services::user_service::UserService},
    infrastructure::{
        api::controllers::user_controller::{
            create_user, delete_user, get_user_by_id, get_users, update_user,
        },
        data::{mysql, repositories::user_repository::UserRepository},
    },
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let conn = mysql::connect_to_database().await.unwrap();
    let user_repo = UserRepository::new(conn);
    let user_service: Arc<dyn IUserService> = Arc::new(UserService::new(user_repo));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/users", web::get().to(get_users))
            .route("/users/{user_id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
            .route("/users/{user_id}", web::put().to(update_user))
            .route("/users/{user_id}", web::delete().to(delete_user))
            .app_data(web::Data::new(user_service.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
