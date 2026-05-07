use serenity::{
    model::{channel::Message, id::UserId},
    prelude::Context,
};

pub async fn slap(
    ctx: &Context,
    msg: &Message,
    target: &str,
    args: &str,
) -> Result<(), anyhow::Error> {
    let slapper = &msg.author.name;
    let slappee = match target.parse() {
        Ok(id) => UserId::new(id),
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please specify a victim.")
                .await?;
            return Ok(());
        }
    };

    if args.to_ascii_uppercase().contains("EVERYONE") || args.to_ascii_uppercase().contains("HERE")
    {
        msg.channel_id.say(&ctx.http, "do not").await?;
        return Ok(());
    }

    let message_text = if args
        .to_ascii_uppercase()
        .starts_with(&['A', 'E', 'I', 'O', 'U'][..])
    {
        format!(
            "*{} slaps {} in the face with an {}!*",
            slapper, slappee, args
        )
    } else {
        format!(
            "*{} slaps {} in the face with a {}!*",
            slapper, slappee, args
        )
    };

    msg.channel_id.say(&ctx.http, message_text).await?;

    Ok(())
}
