use serenity::{model::channel::Message, prelude::Context};

use crate::util::get_id_from_env;

pub async fn mute(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    let member = ctx
        .http
        .get_member(guild_id, author.id)
        .await
        .expect("Error getting user");

    msg.channel_id
        .say(&ctx.http, "shut the FUCK")
        .await
        .expect("Error sending message");

    if let Err(e) = member
        .add_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE"))
        .await
    {
        eprintln!("Error muting user: {}", e);
    }
}
