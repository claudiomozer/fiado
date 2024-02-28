use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json
};
use std::error::Error as stdError;

use serde::Serialize;
use crate::domain::error::{Error, Kind};


#[derive(Debug)]
pub struct AppError(Error);
impl AppError {
    fn extract_body(&self) -> Body {
        Body { code: self.0.get_code(), msg: self.0.get_message() }
    }

    pub fn from_domain(err: Error) -> AppError {
        AppError(err)
    }

    pub fn get_message(&self) -> String {
        return self.0.get_message();
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

impl stdError for AppError {
    fn source(&self) -> Option<&(dyn stdError + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn stdError> {
        self.source()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.0.get_code(), self.0.get_message())
    }
}