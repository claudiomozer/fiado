use fiadors::app::{http, container::Container};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let container = Container::load_dependencies().await;
    let app = http::build_app(container);

    let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// TODO: move spans to handlers
// TODO: close tracer at container drop
