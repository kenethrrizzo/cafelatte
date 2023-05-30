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
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

pub async fn get_user_by_id(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    if let Ok(user) = user_service.get_user_by_id(user_id).await {
        HttpResponse::Ok().json(UserResponse::from(user))
    } else {
        HttpResponse::InternalServerError().body("Error")
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
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}
