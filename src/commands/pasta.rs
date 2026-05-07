use crate::{
    commands::common::confirm_admin,
    storage_models::{PastaModel, Scratchpad},
    util::{get_id_from_env, random_string, roll_dice},
};
use serenity::{model::channel::Message, prelude::Context};

pub async fn check_pasta(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if msg.channel_id == get_id_from_env("ABB_SHITPOST_CHANNEL")? //"attempt to reduce copypasta spam"
        || msg.author.id == get_id_from_env("ABB_BOT_USER_ID")?
    {
        return Ok(());
    }

    let Some(pasta) = pad.with(|pad| {
        pad.pastas
            .iter()
            .find(|(_slug, pasta)| {
                pasta
                    .triggers
                    .iter()
                    .any(|trigger| trigger.eq_ignore_ascii_case(&msg.content))
            })
            .map(|(_slug, pasta)| pasta.clone())
    }) else {
        return Ok(());
    };

    if roll_dice("1d100")? > pasta.chance {
        return Ok(());
    }

    let payload = format!(
        "{}{}",
        if pasta.includes_mention {
            format!("<@{}> ", msg.author.id)
        } else {
            "".to_string()
        },
        pasta.payload
    );

    msg.channel_id.say(&ctx.http, &payload).await?;

    Ok(())
}

pub async fn get_pasta(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
    args: &str,
) -> Result<(), anyhow::Error> {
    let Some(pasta) = pad.with(|pad| {
        pad.pastas
            .iter()
            .find(|(slug, _pasta)| slug.eq_ignore_ascii_case(args))
            .map(|(_slug, pasta)| pasta.clone())
    }) else {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "No pasta found for this slug.\nExisting pasta slugs:`{:?}`",
                    pad.with(|pad| { pad.pastas.keys().cloned().collect::<Vec<String>>() })
                ),
            )
            .await?;
        return Ok(());
    };

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "**Triggers:** {:?}\n**Chance:** {}/100\n**Includes mention:** {}\n**Payload:** {}",
                pasta.triggers, pasta.chance, pasta.includes_mention, pasta.payload
            ),
        )
        .await?;
    Ok(())
}

pub async fn set_pasta(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
    args: &str,
) -> Result<(), anyhow::Error> {
    if !confirm_admin(ctx, &msg.author, msg.guild_id.expect("BumbleBot does not support DMs")).await? {
        return Ok(());
    }

    let (slug, pasta) = match {
        || -> Result<_, String> {
            let parts = shlex::split(args).ok_or("Invalid quoting")?;
            let mut parts = parts.iter();

            let slug = parts.next().ok_or("Missing slug")?.to_string();
            let mut triggers = Vec::new();
            let mut chance = 100u32;
            let mut includes_mention = false;
            let mut payload = None;

            for part in parts {
                if let Some(val) = part.strip_prefix("trigger:") {
                    triggers.push(val.to_string());
                } else if let Some(val) = part.strip_prefix("chance:") {
                    let n: u32 = val
                        .parse()
                        .map_err(|_| format!("Invalid chance: '{}'", val))?;
                    if !(1..=100).contains(&n) {
                        return Err(format!("Chance must be between 1 and 100, got '{}'", n));
                    }
                    chance = n;
                } else if let Some(val) = part.strip_prefix("mention:") {
                    includes_mention = val
                        .parse()
                        .map_err(|_| format!("Invalid mention: '{}'", val))?;
                } else if let Some(val) = part.strip_prefix("payload:") {
                    payload = Some(val.to_string());
                } else {
                    return Err(format!("Unknown argument: '{}'", part));
                }
            }

            if triggers.is_empty() {
                return Err("Expected at least one trigger".to_string());
            }

            Ok((
                slug,
                PastaModel {
                    triggers,
                    payload: payload.ok_or("Missing payload")?,
                    chance,
                    includes_mention,
                },
            ))
        }
    }() {
        Ok((slug, pasta)) => (slug, pasta),
        Err(error) => {
            msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "**{}**\nUsage: `$setPasta SLUG trigger:'TRIGGER1' trigger:'TRIGGER2'* chance:(1-100)* mention:(true|false)* payload:'PASTA'`\n*=optional",
                    error
                ),
            )
            .await?;
            return Ok(());
        }
    };

    pad.with_mut(|pad| pad.pastas.insert(slug.clone(), pasta.clone()))?;

    msg.channel_id
    .say(
        &ctx.http,
        format!(
            "Updated the `{}` pasta.\n**Triggers:** {:?}\n**Chance:** {}/100\n**Includes mention:** {}\n**Payload:** {}",
            slug, pasta.triggers, pasta.chance, pasta.includes_mention, pasta.payload
        ),
    )
    .await?;

    Ok(())
}

pub async fn del_pasta(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
    args: &str,
) -> Result<(), anyhow::Error> {
    if !confirm_admin(
        ctx,
        &msg.author,
        msg.guild_id.expect("BumbleBot does not support DMs"),
    )
    .await?
    {
        return Ok(());
    }

    if pad.with_mut(|pad| pad.pastas.remove(args))?.is_none() {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "No pasta found for this slug.\nExisting pasta slugs:`{:?}`",
                    pad.with(|pad| { pad.pastas.keys().cloned().collect::<Vec<String>>() })
                ),
            )
            .await?;
        return Ok(());
    };

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "Pasta deleted. {}",
                random_string(&[
                    "I liked that one.",
                    "What a shame.",
                    "Wasn't really fond of that one to begin with.",
                    "Good riddance."
                ])
            ),
        )
        .await?;

    Ok(())
}
