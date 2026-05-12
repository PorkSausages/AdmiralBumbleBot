use {
    crate::{storage_models::Scratchpad, util::get_id_from_env},
    anyhow::Context as _,
    rand::{thread_rng, Rng},
    serenity::{all::CreateChannel, model::channel::Message, prelude::Context},
};

pub async fn create_dumb_channel(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let channels = pad.with(|pad| pad.dumb_channels.clone());
    let names: Vec<String> = channels.keys().cloned().collect();
    let roll = thread_rng().gen_range(0..names.len());

    let (name, description) = (
        names
            .get(roll - 1)
            .context("Scratchpad doesn't contain random channels")?
            .clone(),
        channels[names
            .get(roll - 1)
            .context("Scratchpad doesn't contain random channels")?]
        .clone(),
    );

    msg.channel_id
        .say(&ctx.http, "Creating a fun new channel!")
        .await?;

    msg.guild_id
        .expect("BumbleBot does not support DMs")
        .create_channel(
            &ctx.http,
            CreateChannel::new(name)
                .topic(description)
                .category(get_id_from_env("ABB_MAIN_CHANNEL_CATEGORY")?),
        )
        .await?;

    Ok(())
}
