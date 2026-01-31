use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Env::prefixed("APP_"))
            .merge(Toml::file("App.toml"))
            .join(Env::raw()) // Load raw env vars as well if needed
            .extract()
    }
}
