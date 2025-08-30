use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
    pub logging: LoggingSettings,
    pub security: SecuritySettings,
    pub monitoring: MonitoringSettings,
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
    pub request_timeout_seconds: u64,
    pub max_request_size: usize,
    pub workers: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: Secret<String>,
    pub expires_in_minutes: i64,
    pub refresh_expires_in_days: i64,
    pub cookie_name: String,
    pub cookie_domain: Option<String>,
    pub cookie_secure: bool,
    pub cookie_http_only: bool,
    pub cookie_same_site: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String, // "json" or "pretty"
    pub file_path: Option<String>,
    pub max_file_size_mb: Option<u64>,
    pub max_files: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    pub rate_limit_requests_per_minute: u32,
    pub rate_limit_burst: u32,
    pub password_min_length: usize,
    pub password_require_special_chars: bool,
    pub password_require_numbers: bool,
    pub password_require_uppercase: bool,
    pub session_timeout_minutes: i64,
    pub max_login_attempts: u32,
    pub lockout_duration_minutes: i64,
    pub trusted_proxies: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringSettings {
    pub metrics_enabled: bool,
    pub health_check_interval_seconds: u64,
    pub performance_monitoring: bool,
    pub error_reporting: bool,
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let mut builder = config::Config::builder()
            .add_source(config::File::from(base_path.join("configuration.yaml")));

        // Add environment-specific configuration if it exists
        let env_config_path = base_path.join(format!("configuration.{}.yaml", environment));
        if env_config_path.exists() {
            builder = builder.add_source(config::File::from(env_config_path));
        }

        // Environment variables override file configuration
        builder = builder.add_source(
            config::Environment::default()
                .separator("_")
        );

        let settings = builder.build()?;
        settings.try_deserialize()
    }

    pub fn environment(&self) -> Environment {
        std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .unwrap_or(Environment::Development)
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment(), Environment::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment(), Environment::Development)
    }
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_seconds)
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
            .username(&self.user)
            .password(self.password.expose_secret())
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
    }

    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_seconds)
    }

    pub fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout_seconds)
    }

    pub fn max_lifetime(&self) -> Duration {
        Duration::from_secs(self.max_lifetime_seconds)
    }
}

impl JwtSettings {
    pub fn access_token_duration(&self) -> Duration {
        Duration::from_secs((self.expires_in_minutes * 60) as u64)
    }

    pub fn refresh_token_duration(&self) -> Duration {
        Duration::from_secs((self.refresh_expires_in_days * 24 * 60 * 60) as u64)
    }
}

impl SecuritySettings {
    pub fn session_timeout(&self) -> Duration {
        Duration::from_secs((self.session_timeout_minutes * 60) as u64)
    }

    pub fn lockout_duration(&self) -> Duration {
        Duration::from_secs((self.lockout_duration_minutes * 60) as u64)
    }
}

/// Application environment
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Test,
    Staging,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Test => "test",
            Environment::Staging => "staging",
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
            "staging" | "stage" => Ok(Self::Staging),
            other => Err(format!(
                "{} is not a supported environment. Use either `development`, `production`, `test`, or `staging`.",
                other
            )),
        }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            base_url: "http://localhost:8000".to_string(),
            cors: CorsSettings::default(),
            request_timeout_seconds: 30,
            max_request_size: 1024 * 1024, // 1MB
            workers: None,
        }
    }
}

impl Default for CorsSettings {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["http://localhost:3000".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            allow_credentials: true,
            max_age: Some(3600),
        }
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            rate_limit_requests_per_minute: 60,
            rate_limit_burst: 10,
            password_min_length: 8,
            password_require_special_chars: true,
            password_require_numbers: true,
            password_require_uppercase: true,
            session_timeout_minutes: 60,
            max_login_attempts: 5,
            lockout_duration_minutes: 15,
            trusted_proxies: vec![],
        }
    }
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_seconds: 30,
            performance_monitoring: true,
            error_reporting: true,
        }
    }
}