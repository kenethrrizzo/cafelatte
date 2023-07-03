use crate::core::{errors::user_errors::UserError, ports::user_port::IUserService};
use crate::infrastructure::api::dto::user_request::LoginRequest;
use crate::infrastructure::api::dto::user_response::LoginResponse;
use crate::infrastructure::api::dto::{user_request::UserRequest, user_response::UserResponse};
use actix_web::{web, HttpResponse, Responder};

type UserService = web::Data<std::sync::Arc<dyn IUserService>>;

pub async fn register(
    user_service: UserService,
    user_request: web::Json<UserRequest>,
) -> impl Responder {
    if !user_request.is_valid() {
        return HttpResponse::BadRequest().body("Invalid fields.");
    }

    match user_service
        .register(UserRequest::to_user_core(&user_request))
        .await
    {
        Ok(res) => HttpResponse::Created().json(LoginResponse::from_login(res)),
        Err(err) => {
            log::error!("{:?}: ha ocurrido un error inesperado", err.to_string());
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn login(
    user_service: UserService,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    if !login_request.is_valid() {
        return HttpResponse::BadRequest().body("Invalid fields.");
    }

    match user_service
        .login(login_request.get_email(), login_request.get_password())
        .await
    {
        Ok(res) => HttpResponse::Ok().json(LoginResponse::from_login(res)),
        Err(err) => {
            log::error!("{:?}: ha ocurrido un error inesperado", err.to_string());
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_users(user_service: UserService) -> impl Responder {
    match user_service.get_users().await {
        Ok(users) => HttpResponse::Ok().json(UserResponse::from_user_core_vec(users)),
        Err(err) => match &err {
            UserError::NotFound => {
                log::error!("{:?}: no se ha encontrado ningún usuario", err.to_string());
                HttpResponse::NotFound().body(err.to_string())
            }
            _ => {
                log::error!("{:?}: ha ocurrido un error inesperado", err.to_string());
                HttpResponse::InternalServerError().body(err.to_string())
            }
        },
    }
}

pub async fn get_user_by_id(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.get_user_by_id(user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from_user_core(user)),
        Err(err) => match &err {
            UserError::NotFound => {
                log::error!(
                    "{:?}: usuario con ID <{:?}> no encontrado",
                    err.to_string(),
                    user_id
                );
                HttpResponse::NotFound().body(err.to_string())
            }
            _ => {
                log::error!("{:?}: ha ocurrido un error inesperado", err.to_string());
                HttpResponse::InternalServerError().body(err.to_string())
            }
        },
    }
}

pub async fn update_user(
    user_service: UserService,
    path: web::Path<u8>,
    user_request: web::Json<UserRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    if !user_request.is_valid() {
        return HttpResponse::BadRequest().body("Invalid fields.");
    }

    match user_service
        .update_user(user_id as i32, UserRequest::to_user_core(&user_request))
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User updated."),
        Err(err) => match &err {
            UserError::NotFound => {
                log::error!(
                    "{:?}: usuario con ID <{:?}> no encontrado",
                    err.to_string(),
                    user_id
                );
                HttpResponse::NotFound().body(err.to_string())
            }
            _ => {
                log::error!("{:?}: ha ocurrido un error inesperado", err.to_string());
                HttpResponse::InternalServerError().body(err.to_string())
            }
        },
    }
}

pub async fn delete_user(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.delete_user(user_id as i32).await {
        Ok(_) => HttpResponse::Ok().json("User deleted."),
        Err(err) => match &err {
            UserError::NotFound => {
                log::error!("{:?}: usuario con ID <{:?}> no encontrado", err, user_id);
                HttpResponse::NotFound().body(err.to_string())
            }
            _ => {
                log::error!("{:?}: ha ocurrido un error inesperado", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        },
    }
}

#[cfg(test)]
mod user_handler_tests {
    use super::*;
    use crate::core::services::user_service_stub::UserServiceStub;
    use actix_web::{
        dev::ServiceResponse,
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        web, App, Route,
    };

    async fn process_test(
        path: &str,
        req: TestRequest,
        route: Route,
        status_code: i32,
    ) -> ServiceResponse {
        let user_service: std::sync::Arc<dyn IUserService> =
            std::sync::Arc::new(UserServiceStub { status_code });

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
    async fn test_register_ok() {
        let resp = process_test(
            "/users",
            TestRequest::post()
                .uri("/users")
                .set_json(UserRequest::dummy()),
            web::post().to(register),
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_register_bad_request_when_request_is_not_valid() {
        let resp = process_test(
            "/users",
            TestRequest::post().uri("/users").set_json(UserRequest {
                name: "Keneth".to_string(),
                surname: "None".to_string(),
                phone_number: None,
                email: "anything".to_string(),
                password: "nasd8hj819".to_string(),
            }),
            web::post().to(register),
            200,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_register_internal_server_error() {
        let resp = process_test(
            "/users",
            TestRequest::post()
                .uri("/users")
                .set_json(UserRequest::dummy()),
            web::post().to(register),
            500,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_update_user_ok() {
        let resp = process_test(
            "/users/{user_id}",
            TestRequest::put()
                .uri("/users/1")
                .set_json(UserRequest::dummy()),
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
            TestRequest::put()
                .uri("/users/1")
                .set_json(UserRequest::dummy()),
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
            TestRequest::put()
                .uri("/users/1")
                .set_json(UserRequest::dummy()),
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
