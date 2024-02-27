use opentelemetry_sdk::trace::{self, Tracer};
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::runtime;
use opentelemetry_otlp::{self, WithExportConfig};


pub fn init_tracer(otlp_endpoint: &str, service_name: &str) -> Result<Tracer, String>  {
    let otel_service_name = KeyValue::new("service.name", String::from(service_name));
    let result = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint)
        )
        .with_trace_config(
            trace::config().with_resource(
                Resource::new(vec![otel_service_name])
            ),
        ).install_batch(runtime::Tokio);

    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string())
    }
}