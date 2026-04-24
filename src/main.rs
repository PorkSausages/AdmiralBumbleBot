#![warn(clippy::all, clippy::needless_pass_by_value)]

use {
    crate::storage_models::Scratchpad,
    handler::Handler,
    redb::Database,
    serenity::{prelude::GatewayIntents, Client},
    std::{env, sync::Arc},
};

mod commands;
mod consciousness;
mod handler;
mod logging;
mod pastas;
mod storage;
mod storage_models;
mod util;

const CACHE_SIZE: usize = 500;
const CLEVERBOT_LIMIT: u8 = 10;
const CLEVERBOT_DELAY_SECONDS: u64 = 300;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Can't load .env");

    let mut intents = GatewayIntents::non_privileged();
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents.insert(GatewayIntents::GUILD_MEMBERS);

    let mut client = Client::builder(env::var("ABB_TOKEN").expect("No bot token"), intents)
        .event_handler(Handler {
            db: Arc::new(Database::create("data.redb").expect("Failed to open DB")),
            pad: Scratchpad::new(),
        })
        .await
        .expect("Error creating client");

    client.cache.set_max_messages(CACHE_SIZE);

    if let Err(e) = client.start().await {
        eprintln!("Error starting client: {}", e);
    }
}
