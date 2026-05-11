use serenity::{model::channel::Message, prelude::Context};

use crate::util::roll_dice;

pub async fn roll(ctx: &Context, msg: &Message, args: &str) -> Result<(), anyhow::Error> {
    match roll_dice(args) {
        Ok(result) => {
            msg.channel_id
                .say(&ctx.http, format!("{}!", result))
                .await?
        }
        Err(err) => {
            msg.channel_id
                .say(&ctx.http, format!("Error in dice format! {}", err))
                .await?
        }
    };
    Ok(())
}
