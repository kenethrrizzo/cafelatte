use salvo::prelude::*;

#[handler]
async fn get_users(_res: &mut Response) {}

#[handler]
async fn get_user_by_id(_req: &mut Request, _res: &mut Response) {}

#[handler]
async fn create_user(_req: &mut Request, _res: &mut Response) {}

pub fn router() -> Router {
    Router::with_path("users")
        .get(get_users)
        .post(create_user)
        .push(Router::with_path("<id>").get(get_user_by_id))
}
