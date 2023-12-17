use fiadors::app::{http, container::Container};

#[tokio::main]
async fn main() {
    let container = Container::load_dependencies().await;
    let app = http::build_app(container);

    axum::Server::bind(&"0.0.0.0:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
