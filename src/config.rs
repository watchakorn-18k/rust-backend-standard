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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        assert_eq!(default_port(), 3000);
        assert_eq!(default_mode(), "production");
    }

    #[test]
    fn test_config_extraction() {
        use figment::providers::Serialized;
        let config: AppConfig = Figment::new()
            .merge(Serialized::default("mongodb_uri", "uri"))
            .merge(Serialized::default("mongodb_name", "db"))
            .merge(Serialized::default("redis_host", "localhost"))
            .merge(Serialized::default("redis_port", 6379))
            .merge(Serialized::default("redis_db", 0))
            .merge(Serialized::default("jwt_secret", "secret"))
            .merge(Serialized::default("aws_region", "us-east-1"))
            .merge(Serialized::default("aws_access_key_id", "id"))
            .merge(Serialized::default("aws_secret_access_key", "key"))
            .merge(Serialized::default("aws_bucket_name", "bucket"))
            .merge(Serialized::default("firebase_credentials_file", "file"))
            .extract()
            .unwrap();

        assert_eq!(config.mongodb_uri, "uri");
        assert_eq!(config.port, 3000); // default
    }

    #[test]
    fn test_config_extraction_fail() {
        use figment::providers::Serialized;
        let res: Result<AppConfig, _> = Figment::new()
            .merge(Serialized::default("mongodb_uri", "uri"))
            .extract();
        assert!(res.is_err());
    }

    #[test]
    fn test_config_extraction_full() {
        use figment::providers::Serialized;
        let config: AppConfig = Figment::new()
            .merge(Serialized::default("mongodb_uri", "uri"))
            .merge(Serialized::default("mongodb_name", "db"))
            .merge(Serialized::default("redis_host", "localhost"))
            .merge(Serialized::default("redis_port", 6379))
            .merge(Serialized::default("redis_db", 0))
            .merge(Serialized::default("redis_password", "pass"))
            .merge(Serialized::default("jwt_secret", "secret"))
            .merge(Serialized::default("aws_region", "us-east-1"))
            .merge(Serialized::default("aws_access_key_id", "id"))
            .merge(Serialized::default("aws_secret_access_key", "key"))
            .merge(Serialized::default("aws_bucket_name", "bucket"))
            .merge(Serialized::default("firebase_credentials_file", "file"))
            .merge(Serialized::default("port", 8080))
            .extract()
            .unwrap();

        assert_eq!(config.port, 8080);
        assert_eq!(config.redis_password, Some("pass".to_string()));
    }

    #[test]
    fn test_app_config_new() {
        let _ = AppConfig::new();
    }
}
