use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
    pub logging: LoggingSettings,
}

/// Load configuration from files and environment variables
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    Settings::load()
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub cors: CorsSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: Secret<String>,
    pub expires_in_minutes: i64,
    pub refresh_expires_in_days: i64,
    pub cookie_name: String,
    pub cookie_domain: Option<String>,
    pub cookie_secure: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String, // "json" or "pretty"
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("configuration");

        // Detect the running environment
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT");

        let environment_filename = format!("{}.yaml", environment.as_str());
        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        settings.try_deserialize()
    }
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl DatabaseSettings {
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
    }

    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
}

/// Application environment
#[derive(Debug, Clone)]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Test => "test",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(format!(
                "{} is not a supported environment. Use either `development`, `production`, or `test`.",
                other
            )),
        }
    }
}
