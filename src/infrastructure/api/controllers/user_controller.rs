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

pub async fn create_user(user_service: UserService, user_request: web::Json<UserRequest>) -> impl Responder {
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
    use crate::core::services::user_service_stub::UserServiceStub;
    use actix_web::{
        dev::ServiceResponse,
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        web, App, Route,
    };

    async fn process_test(path: &str, req: TestRequest, route: Route, success: bool) -> ServiceResponse {
        let user_service: Arc<dyn IUserService> = Arc::new(UserServiceStub { success });

        let app = init_service(
            App::new()
                .app_data(web::Data::new(user_service.clone()))
                .route(path, route),
        )
        .await;

        call_service(&app, req.to_request()).await
    }

    #[actix_web::test]
    async fn test_get_users_ok() {
        let resp = process_test(
            "/users",
            TestRequest::get().uri("/users"),
            web::get().to(get_users),
            true,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_users_internal_server_error() {
        let resp = process_test(
            "/users",
            TestRequest::get().uri("/users"),
            web::get().to(get_users),
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_ok() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::get().uri("/users/1"),
            web::get().to(get_user_by_id),
            true,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::get().uri("/users/1"),
            web::get().to(get_user_by_id),
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_create_user_ok() {
        let resp = process_test(
            "/users",
            TestRequest::post().uri("/users").set_json(UserRequest::dummy()),
            web::post().to(create_user),
            true,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_create_user_internal_server_error() {
        let resp = process_test(
            "/users",
            TestRequest::post().uri("/users").set_json(UserRequest::dummy()),
            web::post().to(create_user),
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_update_user_ok() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::put().uri("/users/1").set_json(UserRequest::dummy()),
            web::put().to(update_user),
            true,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_user_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::put().uri("/users/1").set_json(UserRequest::dummy()),
            web::put().to(update_user),
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_delete_user_ok() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::delete().uri("/users/1"),
            web::delete().to(delete_user),
            true,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_delete_user_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::delete().uri("/users/1"),
            web::delete().to(delete_user),
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
