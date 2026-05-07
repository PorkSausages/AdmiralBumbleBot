use crate::{
    storage_models::Scratchpad,
    util::{get_id_from_env, roll_dice},
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

    sonic(ctx, msg).await?;
    pasta::check_pasta(ctx, msg, pad).await?;
    consciousness::consciousness(ctx, msg, pad).await?;

    if !msg.content.starts_with('$') {
        return Ok(());
    }

    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let is_booster = msg
        .author
        .has_role(
            &ctx.http,
            guild_id,
            RoleId::new(get_id_from_env("ABB_BOOSTER_ROLE")?),
        )
        .await?;

    let (command, target, args) = match parse_command(&msg.content)? {
        Some(result) => result,
        None => return Ok(()),
    };

    if roll_dice("1d20")? == 20
        && msg.channel_id.get() != get_id_from_env("ABB_BOT_TEST_CHANNEL")?
        && !is_booster
    {
        bee_sting::bee_sting(ctx, msg, pad).await?;
        return Ok(());
    }

    match command.as_str() {
        "$help" => help::help(ctx, msg).await,
        "$buzz" => buzz::buzz(ctx, msg).await,
        "$kick" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Kick).await,
        "$ban" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Ban).await,
        "$mute" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Mute).await,
        "$unmute" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Unmute).await,
        "$announcement" => announcement::announcement(ctx, msg).await,
        "$giveAdmin" => give_admin::give_admin(ctx, msg, pad).await,
        "$clean" => clean::clean(ctx, msg, &args).await,
        "$getMessageData" => get_message_data::get_message_data(ctx, msg, &target, db).await,
        "$slap" => slap::slap(ctx, msg, &target, &args).await,
        "$passJenkem" => jenkem::pass_jenkem(ctx, msg, &target, pad).await,
        "$brewJenkem" => jenkem::brew_jenkem(ctx, msg, pad).await,
        "$rejectJenkem" => jenkem::reject_jenkem(ctx, msg, pad).await,
        "$locateJenkem" => jenkem::locate_jenkem(ctx, msg, pad).await,
        "$jenkemStreak" => jenkem::jenkem_streak(ctx, msg, pad).await,
        "$roll" => roll::roll(ctx, msg, &args).await,
        "$beeHealthStatus" => health::health(ctx, msg).await,
        "$getPasta" => pasta::get_pasta(ctx, msg, pad, &args).await,
        "$setPasta" => pasta::set_pasta(ctx, msg, pad, &args).await,
        "$delPasta" => pasta::del_pasta(ctx, msg, pad, &args).await,
        _ => Ok(()),
    }
}

fn parse_command(text: &str) -> Result<Option<(String, String, String)>, anyhow::Error> {
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

            let command = match caps.name("command") {
                Some(command) => String::from(command.as_str()),
                None => String::new(),
            };

            let target = match caps.name("target") {
                Some(target) => String::from(target.as_str()),
                None => String::new(),
            };

            let args = match caps.name("args") {
                Some(args) => String::from(args.as_str()),
                None => String::new(),
            };

            return Ok(Some((command, target, args)));
        }
    }

    Ok(None)
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
