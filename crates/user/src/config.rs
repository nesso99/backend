use clap::Parser;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}
