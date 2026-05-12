use {
    super::common,
    crate::logging,
    serenity::{
        all::{GetMessages, Message},
        prelude::Context,
    },
};

pub async fn clean(
    ctx: &Context,
    msg: &Message,
    limit: Option<String>,
) -> Result<(), anyhow::Error> {
    if !common::confirm_admin(ctx, msg).await? {
        return Ok(());
    };

    let Some(limit) = limit.and_then(|l| l.parse::<u8>().ok()) else {
        msg.channel_id
            .say(&ctx.http, "Please include a message count.")
            .await?;
        return Ok(());
    };
    let mut messages = msg
        .channel_id
        .messages(&ctx.http, GetMessages::new().before(msg.id).limit(limit))
        .await?;

    messages.reverse();
    messages.push(msg.clone());

    msg.channel_id
        .delete_messages(&ctx.http, messages.iter())
        .await?;

    let mut log_text = format!(
        "🧼 {} messages cleaned by <@!{}>!",
        limit,
        msg.author.id.get()
    );

    msg.channel_id.say(&ctx.http, &log_text).await?;

    log_text.pop(); //remove the '!'
    log_text.push_str(&format!(" in <#{}>:\n", msg.channel_id));

    let range = 0..messages.len() - 1;
    for i in range {
        let stripped_message = messages[i].content.clone().replace("`", "");
        let author = messages[i].author.clone();

        log_text.push_str(&format!(
            "` ┣ `<@!{}>`: {}`\n",
            author.id.get(),
            stripped_message
        ))
    }

    let last_message = messages
        .pop()
        .expect("Vec will always have atleast 1 element");
    let stripped_message = last_message.content.replace("`", "");
    let author = last_message.author;

    log_text.push_str(&format!(
        "` ┗ `<@!{}>`: {}`",
        author.id.get(),
        stripped_message
    ));

    logging::log(ctx, &log_text).await;
    Ok(())
}
