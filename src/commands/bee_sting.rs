use {
    crate::{storage_models::Scratchpad, util::random_string},
    serenity::{model::channel::Message, prelude::Context},
};

mod create_dumb_channel;
mod kick;
mod mute;

pub async fn bee_sting(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    msg.channel_id.say(&ctx.http, "*Stings you*").await?;

    match random_string(&["channel", "kick", "mute"]).as_str() {
        "channel" => {
            create_dumb_channel::create_dumb_channel(ctx, msg, pad).await
        }
        "kick" => kick::kick(ctx, msg).await,
        "mute" => mute::mute(ctx, msg).await,
        _ => Ok(())
    }
}
