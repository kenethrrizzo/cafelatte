use actix_service::Transform;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
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
            log::debug!("Antes de la ejecución del handler.");
            let res = srv.call(req).await?;
            log::debug!("Después de la ejecución del handler.");

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
