use serenity::{model::channel::Message, prelude::Context};

pub async fn kick(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    msg.channel_id.say(&ctx.http, "Begone!").await?;

    msg.guild_id
        .expect("BumbleBot does not support DMs")
        .kick(&ctx.http, &msg.author)
        .await?;

    Ok(())
}
