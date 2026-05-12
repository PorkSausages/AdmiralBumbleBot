use serenity::{
    all::{ChannelId, CreateAllowedMentions, CreateMessage, Member, Message},
    model::id::RoleId,
    prelude::Context,
};

use crate::util::{get_id_from_env, roll_dice};

pub async fn confirm_admin(ctx: &Context, msg: &Message) -> Result<bool, anyhow::Error> {
    let lucky = roll_dice("2d20")? >= 39;
    Ok(msg
        .author
        .has_role(
            &ctx.http,
            msg.guild_id.expect("BumbleBot does not support DMs"),
            RoleId::new(get_id_from_env("ABB_ADMIN_ROLE")?),
        )
        .await?
        || msg.author.id == RoleId::new(get_id_from_env("ABB_USER_ID")?).get()
        || lucky)
}

pub fn in_bot_channel(channel_id: u64) -> Result<bool, anyhow::Error> {
    if channel_id == get_id_from_env("ABB_BOT")?
        || channel_id == get_id_from_env("ABB_BOT_TEST_CHANNEL")?
    {
        return Ok(true);
    }
    Ok(false)
}

pub async fn get_member_from_user_id(
    ctx: &Context,
    msg: &Message,
    id: Option<String>,
    fail_message: Option<&str>,
) -> Result<Option<Member>, anyhow::Error> {
    let Some(victim) = (match id.and_then(|s| s.parse::<u64>().ok()) {
        Some(id) => Some(
            msg.guild_id
                .expect("BumbleBot does not support DMs")
                .member(&ctx.http, id)
                .await?,
        ),
        None => None,
    }) else {
        if let Some(fail_message) = fail_message {
            send_clean_message(
                ctx,
                msg.channel_id,
                fail_message,
                CreateAllowedMentions::new(),
            )
            .await?
        }
        return Ok(None);
    };
    Ok(Some(victim))
}

pub async fn send_clean_message(
    ctx: &Context,
    channel_id: ChannelId,
    content: &str,
    allowed: CreateAllowedMentions,
) -> Result<(), anyhow::Error> {
    let clean_msg = CreateMessage::new()
        .content(content)
        .allowed_mentions(allowed);
    channel_id.send_message(&ctx.http, clean_msg).await?;
    Ok(())
}

pub async fn has_wuss_role(ctx: &Context, msg: &Message) -> Result<bool, anyhow::Error> {
    Ok(msg
        .author
        .has_role(
            &ctx.http,
            msg.guild_id.expect("BumbleBot does not support DMs"),
            RoleId::new(get_id_from_env("ABB_WUSS_ROLE")?),
        )
        .await?)
}
