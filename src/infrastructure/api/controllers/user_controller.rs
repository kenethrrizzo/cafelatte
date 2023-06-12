use crate::{
    core::{errors::user_errors::UserError, ports::user_port::IUserService},
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
        Err(err) => match &err {
            UserError::NotFound => HttpResponse::NotFound().body(err.to_string()),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}

pub async fn get_user_by_id(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.get_user_by_id(user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(err) => match &err {
            UserError::NotFound => HttpResponse::NotFound().body(err.to_string()),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}

pub async fn create_user(user_service: UserService, user_request: web::Json<UserRequest>) -> impl Responder {
    match user_service
        .create_user(UserRequest::to_user_core(&user_request))
        .await
    {
        Ok(_) => HttpResponse::Created().json("User created."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
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
        Err(err) => match &err {
            UserError::NotFound => HttpResponse::NotFound().body(err.to_string()),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}

pub async fn delete_user(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.delete_user(user_id as i32).await {
        Ok(_) => HttpResponse::Ok().json("User deleted."),
        Err(err) => match &err {
            UserError::NotFound => HttpResponse::NotFound().body(err.to_string()),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
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

    async fn process_test(path: &str, req: TestRequest, route: Route, status_code: i32) -> ServiceResponse {
        let user_service: Arc<dyn IUserService> = Arc::new(UserServiceStub { status_code });

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
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_users_not_found_error() {
        let resp = process_test(
            "/users",
            TestRequest::get().uri("/users"),
            web::get().to(get_users),
            404,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_users_internal_server_error() {
        let resp = process_test(
            "/users",
            TestRequest::get().uri("/users"),
            web::get().to(get_users),
            500,
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
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_not_foud_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::get().uri("/users/1"),
            web::get().to(get_user_by_id),
            404,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::get().uri("/users/1"),
            web::get().to(get_user_by_id),
            500,
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
            200,
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
            500,
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
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_user_not_found_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::put().uri("/users/1").set_json(UserRequest::dummy()),
            web::put().to(update_user),
            404,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_update_user_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::put().uri("/users/1").set_json(UserRequest::dummy()),
            web::put().to(update_user),
            500,
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
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_delete_user_not_found_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::delete().uri("/users/1"),
            web::delete().to(delete_user),
            404,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_user_internal_server_error() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::delete().uri("/users/1"),
            web::delete().to(delete_user),
            500,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
