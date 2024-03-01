use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{Response, IntoResponse}
};
use std::sync::Arc;
use crate::app::container::Container;
use crate::app::http::error::AppError;
use crate::domain::error::Error;
use crate::domain::usecases::admin::{MISSING_AUTH_TOKEN, INVALID_TOKEN_ERROR};

pub async fn admin_layer(
    State(_state): State<Arc<Container>>,
    request: Request,
    next: Next,
) -> Response {
    let token = match get_token_from_header(&request) {
        Ok(t) => t,
        Err(e) => return e.into_response()
    };

    if let Err(e) = _state.admin_use_case.validate_token(token).await {
        return AppError::from_domain(e).into_response();
    }

    next.run(request).await
}

fn get_token_from_header(request: &Request) -> Result<String, AppError> {
    if let Some(bearer) = request.headers().get("Authorization") {
        let token_op = match bearer.to_str() {
            Ok(t) => t.split(' ').last(),
            Err(_) => return Err(AppError::from_domain(Error::new_business(MISSING_AUTH_TOKEN)))
        };

        // return invalid token if cannot proceed
        let token = match token_op {
            Some(t) => t,
            None => return Err(AppError::from_domain(Error::new_business(INVALID_TOKEN_ERROR)))
        };

        if token.trim() == "" { 
            return Err(AppError::from_domain(Error::new_business(INVALID_TOKEN_ERROR)))
        }
        return Ok(String::from(token)); 
    }
    Err(AppError::from_domain(Error::new_business(MISSING_AUTH_TOKEN)))
}