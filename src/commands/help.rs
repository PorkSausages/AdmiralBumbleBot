use serenity::{model::channel::Message, prelude::Context, utils::Color};

pub async fn help(ctx: &Context, msg: &Message) {
    if let Err(e) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Help - Command List");
                e.color(Color::from_rgb(255, 255, 0));
                e.fields(vec![
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
            ]);
                e
            });
            m
        })
        .await
    {
        eprintln!("Error displaying help: {}", e);
    }
}
