use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub mongodb_uri: String,
    pub mongodb_name: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: Option<String>,
    pub redis_db: i64,
    pub jwt_secret: String,
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_bucket_name: String,
    pub firebase_credentials_file: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_mode")]
    pub app_mode: String,
}

fn default_port() -> u16 {
    3000
}

fn default_mode() -> String {
    "production".to_string()
}

impl AppConfig {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("App.toml"))
            .merge(Env::raw())
            .extract()
    }
}
