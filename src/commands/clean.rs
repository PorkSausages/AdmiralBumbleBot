use {
    super::common,
    crate::logging,
    serenity::{model::channel::Message, prelude::Context},
};

pub async fn clean(ctx: &Context, msg: &Message, args: &str) {
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    match args.parse::<u64>() {
        Ok(limit) => {
            if common::confirm_admin(ctx, author, guild_id).await
                || d20::roll_dice("2d20").unwrap().total >= 39
            {
                let channel_id = msg.channel_id;

                let mut messages = channel_id
                    .messages(&ctx.http, |retriever| retriever.before(msg.id).limit(limit))
                    .await
                    .expect("Error getting messages to delete");

                messages.reverse();
                messages.push(msg.clone());

                channel_id
                    .delete_messages(&ctx.http, messages.iter())
                    .await
                    .expect("Error deleting messages");

                let mut log_text = format!(
                    "🧼 {} messages cleaned by <@!{}>!",
                    limit,
                    author.id.as_u64()
                );

                channel_id
                    .say(&ctx.http, &log_text)
                    .await
                    .expect("Failed to send message");

                log_text.pop(); //remove the '!'
                log_text.push_str(format!(" in <#{}>:\n", channel_id.0).as_str());

                let range = 0..messages.len() - 1;
                for i in range {
                    let stripped_message = messages[i].content.clone().replace("`", "");
                    let author = messages[i].author.clone();

                    log_text.push_str(
                        format!("` ┣ `<@!{}>`: {}`\n", author.id.as_u64(), stripped_message)
                            .as_str(),
                    )
                }

                let last_message = messages.pop().unwrap();
                let stripped_message = last_message.content.replace("`", "");
                let author = last_message.author;

                log_text.push_str(
                    format!("` ┗ `<@!{}>`: {}`", author.id.as_u64(), stripped_message).as_str(),
                );

                logging::log(ctx, log_text.as_str()).await;
            }
        }
        Err(e) => eprintln!("Error parsing numeric argument: {}", e),
    }
}
