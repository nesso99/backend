use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env, default_value_t = 5)]
    pub database_max_connection: u32,
    #[clap(long, env)]
    pub redis_url: String,
}
