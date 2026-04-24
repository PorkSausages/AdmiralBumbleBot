use {
    crate::{storage_models::Scratchpad, util::get_id_from_env},
    rand::{Rng, thread_rng},
    serenity::{all::CreateChannel, model::channel::Message, prelude::Context},
};

pub async fn create_dumb_channel(ctx: &Context, msg: &Message, pad: &Scratchpad) {
    let channels = pad.with(|pad| {pad.dumb_channels.clone()});
    let names: Vec<String> = channels.keys().cloned().collect();
    let roll = thread_rng().gen_range(0..names.len());

    let (name, description) = (
        names[roll as usize - 1].clone(),
        channels[names[roll as usize - 1].as_str()].clone(),
    );

    msg.channel_id
        .say(&ctx.http, "Creating a fun new channel!")
        .await
        .expect("Error sending message");

    msg.guild_id
        .expect("Error getting guild ID")
        .create_channel(
            &ctx.http,
            CreateChannel::new(name)
                .topic(description)
                .category(get_id_from_env("ABB_MAIN_CHANNEL_CATEGORY")),
        )
        .await
        .expect("Error creating channel");
}