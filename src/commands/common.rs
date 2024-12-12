use serenity::{
    model::{
        channel::Message,
        guild::Member,
        id::{GuildId, RoleId},
        prelude::User,
    },
    prelude::Context,
};

use rand::Rng;

const MEMBER_LIMIT: u64 = 1000;

pub async fn random_user(ctx: &Context, guild_id: &GuildId) -> Member {
    let members: Vec<Member> = guild_id
        .members(&ctx.http, Some(MEMBER_LIMIT), None)
        .await
        .unwrap();

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0, members.len());

    members[random_index].clone()
}

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
