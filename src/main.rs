#[macro_use] extern crate log;

mod msg;
mod models;
mod schema;
mod state;
mod tasks;
mod web;

use diesel_async::pooled_connection::{mobc::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;

type PgPool = Pool<AsyncPgConnection>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv()
        .expect("Failed to load .env");
    let database_url = std::env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL environment variable");
    let workers: Vec<url::Url> = vec![
    ];
    let mut pairs: Vec<(url::Url, async_channel::Sender<url::Url>)> = Vec::new();
    for worker in workers {
        let (ws, wr) = async_channel::unbounded::<url::Url>();
        pairs.push((worker.clone(), ws));
        tokio::task::spawn(tasks::worker(worker.clone(), wr.clone()));
    };
    let capture_manager = state::CaptureManager::from_pairs(pairs);
    let state = state::State::new(init_db(database_url), capture_manager);
    web::run("127.0.0.1", 8002, state)
        .await
        .unwrap();
}

fn init_db(db_url: String) -> PgPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::new(config)
}
