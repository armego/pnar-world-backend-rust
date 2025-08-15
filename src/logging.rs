use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

/// Create a logging subscriber based on the app name and log level
pub fn create_logging_subscriber(name: String, level: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Initialize the subscriber as the global default
pub fn init_sub(subscriber: impl Subscriber + Send + Sync) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
