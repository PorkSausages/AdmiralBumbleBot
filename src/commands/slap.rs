use serenity::{
    model::{channel::Message, id::UserId},
    prelude::Context,
};

pub async fn slap(ctx: &Context, msg: &Message, target: &str, args: &str) {
    let slapper = &msg.author.name;
    let slappee = UserId::new(target.parse().expect("Error parsing target"))
        .to_user(&ctx.http)
        .await
        .unwrap()
        .name;

    if args.to_ascii_uppercase().contains("EVERYONE") || args.to_ascii_uppercase().contains("HERE")
    {
        msg.channel_id
            .say(&ctx.http, "do not")
            .await
            .expect("Error sending message");
        return;
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

    msg.channel_id
        .say(&ctx.http, message_text)
        .await
        .expect("Error sending message");
}
