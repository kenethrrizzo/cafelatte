use crate::{
    core::ports::user_port::IUserService,
    infrastructure::api::dto::user::{UserRequest, UserResponse},
};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

type UserService = web::Data<Arc<dyn IUserService>>;

pub async fn get_users(user_service: UserService) -> impl Responder {
    match user_service.get_users().await {
        Ok(users) => {
            let mut response: Vec<UserResponse> = vec![];
            for user in users {
                response.push(UserResponse::from(user));
            }

            HttpResponse::Ok().json(response)
        }
        Err(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
    }
}

pub async fn get_user_by_id(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.get_user_by_id(user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
    }
}

pub async fn create_user(
    user_service: UserService,
    user_request: web::Json<UserRequest>,
) -> impl Responder {
    match user_service
        .create_user(UserRequest::to_user_core(&user_request))
        .await
    {
        Ok(_) => HttpResponse::Created().json("User created."),
        Err(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
    }
}

pub async fn update_user(
    user_service: UserService,
    path: web::Path<u8>,
    user_request: web::Json<UserRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    match user_service
        .update_user(user_id as i32, UserRequest::to_user_core(&user_request))
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User updated."),
        Err(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
    }
}

pub async fn delete_user(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.delete_user(user_id as i32).await {
        Ok(_) => HttpResponse::Ok().json("User deleted."),
        Err(msg) => HttpResponse::InternalServerError().json(msg.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ports::user_port::IUserService;
    use crate::core::services::user_service_stub::UserServiceStub;
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
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_users_internal_server_error() {
        let resp = process_tests(false).await;
        assert_eq!(
            resp.status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
