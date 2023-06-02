#[cfg(test)]
mod tests {
    use crate::core::ports::user_port::IUserService;
    use crate::infrastructure::api::controllers::user_controller::get_users;
    use crate::tests::core::services::user_service_stub::UserServiceStub;
    use actix_web::{dev::ServiceResponse, test, web, App};
    use std::sync::Arc;

    async fn process_tests(success: bool) -> ServiceResponse {
        let user_service: Arc<dyn IUserService> = Arc::new(UserServiceStub { success });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(user_service.clone()))
                .route("/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        test::call_service(&app, req).await
    }

    #[actix_web::test]
    async fn test_get_users_ok() {
        let resp = process_tests(true).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_users_not_ok() {
        let resp = process_tests(false).await;
        assert!(resp.status().is_server_error());
    }
}
