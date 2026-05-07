use {
    super::common,
    crate::{logging, util::get_id_from_env},
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
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
    target: &str,
    args: &str,
    punishment_type: &Punishment,
) -> Result<(), anyhow::Error> {
    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let author = &msg.author;
    let member = match target.parse() {
        Ok(id) => ctx.http.get_member(guild_id, id).await?,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please specify a victim.")
                .await?;
            return Ok(());
        }
    };

    if !common::confirm_admin(ctx, author, guild_id).await? {
        return Ok(());
    }

    let log_text = match punishment_type {
        Punishment::Kick => {
            guild_id
                .kick(&ctx.http, UserId::new(target.parse()?))
                .await?;

            format!(
                "👊 <@!{}> was kicked by <@!{}>:\n` ┗ Reason: {}`",
                target, author.id, args
            )
        }
        Punishment::Ban => {
            guild_id
                .ban(&ctx.http, UserId::new(target.parse()?), BAN_DELETE_DAYS)
                .await?;

            format!(
                "🚫 <@!{}> was banned by <@!{}>:\n` ┗ Reason: {}`",
                target, author.id, args
            )
        }
        Punishment::Mute => {
            member
                .add_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE")?)
                .await?;

            format!(
                "🤐 <@!{}> was muted by <@!{}>:\n` ┗ Reason: {}`",
                target, author.id, args
            )
        }
        Punishment::Unmute => {
            member
                .remove_role(&ctx.http, get_id_from_env("ABB_MUTE_ROLE")?)
                .await?;

            format!("🤐 <@!{}> was unmuted by <@!{}>", target, author.id)
        }
    };

    msg.channel_id.say(&ctx.http, &log_text).await?;
    logging::log(ctx, &log_text).await;

    Ok(())
}
