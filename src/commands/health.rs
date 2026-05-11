use serenity::{model::channel::Message, prelude::Context};

pub async fn health(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    msg.channel_id.say(&ctx.http, "bad").await?;
    Ok(())
}
