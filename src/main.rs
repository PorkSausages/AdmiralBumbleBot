#![warn(clippy::all, clippy::needless_pass_by_value)]

use {
    crate::{storage_models::Scratchpad},
    handler::Handler,
    redb::Database,
    serenity::{Client, prelude::GatewayIntents},
    std::{env, sync::Arc},
};

mod commands;
mod consciousness;
mod handler;
mod logging;
mod storage;
mod storage_models;
mod util;

const CACHE_SIZE: usize = 500;
const CLEVERBOT_LIMIT: u8 = 10;
const CLEVERBOT_DELAY_SECONDS: u64 = 300;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env should exist");

    let mut intents = GatewayIntents::non_privileged();
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents.insert(GatewayIntents::GUILD_MEMBERS);

    let reset = env::args().skip(1).any(|arg| arg == "--reset-scratchpad");
    let pad = match reset {
        true => {
            let pad = Scratchpad::new(true).expect("Reset scratchpad initialises without issue"); 
            pad.with_mut(|_|{}).expect("Scratchpad should be saved without issue"); 
            pad
        }
        false => {Scratchpad::new(false).expect("Valid scratchpad.json should exist (run with --reset-scratchpad to create a valid one)")}
    };

    let mut client = Client::builder(
        env::var("ABB_TOKEN").expect("Bot token should exist in .env"),
        intents,
    )
    .event_handler(Handler {
        db: Arc::new(Database::create("data.redb").expect("data.redb should exist")),
        pad,
    })
    .await
    .expect("Discord should accept the client");

    client.cache.set_max_messages(CACHE_SIZE);
    println!("Sonic");
    client.start().await.expect("Client should be started")
}
