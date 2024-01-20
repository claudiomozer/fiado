use axum::{Json, extract::State};
use crate::app::container::Container;
use std::sync::Arc;

use crate::{
    domain::usecases::user::{UserCreateRequestDTO, UserUpdateRequestDTO},
    app::http::error::AppError
};

pub async fn create_user(State(state): State<Arc<Container>>, Json(payload): Json<UserCreateRequestDTO>)-> Result<(), AppError> {
    match state.user_use_case.create(payload).await {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::from_domain(err))
    }
}

pub async fn update_user(State(state): State<Arc<Container>>, Json(payload): Json<UserUpdateRequestDTO>)-> Result<(), AppError> {
    match state.user_use_case.update(payload).await {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::from_domain(err))
    }
}

