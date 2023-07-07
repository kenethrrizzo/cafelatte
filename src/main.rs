use actix_web::{middleware::Logger, web, App, HttpServer};
use cafelatte::{
    core::{ports::user_port::IUserService, services::user_service::UserService},
    infrastructure::{
        api::handlers::user_handler::routes,
        data::{mysql, repositories::user_repository::UserRepository},
    },
};
use std::{env, io, sync};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    log::info!("Loading environment variables...");
    dotenv::dotenv().ok();

    log::info!("Connecting to database...");
    let conn = mysql::connect_to_database().await.unwrap();
    let user_repo = UserRepository::new(conn);
    let user_service: sync::Arc<dyn IUserService> = sync::Arc::new(UserService::new(user_repo));

    let server_port = env::var("SERVER_PORT").unwrap().parse::<u16>().unwrap();
    log::info!("Listening server on port: {:?}", server_port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("").configure(routes))
            .app_data(web::Data::new(user_service.clone()))
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
