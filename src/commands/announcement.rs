use {
    super::common,
    crate::util::get_id_from_env,
    regex::Regex,
    serenity::{
        all::{Colour, CreateAllowedMentions, CreateEmbed, CreateMessage, Message, UserId},
        model::id::ChannelId,
        prelude::Context,
    },
};

pub async fn announcement(
    ctx: &Context,
    msg: &Message,
    command: Option<String>,
) -> Result<(), anyhow::Error> {
    if !common::confirm_admin(ctx, msg).await? {
        return Ok(());
    }

    let (victim_id, title, body) = match parse_announcement_command(&command) {
        Some(some) => some,
        None => {
            msg.channel_id
                .say(&ctx.http, "Syntax: `$announcement userId **Title** body`")
                .await?;
            return Ok(());
        }
    };

    ChannelId::new(get_id_from_env("ABB_ANNOUNCEMENT_CHANNEL")?)
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!("Hey, <@!{}>! Yes, you!", victim_id))
                .allowed_mentions(CreateAllowedMentions::new().users([UserId::from(victim_id)]))
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

fn parse_announcement_command(command: &Option<String>) -> Option<(u64, String, String)> {
    let re = Regex::new(r"(?P<user_id>\d+) \*\*(?P<title>.*?)\*\* (?P<body>.*)")
        .expect("Valid regex (I checked)");
    let caps = re.captures(command.as_deref()?)?;
    let user_id = &caps["user_id"];
    let title = &caps["title"];
    let body = &caps["body"];

    Some((
        String::from(user_id).parse::<u64>().ok()?,
        String::from(title),
        String::from(body),
    ))
}
