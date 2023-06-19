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
use std::{env, io::Result, sync::Arc};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    log::info!("Loading environment variables...");
    dotenv().ok();

    log::info!("Connecting to database...");
    let conn = mysql::connect_to_database().await.unwrap();
    let user_repo = UserRepository::new(conn);
    let user_service: Arc<dyn IUserService> = Arc::new(UserService::new(user_repo));

    let server_port = env::var("SERVER_PORT").unwrap().parse::<u16>().unwrap();
    log::info!("Listening server on port: {:?}", server_port);
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
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
