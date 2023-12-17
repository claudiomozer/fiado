pub mod error;
pub mod user;

use super::container::Container;
use axum::routing::Router;
use std::sync::Arc;

pub fn build_app(container: Container) -> Router {
    Router::new()
        .nest("/users", user::route::build_routes())
        .with_state(Arc::new(container))
} 