use anyhow::Context as _;
use serenity::{model::channel::Message, prelude::Context};

use crate::util::roll_dice;

pub async fn roll(ctx: &Context, msg: &Message, roll: Option<String>) -> Result<(), anyhow::Error> {
    match roll.context("Empty string").and_then(|s| roll_dice(&s)) {
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
