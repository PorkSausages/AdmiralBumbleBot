use crate::{
    commands::punish::Punishment,
    storage_models::Scratchpad,
    util::{get_id_from_env, random_string, roll_dice},
};
use redb::Database;
use serenity::model::id::RoleId;

use {
    crate::consciousness,
    regex::Regex,
    serenity::{
        model::{channel::Message, channel::ReactionType, id::EmojiId},
        prelude::Context,
    },
};

mod announcement;
mod bee_sting;
mod buzz;
mod clean;
mod common;
mod get_message_data;
pub mod get_message_quips;
mod give_admin;
mod health;
mod help;
mod jenkem;
mod pasta;
mod punish;
mod roll;
mod slap;

pub async fn execute(
    ctx: &Context,
    msg: &Message,
    db: &Database,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if msg.author.id == get_id_from_env("ABB_BOT_USER_ID")? {
        return Ok(());
    }

    let Some(guild_id) = msg.guild_id else {
        msg.channel_id
            .say(
                ctx,
                random_string(&[
                    "Leave me alone!",
                    "I don't know you like that.",
                    "I think we should just stay friends.",
                    "Let's keep things server-only.",
                    "I'm telling Bee about this.",
                ]),
            )
            .await?;
        return Ok(());
    };

    sonic(ctx, msg).await?;
    pasta::check_pasta(ctx, msg, pad).await?;
    consciousness::consciousness(ctx, msg, pad).await?;

    if !msg.content.starts_with('$') {
        return Ok(());
    }

    let is_booster = msg
        .author
        .has_role(
            &ctx.http,
            guild_id,
            RoleId::new(get_id_from_env("ABB_BOOSTER_ROLE")?),
        )
        .await?;

    if roll_dice("1d20")? == 20
        && msg.channel_id.get() != get_id_from_env("ABB_BOT_TEST_CHANNEL")?
        && !is_booster
    {
        bee_sting::bee_sting(ctx, msg, pad).await?;
        return Ok(());
    }

    let (Some(command), target, arg) = parse_command(&msg.content)? else {
        return Ok(());
    };

    match command.as_str() {
        "$help" => help::help(ctx, msg).await,
        "$buzz" => buzz::buzz(ctx, msg).await,
        "$kick" => punish::punish(ctx, msg, target, arg, Punishment::Kick).await,
        "$ban" => punish::punish(ctx, msg, target, arg, Punishment::Ban).await,
        "$mute" => punish::punish(ctx, msg, target, arg, Punishment::Mute).await,
        "$unmute" => punish::punish(ctx, msg, target, arg, Punishment::Unmute).await,
        "$announcement" => announcement::announcement(ctx, msg, arg).await,
        "$giveAdmin" => give_admin::give_admin(ctx, msg, pad).await,
        "$clean" => clean::clean(ctx, msg, arg).await,
        "$getMessageData" => get_message_data::get_message_data(ctx, msg, target, db).await,
        "$slap" => slap::slap(ctx, msg, target, arg).await,
        "$passJenkem" => jenkem::pass_jenkem(ctx, msg, target, pad).await,
        "$brewJenkem" => jenkem::brew_jenkem(ctx, msg, pad).await,
        "$rejectJenkem" => jenkem::reject_jenkem(ctx, msg, pad).await,
        "$locateJenkem" => jenkem::locate_jenkem(ctx, msg, pad).await,
        "$jenkemStreak" => jenkem::jenkem_streak(ctx, msg, pad).await,
        "$roll" => roll::roll(ctx, msg, arg).await,
        "$beeHealthStatus" => health::health(ctx, msg).await,
        "$getPasta" => pasta::get_pasta(ctx, msg, arg, pad).await,
        "$setPasta" => pasta::set_pasta(ctx, msg, arg, pad).await,
        "$delPasta" => pasta::del_pasta(ctx, msg, arg, pad).await,
        _ => Ok(()),
    }
}

fn parse_command(
    text: &str,
) -> Result<(Option<String>, Option<String>, Option<String>), anyhow::Error> {
    let regexes = vec![
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)> (?P<args>.*)")?,
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)>")?,
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)> (?P<args>.*)")?,
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)>")?,
        Regex::new(r"(?P<command>^\$\w+) (?P<args>.*)")?,
        Regex::new(r"(?P<command>^\$\w+)")?,
    ];

    for re in regexes {
        if re.is_match(text) {
            let caps = re.captures(text).expect("Checked for match");
            let command = caps
                .name("command")
                .map(|command| String::from(command.as_str()));
            let target = caps
                .name("target")
                .map(|target| String::from(target.as_str()));
            let args = caps.name("args").map(|args| String::from(args.as_str()));
            return Ok((command, target, args));
        }
    }

    Ok((None, None, None))
}

async fn sonic(ctx: &Context, msg: &Message) -> Result<(), anyhow::Error> {
    if msg.content.to_ascii_lowercase().contains("sonic")
        || msg.content.to_ascii_lowercase().contains("sanic")
    {
        msg.react(
            &ctx.http,
            ReactionType::Custom {
                id: EmojiId::new(get_id_from_env("ABB_SONIC_EMOTE")?),
                animated: false,
                name: Some(String::from("sonic-1")),
            },
        )
        .await?;
    }
    Ok(())
}
