use axum::{
    Router,
    routing::{post, put, get, delete},
    middleware, extract::State
};
use std::sync::Arc;
use super::handler::{create_user, update_user, get_user_by_document, delete_user_by_document};
use crate::app::{container::Container, http::middlewares::admin::admin_layer};

pub fn build_routes(State(state): State<Arc<Container>>) -> Router<Arc<Container>> {
    Router::new().route("/", post(create_user))
        .route("/", put(update_user))
        .route("/:document", get(get_user_by_document))
        .route("/:document", delete(delete_user_by_document))
        .layer(middleware::from_fn_with_state(state.clone(), admin_layer ))
}
