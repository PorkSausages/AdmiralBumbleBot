use {
    super::common,
    crate::{
        logging,
        util::{get_id_from_env, get_member_from_user_id, random_string},
    },
    serenity::{all::Message, prelude::Context},
};

const BAN_DELETE_DAYS: u8 = 0;

pub enum Punishment {
    Kick,
    Ban,
    Mute,
    Unmute,
}

pub async fn punish(
    ctx: &Context,
    msg: &Message,
    victim_id: Option<String>,
    reason: Option<String>,
    punishment_type: &Punishment,
) -> Result<(), anyhow::Error> {
    if !common::confirm_admin(ctx, &msg).await? {
        return Ok(());
    }

    let Some(victim) =
        get_member_from_user_id(ctx, msg, victim_id, Some("Please specify a victim")).await?
    else {
        return Ok(());
    };

    let (guild_id, author) = (
        msg.guild_id.expect("BumbleBot does not support DMs"),
        &msg.author,
    );

    let reason = reason.unwrap_or(random_string(&[
        "(they just felt like it)",
        "(they deserved it)",
        "(did they really need a reason?)",
        "(come on, it's obvious)",
    ]));

    let log_text = match punishment_type {
        Punishment::Kick => {
            guild_id.kick(&ctx.http, &victim).await?;

            format!(
                "👊 <@!{}> was kicked by <@!{}>:\n` ┗ Reason: {}`",
                &victim.user.id, author.id, reason
            )
        }
        Punishment::Ban => {
            guild_id.ban(&ctx.http, &victim, BAN_DELETE_DAYS).await?;

            format!(
                "🚫 <@!{}> was banned by <@!{}>:\n` ┗ Reason: {}`",
                &victim.user.id, author.id, reason
            )
        }
        Punishment::Mute => {
            victim
                .add_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE")?)
                .await?;

            format!(
                "🤐 <@!{}> was muted by <@!{}>:\n` ┗ Reason: {}`",
                &victim.user.id, author.id, reason
            )
        }
        Punishment::Unmute => {
            victim
                .remove_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE")?)
                .await?;

            format!(
                "🤐 <@!{}> was unmuted by <@!{}>",
                &victim.user.id, author.id
            )
        }
    };

    msg.channel_id.say(&ctx.http, &log_text).await?;
    logging::log(ctx, &log_text).await;

    Ok(())
}
