use serenity::{
    all::Message, prelude::Context
};

use crate::util::get_member_from_user_id;

pub async fn slap(
    ctx: &Context,
    msg: &Message,
    victim: Option<String>,
    weapon: Option<String>,
) -> Result<(), anyhow::Error> {

    let slapper = &msg.author.name;
    let Some(victim) = get_member_from_user_id(ctx, msg, victim, Some("Please specify a victim.")).await? else {
        return Ok(());
    };

    let weapon = weapon.unwrap_or("trout".to_string());

    let message_text = if weapon
        .to_ascii_uppercase()
        .starts_with(&['A', 'E', 'I', 'O', 'U'][..])
    {
        format!(
            "*{} slaps {} in the face with an {}!*",
            slapper, victim, weapon
        )
    } else {
        format!(
            "*{} slaps {} in the face with a {}!*",
            slapper, victim, weapon
        )
    };

    msg.channel_id.say(&ctx.http, message_text).await?;
    Ok(())
}
