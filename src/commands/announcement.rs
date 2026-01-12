use {
    super::common,
    regex::Regex,
    serenity::{
        model::{channel::Message, id::ChannelId},
        prelude::Context,
        utils::Color,
    },
};

pub async fn announcement(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let author = &msg.author;

    let (id, title, body) = match parse_announcement_message(msg.content.as_str()) {
        Some(some) => some,
        None => return,
    };

    if common::confirm_admin(ctx, author, guild_id).await
        || d20::roll_dice("2d20").unwrap().total >= 39
    {
        if let Err(e) = ChannelId(get_env!("ABB_ANNOUNCEMENT_CHANNEL", u64))
            .send_message(&ctx.http, |m| {
                m.tts(true);
                m.content(format!("Hey, <@!{}>! Yes, you!", id));
                m.embed(|e| {
                    e.title(title);
                    e.description(body);
                    e.color(Color::from_rgb(255, 255, 0));
                    e
                });
                m
            })
            .await
        {
            eprintln!("Error sending announcement: {}", e);
        }
    }
}

fn parse_announcement_message(message: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r"(?P<user_id>\d+) \*\*(?P<title>.*?)\*\* (?P<body>.*)").unwrap();

    if !re.is_match(message) {
        return None;
    }

    let caps = re.captures(message).unwrap();
    let (user_id, title, body) = (
        caps.name("user_id")
            .expect("Error parsing user ID")
            .as_str(),
        caps.name("title")
            .expect("Error parsing announcement title")
            .as_str(),
        caps.name("body")
            .expect("Error parsing announcement body")
            .as_str(),
    );

    Some((
        String::from(user_id),
        String::from(title),
        String::from(body),
    ))
}
