use axum::{
    Router,
    routing::{post, put}
};
use std::sync::Arc;
use super::handler::{create_user, update_user};
use crate::app::container::Container;

pub fn build_routes() -> Router<Arc<Container>> {
    return Router::new().route("/", post(create_user))
        .route("/", put(update_user))
}
