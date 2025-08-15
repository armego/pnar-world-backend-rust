use pnar_world_api::config::{get_configuration, Settings};
use pnar_world_api::logging::{create_logging_subscriber, init_sub};
use pnar_world_api::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logging subscriber of the application.
    let subscriber = create_logging_subscriber("api".into(), "info".into());
    init_sub(subscriber);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Load the application configuration
    let settings: Settings = get_configuration().expect("Failed to read app configuration");

    // Create and run the application
    let application = Application::build(settings).await?;
    application.run_until_stopped().await?;
    
    Ok(())
}
