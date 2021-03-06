mod bot;
mod config;
mod database;

use crate::config::Config;
use simple_logger::SimpleLogger;

use log::{info, LevelFilter};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    info!("Starting...");

    let config = Config::new(String::from("./config.yml"));
    bot::start(config).await;
}
