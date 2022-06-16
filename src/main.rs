#![warn(clippy::all, clippy::needless_pass_by_value)]

use {
    dotenv::dotenv,
    handler::Handler,
    serenity::{prelude::RwLock, Client},
    std::{collections::HashMap, sync::Arc},
};

#[macro_use]
mod macros;
mod commands;
mod consciousness;
mod handler;
mod logging;
mod pastas;
mod storage;

const STORAGE_PATH: &str = "storage";
const CACHE_SIZE: usize = 500;
const CLEVERBOT_LIMIT: u8 = 10;
const CLEVERBOT_DELAY_SECONDS: u64 = 300;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(get_env!("ABB_TOKEN"))
        .event_handler(Handler {
            storage: sled::open(STORAGE_PATH).expect("Error opening storage database"),
            ignore_list: Arc::new(RwLock::new(HashMap::new())),
        })
        .await
        .expect("Error creating client");

    client
        .cache_and_http
        .cache
        .set_max_messages(CACHE_SIZE)
        .await;

    if let Err(e) = client.start().await {
        eprintln!("Error starting client: {}", e);
    }
}
