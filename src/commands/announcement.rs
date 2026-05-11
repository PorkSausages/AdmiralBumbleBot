use {
    super::common,
    crate::util::get_id_from_env,
    regex::Regex,
    serenity::{
        all::{Colour, CreateEmbed, CreateMessage},
        model::{channel::Message, id::ChannelId},
        prelude::Context,
    },
};

pub async fn announcement(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let author = &msg.author;

    let (id, title, body) = match parse_announcement_message(&msg.content) {
        Some(some) => some,
        None => {
            msg.channel_id
                .say(&ctx.http, "Syntax: `$announcement userId **Title** body`")
                .await?;
            return Ok(());
        }
    };

    if !common::confirm_admin(ctx, author, guild_id).await? {
        return Ok(());
    }

    ChannelId::new(get_id_from_env("ABB_ANNOUNCEMENT_CHANNEL")?)
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!("Hey, <@!{}>! Yes, you!", id))
                .add_embed(
                    CreateEmbed::new()
                        .title(title)
                        .description(body)
                        .colour(Colour::from_rgb(255, 255, 0)),
                ),
        )
        .await?;
    Ok(())
}

fn parse_announcement_message(message: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r"(?P<user_id>\d+) \*\*(?P<title>.*?)\*\* (?P<body>.*)")
        .expect("Valid regex (I checked)");
    let caps = re.captures(message)?;
    let user_id = &caps["user_id"];
    let title = &caps["title"];
    let body = &caps["body"];

    Some((
        String::from(user_id),
        String::from(title),
        String::from(body),
    ))
}
