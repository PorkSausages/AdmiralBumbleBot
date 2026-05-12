use anyhow::Context as _;
use serenity::{all::CreateAllowedMentions, model::channel::Message, prelude::Context};

use crate::{commands::common::send_clean_message, util::roll_dice};

pub async fn roll(ctx: &Context, msg: &Message, roll: Option<String>) -> Result<(), anyhow::Error> {
    match roll.context("Empty string").and_then(|s| roll_dice(&s)) {
        Ok(result) => {
            msg.channel_id
                .say(&ctx.http, format!("{}!", result))
                .await?;
        }
        Err(err) => {
            send_clean_message(
                ctx,
                msg.channel_id,
                &format!("Error in dice format! {}", err),
                CreateAllowedMentions::new(),
            )
            .await?;
        }
    };
    Ok(())
}
