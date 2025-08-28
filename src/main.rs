use pnar_world_api::config::{get_configuration, Settings};
use pnar_world_api::logging::{create_logging_subscriber, init_sub};
use pnar_world_api::startup::Application;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Determine environment and log levels
    let is_production = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into()) == "production";
    let log_level = if is_production { "warn" } else { "info" };
    let rust_log_level = if is_production { "actix_web=warn" } else { "actix_web=info" };

    // Initialize logging
    let subscriber = create_logging_subscriber("api".into(), log_level.into());
    init_sub(subscriber);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", rust_log_level);
    }

    info!("Starting PNAR World API v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let settings: Settings = get_configuration()
        .map_err(|e| {
            error!("Failed to read application configuration: {}", e);
            anyhow::anyhow!("Configuration error: {}", e)
        })?;

    info!("Configuration loaded successfully");
    info!("Server will bind to {}:{}", settings.application.host, settings.application.port);

    // Create and run the application
    let application = match Application::build(settings.clone()).await {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to build application: {}", e);
            error!("Application startup failed - ensure database is running and accessible");
            std::process::exit(1);
        }
    };

    let port = application.port();
    info!("PNAR World API is ready and listening on port {}", port);
    info!("Health check available at: http://localhost:{}/api/v1/health", port);

    // Show Swagger URL only in development
    if !is_production {
        info!("API documentation available at: http://localhost:{}/swagger-ui/index.html", port);
    }

    // Run the application
    application.run_until_stopped().await
        .map_err(|e| {
            error!("Application runtime error: {}", e);
            anyhow::anyhow!("Runtime error: {}", e)
        })?;

    info!("PNAR World API shutdown complete");
    Ok(())
}