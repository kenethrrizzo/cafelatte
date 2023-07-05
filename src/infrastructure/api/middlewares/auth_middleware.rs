use crate::utils::security_util::verify_jwt_token;
use actix_service::Transform;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    Error,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;

pub struct AuthenticateMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticateMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        futures::FutureExt::boxed_local(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(token) = auth_header.to_str() {
                    let is_valid = verify_jwt_token(token.to_string());

                    if let Err(_) = is_valid {
                        return Err(ErrorUnauthorized("Invalid token."));
                    }
                } else {
                    return Err(ErrorUnauthorized("Error while parsing header."));
                }
            } else {
                return Err(ErrorUnauthorized("Authorization header not found."));
            }

            let res = srv.call(req).await?;

            Ok(res)
        })
    }
}

pub struct AuthenticateMiddlewareFactory {}

impl AuthenticateMiddlewareFactory {
    pub fn new() -> Self {
        AuthenticateMiddlewareFactory {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthenticateMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticateMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateMiddleware {
            service: Rc::new(service),
        }))
    }
}
