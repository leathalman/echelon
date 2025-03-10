#[derive(Debug, Clone)]
pub struct Config {
    pub deployment_url: String,
    pub postgres_url: String,
    pub qdrant_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,
}

impl Config {
    pub fn init() -> Config {
        let deployment_url = std::env::var("DEPLOYMENT_URL").expect("DEPLOYMENT_URL must be set");
        let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");
        Config {
            deployment_url,
            postgres_url,
            qdrant_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),
        }
    }
}