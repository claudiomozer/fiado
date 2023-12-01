use axum::{Json, extract::State};
use std::sync::Arc;

use crate::{
    domain::{
        usecases::user::{UserRequestDTO, UserUseCase},
        entities::User,
    },
    app::http::error::AppError
};

pub struct UserState {
    user_use_case: Box<dyn UserUseCase>
}

pub async fn create_user(State(state): State<Arc<UserState>>, Json(payload): Json<UserRequestDTO>)-> Result<Json<User>, AppError> {
    Ok(Json(payload.to_user().unwrap()))
}

