#[macro_use] extern crate log;

mod config;
mod extract;
mod models;
mod schema;
mod state;
mod web;

use diesel_async::pooled_connection::{mobc::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;

type PgPool = Pool<AsyncPgConnection>;

#[tokio::main]
async fn main() {
    // Setup
    pretty_env_logger::init();
    dotenvy::dotenv()
        .expect("Failed to load .env");
    let config = match config::Config::from_env("WEBROLL_SERVER_CONFIG") {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    let database_url = std::env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL env var");
    let state = state::State::new(init_db(database_url), config.workers());
    web::run(config.listen_ip(), config.listen_port(), state)
        .await
        .unwrap();
}

fn init_db(db_url: String) -> PgPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::new(config)
}
