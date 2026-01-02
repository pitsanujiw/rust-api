use std::env;

use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub app_env: AppEnv,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEnv {
    Local,
    Dev,
    Prod,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set".to_string())?;

        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

        let app_env = AppEnv::from_str(&app_env)?;

        Ok(Self { db_url, app_env })
    }
}

impl AppEnv {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "local" => Ok(AppEnv::Local),
            "dev" | "development" => Ok(AppEnv::Dev),
            "prod" | "production" => Ok(AppEnv::Prod),
            _ => Err(format!("invalid APP_ENV: {}", s)),
        }
    }

    pub fn is_prod(&self) -> bool {
        matches!(self, AppEnv::Prod)
    }
}
