use axum::{
    Router,
    routing::{get, post},
    Json,
};

use fiadors::{
    domain::{
        usecases::user::UserRequestDTO,
        entities::User,
    },
    app::error::AppError
};

#[tokio::main]
async fn main() {
    let mut app = Router::new();

    app = app.route("/", get(|| async {"Hello World"}))
    .route("/users", post(create_user));


    axum::Server::bind(&"0.0.0.0:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(Json(payload): Json<UserRequestDTO>)-> Result<Json<User>, AppError> {
    Ok(Json(payload.to_user()))
}
