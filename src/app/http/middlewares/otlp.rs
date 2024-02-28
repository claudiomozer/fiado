use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response
};
use opentelemetry::trace::{Tracer, Span};
use std::sync::Arc;
use crate::app::container::Container;

pub async fn otlp_layer(
    State(_state): State<Arc<Container>>,
    request: Request,
    next: Next,
) -> Response {
    let span_name = format!("{} {}", request.method().to_string(), request.uri()); 

    let mut span = _state.tracer.start(span_name);
    let result = next.run(request).await;
    span.end();
    return result;
}
