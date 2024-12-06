use anyhow::Result;

use super::{
    config_model::{DotEnvyConfig, JwtsSecret},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = super::config_model::Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
        origin: std::env::var("ORIGIN").expect("ORIGIN is invalid").parse()?,
    };

    let database = super::config_model::Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_jwt_secret_env() -> Result<JwtsSecret> {
    dotenvy::dotenv().ok();

    Ok(JwtsSecret {
        secret: std::env::var("JWT_SECRET")?,
        refresh_secret: std::env::var("JWT_REFRESH_SECRET")?,
    })
}
