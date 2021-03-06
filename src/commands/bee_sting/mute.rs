use serenity::{model::channel::Message, prelude::Context};

pub async fn mute(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    let mut member = ctx
        .http
        .get_member(*guild_id.as_u64(), *author.id.as_u64())
        .await
        .expect("Error getting user");

    msg.channel_id
        .say(&ctx.http, "shut the FUCK")
        .await
        .expect("Error sending message");

    if let Err(e) = member
        .add_role(&ctx.http, get_env!("ABB_MUTE_ROLE", u64))
        .await
    {
        eprintln!("Error muting user: {}", e);
    }
}
