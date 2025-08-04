use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "CORS_ALLOW_ORIGIN", default = "*")]
    pub cors_allow_origin: String,

    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "DEFAULT_ADMIN_EMAIL", default = "admin@example.com")]
    pub default_admin_email: String,

    #[envconfig(from = "DEFAULT_ADMIN_PASSWORD", default = "Sevria123")]
    pub default_admin_password: String,

    #[envconfig(from = "HTTP_ADDRESS", default = "0.0.0.0:4000")]
    pub http_address: String,

    #[envconfig(from = "JWT_EXPIRES_IN_MINUTES", default = "15")]
    pub jwt_expires_in_minutes: i64,

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
}
