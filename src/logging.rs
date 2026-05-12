use serenity::{
    all::{CreateAllowedMentions, CreateMessage},
    model::id::ChannelId,
    prelude::Context,
};

use crate::util::get_id_from_env;

pub async fn log(ctx: &Context, message: &str) {
    let dis_message = CreateMessage::new()
        .content(message)
        .allowed_mentions(CreateAllowedMentions::new());
    ChannelId::new(get_id_from_env("ABB_LOG_CHANNEL").expect("Log Channel should be set"))
        .send_message(&ctx.http, dis_message)
        .await
        .unwrap_or_else(|_| panic!("Error logging this message: {}", message));
}
