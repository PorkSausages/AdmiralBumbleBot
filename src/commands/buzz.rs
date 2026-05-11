use serenity::{model::channel::Message, prelude::Context};

pub async fn buzz(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    msg.channel_id.say(&ctx.http, "BUZZ!").await?;
    Ok(())
}
