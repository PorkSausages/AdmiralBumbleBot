use serenity::{model::channel::Message, prelude::Context};

use crate::util::roll_dice;

pub async fn roll(ctx: &Context, msg: &Message, args: &str) {
    let result = roll_dice(args).unwrap_or(0);

    if result == 0 {
        msg.channel_id
            .say(&ctx.http, "Error in dice format!")
            .await
            .expect("Error sending message");

        return;
    }

    msg.channel_id
        .say(&ctx.http, format!("{}!", result))
        .await
        .expect("Error sending message");
}
