use std::env::VarError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" => Environment::Production,
            _ => Environment::Development, // Fallback to Development for any unrecognized value
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub deployment_url: String,
    pub postgres_url: String,
    pub qdrant_url: String,
    // not required for development environments
    pub qdrant_api_key: Result<String, VarError>,
    pub jwt_secret: String,
    pub jwt_expired_in: i64,
    pub jwt_max_age: i64,
    pub environment: Environment,
}

impl Config {
    pub fn init() -> Config {
        // RUST_ENV should match 'development' or 'production', case-sensitive
        let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        let env_file = format!(".env.{}", env);

        dotenv::from_filename(env_file).ok();

        let deployment_url = std::env::var("DEPLOYMENT_URL").expect("DEPLOYMENT_URL must be set");
        let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
        let qdrant_api_key = std::env::var("QDRANT_API_KEY");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expired_in = std::env::var("JWT_EXPIRED_IN")
            .expect("JWT_EXPIRED_IN must be set").parse::<i64>()
            .expect("Could not parse JWT_EXPIRED_IN as i64");
        let jwt_max_age = std::env::var("JWT_MAX_AGE")
            .expect("JWT_MAX_AGE must be set").parse::<i64>()
            .expect("Could not parse JWT_MAX_AGE as i64");

        Config {
            deployment_url,
            postgres_url,
            qdrant_url,
            qdrant_api_key,
            jwt_secret,
            jwt_expired_in,
            jwt_max_age,
            environment: Environment::from_string(&env),
        }
    }
}