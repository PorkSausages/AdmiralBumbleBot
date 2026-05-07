use {
    super::common,
    crate::logging,
    serenity::{all::GetMessages, model::channel::Message, prelude::Context},
};

pub async fn clean(ctx: &Context, msg: &Message, args: &str) -> Result<(), anyhow::Error> {
    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let author = &msg.author;
    if !common::confirm_admin(ctx, author, guild_id).await? {
        return Ok(());
    };

    let limit = args.parse::<u8>()?;
    let channel_id = msg.channel_id;

    let mut messages = channel_id
        .messages(&ctx.http, GetMessages::new().before(msg.id).limit(limit))
        .await?;

    messages.reverse();
    messages.push(msg.clone());

    channel_id
        .delete_messages(&ctx.http, messages.iter())
        .await?;

    let mut log_text = format!("🧼 {} messages cleaned by <@!{}>!", limit, author.id.get());

    channel_id.say(&ctx.http, &log_text).await?;

    log_text.pop(); //remove the '!'
    log_text.push_str(&format!(" in <#{}>:\n", channel_id));

    let range = 0..messages.len() - 1;
    for i in range {
        let stripped_message = messages[i].content.clone().replace("`", "");
        let author = messages[i].author.clone();

        log_text
            .push_str(&format!("` ┣ `<@!{}>`: {}`\n", author.id.get(), stripped_message))
    }

    let last_message = messages
        .pop()
        .expect("Vec will always have atleast 1 element");
    let stripped_message = last_message.content.replace("`", "");
    let author = last_message.author;

    log_text.push_str(&format!("` ┗ `<@!{}>`: {}`", author.id.get(), stripped_message));

    logging::log(ctx, &log_text).await;
    Ok(())
}
