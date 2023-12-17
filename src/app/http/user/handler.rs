use axum::{Json, extract::State};
use crate::app::container::Container;
use std::sync::Arc;

use crate::{
    domain::{
        usecases::user::{UserRequestDTO, UserUseCase},
        entities::User,
    },
    app::http::error::AppError
};

pub async fn create_user(State(state): State<Arc<Container>>, Json(payload): Json<UserRequestDTO>)-> Result<Json<User>, AppError> {
    Ok(Json(payload.to_user().unwrap()))
}
