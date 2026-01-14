use serenity::{
    all::{Colour, CreateEmbed, CreateMessage},
    model::channel::Message,
    prelude::Context,
};

pub async fn help(ctx: &Context, msg: &Message) {
    let message = CreateMessage::new().add_embed(
        CreateEmbed::new()
            .title("Help - Command List")
            .colour(Colour::from_rgb(255, 255, 0))
            .fields([
                ("$help", "Show this again.", true),
                ("$buzz", "BUZZ!", true),
                (
                    "$kick `{target}` `{reason}`",
                    "Kicks the specified user.",
                    true,
                ),
                (
                    "$ban `{target}` `{reason}`",
                    "Bans the specified user.",
                    true,
                ),
                (
                    "$mute `{target}` `{reason}`",
                    "Mutes the specified user.",
                    true,
                ),
                (
                    "$unmute `{target}` `{reason}`",
                    "Unmutes the specified user.",
                    true,
                ),
                (
                    "$announcement `{userID}` `**{title}**` `{body}`",
                    "Target a special user with a personalised message.",
                    true,
                ),
                (
                    "$giveAdmin",
                    "Makes you an administrator of the server.",
                    true,
                ),
                (
                    "$clean `{count}`",
                    "Deletes the specified number of messages in the channel you summon me from.",
                    true,
                ),
                (
                    "$getMessageData `{target}`",
                    "Shows some info about the users posting habits.",
                    true,
                ),
                (
                    "$slap `{target}` `{object}`",
                    "Slap somebody with an object of your choosing.",
                    true,
                ),
                (
                    "$roll `{dice}`",
                    "Roll one or more dice with as many sides as you choose, e.g `2d20`",
                    true,
                ),
            ]),
    );

    if let Err(e) = msg.channel_id.send_message(&ctx.http, message).await {
        eprintln!("Error displaying help: {}", e);
    }
}
