use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use salvo_skeleton::infrastructure::api::controllers::user_controller::hello_world;

#[tokio::main]
async fn main() {
    let router: Router = Router::with_path("hello-world").get(hello_world);
    let acceptor: TcpAcceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
