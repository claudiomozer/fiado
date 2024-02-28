use axum::{Json, extract::{State, Path}};
use opentelemetry::trace::{Tracer, Span};
use crate::app::container::Container;
use std::sync::Arc;

use crate::{
    domain::usecases::user::{UserCreateRequestDTO, UserUpdateRequestDTO, PublicUserResponseDTO},
    app::http::error::AppError
};

pub async fn create_user(State(state): State<Arc<Container>>, Json(payload): Json<UserCreateRequestDTO>)-> Result<(), AppError> {
    let mut span = state.tracer.start("create.user");
    let result = match state.user_use_case.create(payload).await {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::from_domain(err))
    };
    span.end();
    return result;
}

pub async fn update_user(State(state): State<Arc<Container>>, Json(payload): Json<UserUpdateRequestDTO>)-> Result<(), AppError> {
    let mut span = state.tracer.start("update.user");
    let result = match state.user_use_case.update(payload).await {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::from_domain(err))
    };
    span.end();
    return result;
}

pub async fn get_user_by_document(State(state): State<Arc<Container>>, Path(document): Path<String>)-> Result<Json<PublicUserResponseDTO>, AppError> {
    let mut span = state.tracer.start("get.user");
    let result = match state.user_use_case.get(document.as_str()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(AppError::from_domain(err))
    };
    span.end();
    return result;
}

pub async fn delete_user_by_document(State(state): State<Arc<Container>>, Path(document): Path<String>)-> Result<(), AppError> {
    let mut span = state.tracer.start("delete.user");
    let result =     match state.user_use_case.delete(document.as_str()).await {
        Ok(()) => Ok(()),
        Err(err) => Err(AppError::from_domain(err))
    };
    span.end();
    return result;
}
