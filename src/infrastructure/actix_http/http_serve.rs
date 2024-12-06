use actix_cors::Cors;
use actix_web::{error, http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use std::{sync::Arc, time::Duration};

use crate::{
    config::config_model::DotEnvyConfig, infrastructure::postgres::postgres_connector::PgPoolSquad,
};

use anyhow::Result;
use tracing::info;
use tracing_actix_web::TracingLogger;

// use super::default_routers;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    // let origins = std::env::var("ORIGINS").expect("ORIGINS must be set");

    let config_clone = Arc::clone(&config);

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config_clone.server.origin)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        let json_config = web::JsonConfig::default()
            .limit((config_clone.server.body_limit * 1024 * 1024).try_into().unwrap())
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .app_data(web::Data::new(Arc::clone(&db_pool)))
            .app_data(json_config)
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
        // .route(
        //     "/health-check",
        //     web::get().to(default_routers::health_check),
        // )
    })
    .workers(4)
    .client_request_timeout(Duration::from_secs(Arc::clone(&config).server.timeout))
    .bind(("0.0.0.0", Arc::clone(&config).server.port))
    .expect("Failed to bind server")
    .run()
    .await;

    info!("Server running on port {}", Arc::clone(&config).server.port);

    Ok(())
}