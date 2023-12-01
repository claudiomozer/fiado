use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json
};

use serde::Serialize;
use crate::domain::error::{Error, Kind};


pub struct AppError(Error);
impl AppError {
    fn extract_body(&self) -> Body {
        Body { code: self.0.get_code(), msg: self.0.get_message() }
    }
}

#[derive(Serialize)]
pub struct Body {
    code: u8,
    msg: String
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = self.extract_body();
        match self.0.get_kind() {
            Kind::Business =>  return (StatusCode::BAD_REQUEST, Json(body)).into_response(),
            Kind::NotFound =>  return (StatusCode::NOT_FOUND, Json(body)).into_response(),
            Kind::Internal =>  return (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response(),
            Kind::AlreadyExists => return (StatusCode::CONFLICT, Json(body)).into_response()
        }
    }
}
