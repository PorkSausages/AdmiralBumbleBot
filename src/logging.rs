use serenity::{
    all::{CreateAllowedMentions, CreateMessage},
    model::id::ChannelId,
    prelude::Context,
};

use crate::util::get_id_from_env;

pub async fn log(ctx: &Context, message: &str) {
    let message = CreateMessage::new()
        .content(message)
        .allowed_mentions(CreateAllowedMentions::new().empty_users());
    ChannelId::new(get_id_from_env("ABB_LOG_CHANNEL"))
        .send_message(&ctx.http, message)
        .await
        .expect("Error logging message.");
}
