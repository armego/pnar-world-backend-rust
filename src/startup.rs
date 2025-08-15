use crate::{
    config::Settings,
    database::create_connection_pool,
    error::AppResult,
    handlers,
    middleware::auth::AuthMiddleware,
    openapi::ApiDoc,
};
use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use sqlx::PgPool;
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
        let connection_pool = create_connection_pool(&settings.database).await?;

        let address = format!("{}:{}", settings.application.host, settings.application.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        
        let server = run(listener, connection_pool, settings)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(
    listener: TcpListener,
    db_pool: PgPool,
    settings: Settings,
) -> AppResult<actix_web::dev::Server> {
    let db_pool = web::Data::new(db_pool);
    let settings_data = web::Data::new(settings.clone());

    let server = HttpServer::new(move || {
        let cors = configure_cors(&settings.application.cors);
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(db_pool.clone())
            .app_data(settings_data.clone())
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
            .route("/docs", web::get().to(|| async {
                actix_web::HttpResponse::Found()
                    .append_header(("Location", "/swagger-ui/index.html"))
                    .finish()
            }))
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
                                    .service(handlers::auth::profile)
                            )
                    )
                    .service(
                        web::scope("/users")
                            .service(handlers::user::create_user)
                            .service(handlers::user::list_users)
                            .service(handlers::user::get_user_by_email)
                            .service(
                                web::scope("")
                                    .wrap(AuthMiddleware)
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
                            )
                    )
                    .service(
                        web::scope("/dictionary")
                            .wrap(AuthMiddleware)
                            .service(handlers::dictionary::create_entry)
                            .service(handlers::dictionary::get_entry)
                            .service(handlers::dictionary::update_entry)
                            .service(handlers::dictionary::delete_entry)
                            .service(handlers::dictionary::search_entries)
                            .service(handlers::dictionary::list_entries)
                            .service(handlers::dictionary::verify_entry)
                    )
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

    let method_strs: Vec<&str> = cors_settings.allowed_methods.iter().map(|s| s.as_str()).collect();
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

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&settings.level));

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
