use {
    super::common,
    crate::logging,
    crate::storage,
    serenity::{model::channel::Message, prelude::Context},
};

pub async fn give_admin(ctx: &Context, msg: &Message, db: &sled::Db) {
    if !common::in_bot_channel(msg) {
        return;
    }

    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;
    let has_jenkem = storage::locate_jenkem(db) == get_env!("ABB_BOT_USER_ID", u64);
    let dice_roll = d20::roll_dice("2d20").unwrap().total >= 39;

    if dice_roll && !has_jenkem {
        msg.channel_id
            .say(&ctx.http, "Maybe if I had some high quality jenk I'd feel a little more generous...")
            .await
            .expect("Failed to send message");

        return;
    }

    if common::has_wuss_role(ctx, author, guild_id).await {
        msg.channel_id
            .say(&ctx.http, "get fucked nerd")
            .await
            .expect("Error sending message");
            
        return;
    }

    if *author.id.as_u64() == get_env!("ABB_PORKSAUSAGES_ID", u64)
        || (d20::roll_dice("2d20").unwrap().total >= 39 && has_jenkem)
    {
        guild_id
            .member(&ctx.http, author.id)
            .await
            .unwrap()
            .add_role(&ctx.http, get_env!("ABB_ADMIN_ROLE", u64))
            .await
            .expect("Error roling user");

        let log_text = format!("👑 <@!{}> was promoted by me!", author.id);

        msg.channel_id
            .say(&ctx.http, &log_text)
            .await
            .expect("Failed to send message");

        logging::log(ctx, log_text.as_str()).await;
    }
}
