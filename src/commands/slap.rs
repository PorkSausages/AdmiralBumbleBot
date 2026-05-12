use serenity::{
    all::{CreateAllowedMentions, Message},
    prelude::Context,
};

use crate::commands::common::{get_member_from_user_id, send_clean_message};

pub async fn slap(
    ctx: &Context,
    msg: &Message,
    victim: Option<String>,
    weapon: Option<String>,
) -> Result<(), anyhow::Error> {
    let Some(victim) =
        get_member_from_user_id(ctx, msg, victim, Some("Please specify a victim.")).await?
    else {
        return Ok(());
    };

    if victim.user.id == msg.author.id {
        msg.channel_id
            .say(&ctx.http, "Don't be too hard on yourself.")
            .await?;
        return Ok(());
    }

    let weapon = weapon.unwrap_or("trout".to_string());

    let message_text = if weapon
        .to_ascii_uppercase()
        .starts_with(&['A', 'E', 'I', 'O', 'U'][..])
    {
        format!(
            "*{} slaps {} in the face with an {}!*",
            msg.author.name, victim, weapon
        )
    } else {
        format!(
            "*{} slaps {} in the face with a {}!*",
            msg.author.name, victim, weapon
        )
    };

    send_clean_message(
        ctx,
        msg.channel_id,
        &message_text,
        CreateAllowedMentions::new().users([msg.author.id, victim.user.id]),
    )
    .await?;
    Ok(())
}
