use serenity::{
    model::{
        channel::Message,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

pub async fn confirm_admin(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user
        .has_role(&ctx.http, guild, RoleId(get_env!("ABB_ADMIN_ROLE", u64)))
        .await
    {
        Ok(b) => b || user.id == get_env!("ABB_USER_ID", u64),
        Err(e) => {
            eprintln!("Error authenticating user: {}", e);
            false
        }
    }
}

pub fn in_bot_channel(msg: &Message) -> bool {
    if msg.channel_id.0 == get_env!("ABB_BOT_CHANNEL", u64)
        || msg.channel_id.0 == get_env!("ABB_BOT_TEST_CHANNEL", u64)
    {
        return true;
    }
    false
}

pub async fn has_wuss_role(ctx: &Context, user: &User, guild: GuildId) -> bool {
    match user
        .has_role(&ctx.http, guild, RoleId(get_env!("ABB_WUSS_ROLE", u64)))
        .await
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error checking role status: {}", e);
            true
        }
    }
}
