use serenity::{
    all::{Colour, CreateEmbed, CreateMessage, Message},
    prelude::Context,
};

pub async fn help(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    let message = CreateMessage::new().add_embed(
        CreateEmbed::new()
            .title("Help - Command List")
            .colour(Colour::from_rgb(255, 255, 0))
            .fields([
                ("$announcement `{userID}` `**{title}**` `{body}`", "Target a special user with a personalised message.", true),
                ("$buzz", "BUZZ!", true),
                ("$clean `{count}`", "Deletes the specified number of messages in the channel you summon me from.", true),
                ("$getMessageData `{target}`", "Shows some info about the users posting habits.", true),
                ("$giveAdmin", "Makes you an administrator of the server.", true),
                ("$beeHealthStatus", "Shows the live status of AdmiralBumbleBee's health.", true),
                ("$help", "Show this again.", true),
                ("$passJenkem `{target}`", "Spreads the love around.", true),
                ("$brewJenkem", "Make new love.", true),
                ("$locateJenkem", "Find the love.", true),
                ("$rejectJenkem", "Break someone's heart.", true),
                ("$jenkemStreak", "View the love streak.", true),
                ("$getPasta `{slug}`", "Show pasta(s)", true),       
                ("$setPasta `{slug trigger:'TRIGGER1' trigger:'TRIGGER2'* chance:(1-100)* mention:(true|false)* payload:'PASTA'}`", "Upsert a new pasta", false),
                ("$delPasta `{slug}`", "Delete a pasta", true),
                ("$kick `{target}` `{reason}`", "Kicks the specified user.", true),
                ("$ban `{target}` `{reason}`", "Bans the specified user.", true),
                ("$mute `{target}` `{reason}`", "Mutes the specified user.", true),
                ("$unmute `{target}` `{reason}`", "Unmutes the specified user.", true),
                ("$roll `{dice}`", "Roll one or more dice with as many sides as you choose, e.g `2d20`", true),
                ("$slap `{target}` `{object}`", "Slap somebody with an object of your choosing.", true),
            ]),
    );

    msg.channel_id.send_message(&ctx.http, message).await?;
    Ok(())
}
