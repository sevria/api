use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "HTTP_ADDRESS", default = "0.0.0.0:4000")]
    pub http_address: String,
}
