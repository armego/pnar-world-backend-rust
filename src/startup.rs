use crate::{
    config::Settings, database::create_connection_pool, error::{AppResult, AppError}, handlers,
    middleware::auth::AuthMiddleware, openapi::ApiDoc, state::AppState,
};
use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(settings: Settings) -> AppResult<Self> {
        let app_state = web::Data::new(AppState::new());
        
        // Try to connect to database once
        let pool = match create_connection_pool(&settings.database).await {
            Ok(pool) => {
                info!("Database connection established successfully");
                app_state.set_db_pool(pool.clone()).await;
                pool
            }
            Err(e) => {
                info!("Failed to connect to database: {}. Starting without database connection.", e);
                return Err(e.into());
            }
        };
        let _pool_data = web::Data::new(pool);

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
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

    let pool_data = match app_state.get_db_pool().await {
        Some(pool) => web::Data::new(pool),
        None => return Err(AppError::Internal("Database connection not available".to_string())),
    };
    let pool_data = pool_data.clone();

    let server = HttpServer::new(move || {
        let _cors = configure_cors(&settings_data.application.cors);
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(app_state.clone())
            .app_data(settings_data.clone())
            .app_data(pool_data.clone())
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi),
            )
            .route(
                "/docs",
                web::get().to(|| async {
                    actix_web::HttpResponse::Found()
                        .append_header(("Location", "/swagger-ui/index.html"))
                        .finish()
                }),
            )
            .service(
                web::scope("/api/v1")
                    .service(handlers::health::health_check)
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
                                .service(handlers::user::verify_email),
                        ),
                    )
                    .service(
                        web::scope("/dictionary")
                            .wrap(AuthMiddleware)
                            .service(handlers::dictionary::create_entry)
                            .service(handlers::dictionary::get_entry)
                            .service(handlers::dictionary::list_entries)
                            .service(handlers::dictionary::search_entries)
                            .service(handlers::dictionary::update_entry)
                            .service(handlers::dictionary::delete_entry)
                            .service(handlers::dictionary::verify_entry),
                    )
                    .service(
                        web::scope("/translations")
                            .wrap(AuthMiddleware)
                            .route(
                                "",
                                web::post().to(handlers::translation::create_translation),
                            )
                            .route("", web::get().to(handlers::translation::list_translations))
                            .route(
                                "/{id}",
                                web::get().to(handlers::translation::get_translation),
                            )
                            .route(
                                "/{id}",
                                web::put().to(handlers::translation::update_translation),
                            )
                            .route(
                                "/{id}",
                                web::delete().to(handlers::translation::delete_translation),
                            ),
                    )
                    .service(
                        web::scope("/contributions")
                            .wrap(AuthMiddleware)
                            .route(
                                "",
                                web::post().to(handlers::contribution::create_contribution),
                            )
                            .route(
                                "",
                                web::get().to(handlers::contribution::list_contributions),
                            )
                            .route(
                                "/{id}",
                                web::get().to(handlers::contribution::get_contribution),
                            )
                            .route(
                                "/{id}",
                                web::put().to(handlers::contribution::update_contribution),
                            )
                            .route(
                                "/{id}",
                                web::delete().to(handlers::contribution::delete_contribution),
                            ),
                    )
                    .service(
                        web::scope("/analytics")
                            .route(
                                "/anonymous",
                                web::post().to(handlers::analytics::create_anonymous_analytics),
                            )
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware)
                                    .route(
                                        "",
                                        web::post().to(handlers::analytics::create_analytics),
                                    )
                                    .route("", web::get().to(handlers::analytics::list_analytics))
                                    .route(
                                        "/{id}",
                                        web::get().to(handlers::analytics::get_analytics),
                                    )
                                    .route(
                                        "/{id}",
                                        web::put().to(handlers::analytics::update_analytics),
                                    )
                                    .route(
                                        "/{id}",
                                        web::delete().to(handlers::analytics::delete_analytics),
                                    )
                                    .route(
                                        "/words/{word_id}/stats",
                                        web::get().to(handlers::analytics::get_word_stats),
                                    ),
                            ),
                    )
                    .service(
                        web::scope("/alphabets")
                            .route("", web::get().to(handlers::alphabet::list_alphabets))
                            .route("/convert", web::post().to(handlers::alphabet::convert_text)),
                    )
                    .service(
                        web::scope("/roles")
                            .route("", web::get().to(handlers::roles::list_roles)),
                    ),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

fn configure_cors(cors_settings: &crate::config::CorsSettings) -> Cors {
    let mut cors = Cors::default();

    if cors_settings.allowed_origins.contains(&"*".to_string()) {
        cors = cors.allow_any_origin();
    } else {
        for origin in &cors_settings.allowed_origins {
            cors = cors.allowed_origin(origin);
        }
    }

    let method_strs: Vec<&str> = cors_settings
        .allowed_methods
        .iter()
        .map(|s| s.as_str())
        .collect();
    let mut cors = cors.allowed_methods(method_strs);

    for header in &cors_settings.allowed_headers {
        cors = cors.allowed_header(header.as_str());
    }

    if cors_settings.allow_credentials {
        cors = cors.supports_credentials();
    }

    cors
}

pub fn init_tracing(settings: &crate::config::LoggingSettings) -> AppResult<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&settings.level));

    let subscriber = tracing_subscriber::registry().with(env_filter);

    match settings.format.as_str() {
        "json" => {
            let json_layer = tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(false)
                .with_span_list(true);
            subscriber.with(json_layer).init();
        }
        _ => {
            let pretty_layer = tracing_subscriber::fmt::layer()
                .pretty()
                .with_target(true)
                .with_thread_ids(true);
            subscriber.with(pretty_layer).init();
        }
    }

    info!("Tracing initialized with level: {}", settings.level);
    Ok(())
}
