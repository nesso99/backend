use clap::Parser;
use once_cell::sync::Lazy;

pub static APP_CONFIG: Lazy<Config> = Lazy::new(Config::parse);

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env, default_value_t = 5)]
    pub database_max_connection: u32,
    #[clap(long, env)]
    pub redis_url: String,
    #[clap(long, env)]
    pub jwt_secret: String,
    #[clap(long, env, default_value_t = 86400)]
    pub jwt_ttl_secs: u64,
}
