use serenity::{model::channel::Message, prelude::Context};

use crate::util::get_id_from_env;

pub async fn mute(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let author = &msg.author;

    let member = ctx.http.get_member(guild_id, author.id).await?;

    msg.channel_id.say(&ctx.http, "shut the FUCK").await?;

    member
        .add_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE")?)
        .await?;

    Ok(())
}
