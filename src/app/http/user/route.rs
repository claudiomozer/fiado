use axum::{
    Router,
    routing::{post, put, get, delete}
};
use std::sync::Arc;
use super::handler::{create_user, update_user, get_user_by_document, delete_user_by_document};
use crate::app::container::Container;

pub fn build_routes() -> Router<Arc<Container>> {
    return Router::new().route("/", post(create_user))
        .route("/", put(update_user))
        .route("/:document", get(get_user_by_document))
        .route("/:document", delete(delete_user_by_document))
}
