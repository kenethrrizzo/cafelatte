use salvo::prelude::*;

/// Pinta un string estático.
#[handler]
async fn hello() -> &'static str {
    "Hello, World!"
}

/// Recibe un request y un response, con el response se envía el mensaje.
#[handler]
async fn hello_world(_req: &mut Request, res: &mut Response) {
    res.status_code(StatusCode::OK).render("Hello, World!");
}

pub fn router() -> Router {
    Router::with_path("tests").get(hello_world).get(hello)
}
