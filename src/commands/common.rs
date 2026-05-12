use serenity::{all::Message, model::id::RoleId, prelude::Context};

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
