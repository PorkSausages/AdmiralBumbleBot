use crate::{
    storage_models::DatabaseLayer,
    util::{get_id_from_env, roll_dice},
};
use serenity::model::id::RoleId;

use {
    super::pastas,
    crate::consciousness,
    regex::Regex,
    serenity::{
        model::{channel::Message, channel::ReactionType, id::EmojiId},
        prelude::{Context, RwLock},
    },
    std::{collections::HashMap, sync::Arc},
};

mod announcement;
mod bee_sting;
mod buzz;
mod clean;
mod common;
mod get_message_data;
mod give_admin;
mod health;
mod help;
mod jenkem;
mod punish;
mod roll;
mod slap;

pub async fn execute(
    ctx: &Context,
    msg: &Message,
    db: &DatabaseLayer,
    ignore_list: Arc<RwLock<HashMap<u64, u8>>>,
) {
    sonic(ctx, msg).await;
    pastas::copypastas(ctx, msg).await;
    consciousness::consciousness(ctx, msg, ignore_list).await;

    if !msg.content.starts_with('$') {
        return;
    }

    let guild_id = msg.guild_id.expect("Error getting guild ID");
    let is_booster = match msg
        .author
        .has_role(
            &ctx.http,
            guild_id,
            RoleId::new(get_id_from_env("ABB_BOOSTER_ROLE")),
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error: {}", e);
            false
        }
    };

    let (command, target, args) = match parse_command(msg.content.as_str()) {
        Some(result) => result,
        None => return,
    };

    if roll_dice("1d20").unwrap() == 20
        && msg.channel_id.get() != get_id_from_env("ABB_BOT_TEST_CHANNEL")
        && !is_booster
    {
        bee_sting::bee_sting(ctx, msg).await;
        return;
    }

    match command.as_str() {
        "$help" => help::help(ctx, msg).await,
        "$buzz" => buzz::buzz(ctx, msg).await,
        "$kick" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Kick).await,
        "$ban" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Ban).await,
        "$mute" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Mute).await,
        "$unmute" => punish::punish(ctx, msg, &target, &args, &punish::Punishment::Unmute).await,
        "$announcement" => announcement::announcement(ctx, msg).await,
        "$giveAdmin" => give_admin::give_admin(ctx, msg, db).await,
        "$clean" => clean::clean(ctx, msg, &args).await,
        "$getMessageData" => get_message_data::get_message_data(ctx, msg, &target, db).await,
        "$slap" => slap::slap(ctx, msg, &target, &args).await,
        "$passJenkem" => jenkem::pass_jenkem(ctx, msg, &target, db).await,
        "$brewJenkem" => jenkem::brew_jenkem(ctx, msg, db).await,
        "$rejectJenkem" => jenkem::reject_jenkem(ctx, msg, db).await,
        "$locateJenkem" => jenkem::locate_jenkem(ctx, msg, db).await,
        "$jenkemStreak" => jenkem::jenkem_streak(ctx, msg, db).await,
        "$roll" => roll::roll(ctx, msg, &args).await,
        "$beeHealthStatus" => health::health(ctx, msg).await,
        _ => {}
    };
}

fn parse_command(text: &str) -> Option<(String, String, String)> {
    let regexes = vec![
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)> (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@!(?P<target>\d+)>").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)> (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) <@(?P<target>\d+)>").unwrap(),
        Regex::new(r"(?P<command>^\$\w+) (?P<args>.*)").unwrap(),
        Regex::new(r"(?P<command>^\$\w+)").unwrap(),
    ];

    for re in regexes {
        if re.is_match(text) {
            let caps = re.captures(text).unwrap();

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

            return Some((command, target, args));
        }
    }

    None
}

async fn sonic(ctx: &Context, msg: &Message) {
    if msg.content.to_ascii_lowercase().contains("sonic")
        || msg.content.to_ascii_lowercase().contains("sanic")
    {
        msg.react(
            &ctx.http,
            ReactionType::Custom {
                id: EmojiId::new(get_id_from_env("ABB_SONIC_EMOTE")),
                animated: false,
                name: Some(String::from("sonic-1")),
            },
        )
        .await
        .expect("⚠ Sonic not added 🦔☹ ⚠");
    }
}
