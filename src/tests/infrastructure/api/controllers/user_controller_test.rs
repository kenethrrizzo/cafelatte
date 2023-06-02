#[cfg(test)]
mod tests {
    use crate::{
        core::ports::user_port::IUserService,
        infrastructure::api::controllers::user_controller::get_users,
        tests::core::services::user_service_stub::UserServiceStub,
    };
    use actix_web::{App, test, web};
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_get_users_ok() {
        let user_service: Arc<dyn IUserService> = Arc::new(UserServiceStub { success: true });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(user_service.clone()))
                .route("/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_users_not_ok() {
        let user_service: Arc<dyn IUserService> = Arc::new(UserServiceStub { success: false });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(user_service.clone()))
                .route("/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_server_error());
    }
}
