use fiadors::app::{http, container::Container};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let container = Container::load_dependencies().await;

    sqlx::migrate!("./migrations")
    .run(&container.pg_pool.clone())
    .await.unwrap();

    let app = http::build_app(container);

    let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}