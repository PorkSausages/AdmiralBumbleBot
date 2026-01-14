use serenity::{
    model::{
        channel::Message,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

use crate::util::get_id_from_env;

pub async fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user
        .has_role(
            &ctx.http,
            guild,
            RoleId::new(get_id_from_env("ABB_ADMIN_ROLE")),
        )
        .await
    {
        Ok(b) => b || user.id == RoleId::new(get_id_from_env("ABB_USER_ID")).get(),
        Err(e) => {
            eprintln!("Error authenticating user: {}", e);
            false
        }
    }
}

pub fn in_bot_channel(msg: &Message) -> bool {
    if msg.channel_id.get() == get_id_from_env("ABB_BOT_CHANNEL")
        || msg.channel_id.get() == get_id_from_env("ABB_BOT_TEST_CHANNEL")
    {
        return true;
    }
    false
}

pub async fn has_wuss_role(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user
        .has_role(
            &ctx.http,
            guild,
            RoleId::new(get_id_from_env("ABB_WUSS_ROLE")),
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error checking role status: {}", e);
            true
        }
    }
}
