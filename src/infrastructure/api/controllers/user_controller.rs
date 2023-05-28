use salvo::prelude::*;

/// Recibe un request y un response, con el response se envía el mensaje.
#[handler]
pub async fn hello_world(_req: &mut Request, res: &mut Response) {
    res.status_code(StatusCode::OK).render("Hello, World!");
}
