use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub environment: String,
    pub supabase_url: String,
    pub supabase_jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            supabase_url: env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "".to_string()),
            supabase_jwt_secret: env::var("SUPABASE_JWT_SECRET")
                .expect("SUPABASE_JWT_SECRET must be set for authentication"),
        }
    }
}
