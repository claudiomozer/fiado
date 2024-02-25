pub mod error;
pub mod user;
pub mod middlewares;

use axum::extract::State;
use super::container::Container;
use axum::routing::Router;
use std::sync::Arc;

pub fn build_app(container: Container) -> Router {
    let state = Arc::new(container);
    Router::new()
        .nest("/users", user::route::build_routes(State(state.clone())))
        .with_state(state)
} 