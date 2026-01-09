use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_ttl_seconds: u64,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub smtp_from_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/reverse_gantt".into());

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".into());
        let jwt_ttl_seconds = env::var("JWT_TTL_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(86400);

        let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".into());
        let smtp_port = env::var("SMTP_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(587);
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_default();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
        let smtp_from_email = env::var("SMTP_FROM_EMAIL").unwrap_or_else(|_| "noreply@example.com".into());
        let smtp_from_name = env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "Reverse Gantt".into());

        Self {
            host,
            port,
            database_url,
            jwt_secret,
            jwt_ttl_seconds,
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from_email,
            smtp_from_name,
        }
    }
}
