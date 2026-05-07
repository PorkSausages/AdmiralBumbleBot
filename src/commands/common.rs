use serenity::{
    model::{
        channel::Message,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

use crate::util::{get_id_from_env, roll_dice};

pub async fn confirm_admin(
    ctx: &Context,
    user: &User,
    guild: GuildId,
) -> Result<bool, anyhow::Error> {
    let lucky = roll_dice("2d20")? >= 39;
    Ok(user
        .has_role(
            &ctx.http,
            guild,
            RoleId::new(get_id_from_env("ABB_ADMIN_ROLE")?),
        )
        .await?
        || user.id == RoleId::new(get_id_from_env("ABB_USER_ID")?).get()
        || lucky)
}

pub fn in_bot_channel(msg: &Message) -> Result<bool, anyhow::Error> {
    if msg.channel_id.get() == get_id_from_env("ABB_BOT_CHANNEL")?
        || msg.channel_id.get() == get_id_from_env("ABB_BOT_TEST_CHANNEL")?
    {
        return Ok(true);
    }
    Ok(false)
}

pub async fn has_wuss_role(
    ctx: &Context,
    user: &User,
    guild: GuildId,
) -> Result<bool, anyhow::Error> {
    Ok(user
        .has_role(
            &ctx.http,
            guild,
            RoleId::new(get_id_from_env("ABB_WUSS_ROLE")?),
        )
        .await?)
}
