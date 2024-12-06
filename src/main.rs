use std::sync::Arc;

use tracing::{error, info};

use rust_actix::{
    config::config_loader,
    infrastructure::{
        postgres::postgres_connector,
        actix_http::http_serve::start,
    }
};


#[actix_web::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    print!("{:?}", dotenvy_env);

    let postgres_pool = match postgres_connector::establish_connection(&dotenvy_env.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Postgres: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection has been established");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server");
}