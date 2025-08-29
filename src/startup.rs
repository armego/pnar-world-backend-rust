use crate::{
    config::Settings,
    database::create_connection_pool,
    error::{AppResult, AppError},
    handlers,
    middleware::{
        auth::AuthMiddleware,
        security::{SecurityHeaders, RequestId}
    },
    state::AppState,
};
use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use std::net::TcpListener;
use tracing::{info, warn};

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(settings: Settings) -> AppResult<Self> {
        // Create database connection pool - assume DB is running
        info!("Establishing database connection...");
        let pool = create_connection_pool(&settings.database).await
            .map_err(|e| {
                tracing::error!("Failed to connect to database: {}", e);
                AppError::Internal(format!("Database connection failed: {}", e))
            })?;

        info!("Database connection established successfully");

        // Create app state with database pool already set
        let mut app_state = AppState::new();
        app_state.set_db_pool(pool);
        let app_state = web::Data::new(app_state);        let address = settings.application.get_address();
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, app_state, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    app_state: web::Data<AppState>,
    settings: Settings,
) -> AppResult<actix_web::dev::Server> {
    let settings_data = web::Data::new(settings);

    let pool_data = match app_state.get_db_pool() {
        Some(pool) => web::Data::new((*pool).clone()),
        None => return Err(AppError::Internal("Database connection not available".to_string())),
    };

    // Configure workers based on settings or CPU count
    let workers = settings_data.application.workers.unwrap_or_else(|| {
        let cpu_count = num_cpus::get();
        std::cmp::max(1, cpu_count)
    });

    info!("Starting HTTP server with {} workers", workers);

    let server = HttpServer::new(move || {
        let cors = configure_cors(&settings_data.application.cors, settings_data.is_production());
        let _is_dev = !settings_data.is_production();

        let app = App::new()
            .app_data(app_state.clone())
            .app_data(settings_data.clone())
            .app_data(pool_data.clone())
            .app_data(
                web::PayloadConfig::new(settings_data.application.max_request_size)
            )
            .wrap(NormalizePath::trim())
            .wrap(RequestId)
            .wrap(SecurityHeaders)
            .wrap(cors)
            .wrap(Logger::default()); // Keep logging for now

        // Swagger removed - use simple-api-docs.html instead

        app.service(
                web::scope("/api/v1")
                    // Health and monitoring endpoints (no auth required)
                    .service(handlers::health::health_check)
                    .service(handlers::health::readiness_check)
                    .service(handlers::health::liveness_check)
                    .service(handlers::health::metrics)
                    
                    // Authentication endpoints
                    .service(
                        web::scope("/auth")
                            .service(handlers::auth::register)
                            .service(handlers::auth::login)
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware)
                                    .service(handlers::auth::logout)
                                    .service(handlers::auth::profile),
                            ),
                    )
                    
                    // User management endpoints
                    .service(
                        web::scope("/users").service(
                            web::scope("")
                                .wrap(AuthMiddleware)
                                .service(handlers::user::create_user)
                                .service(handlers::user::list_users)
                                .service(handlers::user::get_user_by_email)
                                .service(handlers::user::get_current_user)
                                .service(handlers::user::update_current_user)
                                .service(handlers::user::update_current_user_password)
                                .service(handlers::user::delete_current_user)
                                .service(handlers::user::get_user)
                                .service(handlers::user::update_user)
                                .service(handlers::user::update_user_password)
                                .service(handlers::user::delete_user)
                                .service(handlers::user::award_points)
                                .service(handlers::user::verify_email)
                                .service(handlers::user::update_user_role),
                        ),
                    )
                    
                    // Dictionary endpoints
                    .service(
                        web::scope("/dictionary")
                            // Public read endpoints
                            .service(handlers::dictionary::get_entry)
                            .service(handlers::dictionary::list_entries)
                            .service(handlers::dictionary::search_entries)
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware) // Protected CUD endpoints require auth
                                    .service(handlers::dictionary::create_entry)
                                    .service(handlers::dictionary::update_entry)
                                    .service(handlers::dictionary::delete_entry)
                                    .service(handlers::dictionary::verify_entry),
                            ),
                    )
                    
                    // Translation endpoints
                    .service(
                        web::scope("/translations")
                            // Public read endpoints (no auth required)
                            .service(handlers::translation::list_translations)
                            .service(handlers::translation::get_translation)
                    )
                    // Protected translation endpoints require auth
                    .service(
                        web::scope("/translations")
                            .wrap(AuthMiddleware)
                            .service(handlers::translation::create_translation)
                            .service(handlers::translation::update_translation)
                            .service(handlers::translation::delete_translation)
                    )
                    
                    // Contribution endpoints
                    .service(
                        web::scope("/contributions")
                            // Public read endpoints (no auth required)
                            .service(handlers::contribution::list_contributions)
                            .service(handlers::contribution::get_contribution)
                    )
                    // Protected contribution endpoints require auth
                    .service(
                        web::scope("/contributions")
                            .wrap(AuthMiddleware)
                            .service(handlers::contribution::create_contribution)
                            .service(handlers::contribution::update_contribution)
                            .service(handlers::contribution::delete_contribution)
                    )
                    
                    // Analytics endpoints - Public
                    .service(
                        web::scope("/analytics")
                            .route(
                                "/anonymous",
                                web::post().to(handlers::analytics::create_anonymous_analytics),
                            )
                            .route("", web::get().to(handlers::analytics::list_analytics))
                            .route("/{id}", web::get().to(handlers::analytics::get_analytics))
                            .route(
                                "/words/{word_id}/stats",
                                web::get().to(handlers::analytics::get_word_stats),
                            ),
                    )
                    // Analytics endpoints - Protected
                    .service(
                        web::scope("/analytics")
                            .wrap(AuthMiddleware)
                            .route("", web::post().to(handlers::analytics::create_analytics))
                            .route("/{id}", web::put().to(handlers::analytics::update_analytics))
                            .route("/{id}", web::delete().to(handlers::analytics::delete_analytics)),
                    )
                    
                    // Book management endpoints
                    .service(
                        web::scope("/books")
                            .service(handlers::book::list_books) // Public endpoint for public books
                            .service(handlers::book::get_book)   // Public endpoint for public books
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware) // Protected endpoints require auth
                                    .service(handlers::book::create_book)
                                    .service(handlers::book::update_book)
                                    .service(handlers::book::delete_book)
                                    .service(handlers::book::get_my_books),
                            ),
                    )
                    
                    // Public endpoints (no auth required)
                    .service(
                        web::scope("/alphabets")
                            .route("", web::get().to(handlers::alphabet::list_alphabets))
                            .route("/convert", web::post().to(handlers::alphabet::convert_text)),
                    )
                    // Role management endpoints
                    .service(
                        web::scope("/roles")
                            .service(handlers::roles::list_roles) // Public endpoint
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware) // Protected endpoints require auth
                                    .service(handlers::roles::list_assignable_roles)
                                    .service(handlers::roles::list_manageable_roles),
                            ),
                    )
                    // Notification endpoints
                    .service(
                        web::scope("/notifications")
                            .wrap(AuthMiddleware) // All notification endpoints require auth (user-specific)
                            .service(handlers::notification::get_notification)
                            .service(handlers::notification::list_notifications)
                            .service(handlers::notification::get_unread_count)
                            .service(handlers::notification::create_notification)
                            .service(handlers::notification::update_notification)
                            .service(handlers::notification::mark_notification_read)
                            .service(handlers::notification::delete_notification)
                            .service(handlers::notification::mark_all_notifications_read)
                    ),
            )
    })
    .workers(workers)
    .listen(listener)?
    .run();

    Ok(server)
}

fn configure_cors(cors_settings: &crate::config::CorsSettings, is_production: bool) -> Cors {
    if !is_production {
        // In development, allow any origin, method, and header for easier testing
        warn!("CORS is disabled for development environment");
        return Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
    }

    let mut cors = Cors::default();

    // Configure origins
    if cors_settings.allowed_origins.contains(&"*".to_string()) {
        // Production should not use wildcard, but if it's there, respect it with a warning
        warn!("Wildcard CORS origin detected in production environment. This is a security risk.");
        cors = cors.allow_any_origin();
    } else {
        for origin in &cors_settings.allowed_origins {
            cors = cors.allowed_origin(origin);
        }
    }
    
    // In production, be more restrictive
    cors = cors.supports_credentials();

    // Configure methods
    let method_strs: Vec<&str> = cors_settings
        .allowed_methods
        .iter()
        .map(|s| s.as_str())
        .collect();
    cors = cors.allowed_methods(method_strs);

    // Configure headers
    for header in &cors_settings.allowed_headers {
        cors = cors.allowed_header(header.as_str());
    }

    // Configure max age - longer cache in production
    if let Some(max_age) = cors_settings.max_age {
        cors = cors.max_age(max_age);
    } else {
        cors = cors.max_age(86400); // 24 hours default for production
    }

    cors
}
