use crate::{
    commands::common::{confirm_admin, send_clean_message},
    storage_models::{PastaModel, Scratchpad},
    util::{get_id_from_env, random_string, roll_dice},
};
use serenity::{all::CreateAllowedMentions, model::channel::Message, prelude::Context};

pub async fn check_pasta(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if msg.channel_id == get_id_from_env("ABB_SHIT")? //"attempt to reduce copypasta spam"
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

    send_clean_message(
        ctx,
        msg.channel_id,
        &payload,
        CreateAllowedMentions::new().replied_user(true),
    )
    .await?;

    Ok(())
}

pub async fn get_pasta(
    ctx: &Context,
    msg: &Message,
    slug: Option<String>,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let response = match slug.and_then(|s| {
        pad.with(|pad| {
            pad.pastas
                .iter()
                .find(|(pasta_slug, _pasta)| pasta_slug.eq_ignore_ascii_case(&s))
                .map(|(_slug, pasta)| pasta.clone())
        })
    }) {
        Some(pasta) => format!(
            "**Triggers:** {:?}\n**Chance:** {}/100\n**Includes mention:** {}\n**Payload:** {}",
            pasta.triggers, pasta.chance, pasta.includes_mention, pasta.payload
        ),
        None => format!(
            "No pasta found for this slug.\nExisting pasta slugs:`{:?}`",
            pad.with(|pad| { pad.pastas.keys().cloned().collect::<Vec<String>>() })
        ),
    };
    send_clean_message(ctx, msg.channel_id, &response, CreateAllowedMentions::new()).await?;
    Ok(())
}

pub async fn set_pasta(
    ctx: &Context,
    msg: &Message,
    command: Option<String>,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if !confirm_admin(ctx, msg).await? {
        return Ok(());
    }

    let response = match {
        || -> Result<_, String> {
            let parts = command
                .and_then(|s| shlex::split(&s))
                .ok_or("Invalid command")?;
            let mut parts = parts.iter();

            let slug = parts.next().ok_or("Missing slug")?.to_string();
            let mut triggers = Vec::new();
            let mut chance: u32 = 100;
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
        Ok((slug, pasta)) => {
            pad.with_mut(|pad| pad.pastas.insert(slug.clone(), pasta.clone()))?;
            format!(
                "Updated the `{}` pasta.\n**Triggers:** {:?}\n**Chance:** {}/100\n**Includes mention:** {}\n**Payload:** {}",
                slug, pasta.triggers, pasta.chance, pasta.includes_mention, pasta.payload
            )
        }
        Err(error) => {
            format!(
                "**Error: {}**\nUsage: `$setPasta SLUG trigger:'TRIGGER1' trigger:'TRIGGER2'* chance:(1-100)* mention:(true|false)* payload:'PASTA'`\n*=optional",
                error
            )
        }
    };

    send_clean_message(ctx, msg.channel_id, &response, CreateAllowedMentions::new()).await?;
    Ok(())
}

pub async fn del_pasta(
    ctx: &Context,
    msg: &Message,
    slug: Option<String>,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if !confirm_admin(ctx, msg).await? {
        return Ok(());
    }

    let response = match slug.map_or(Ok(None), |s| pad.with_mut(|pad| pad.pastas.remove(&s)))? {
        Some(_) => format!(
            "Pasta deleted. {}",
            random_string(&[
                "I liked that one.",
                "What a shame.",
                "Wasn't really fond of that one to begin with.",
                "Good riddance."
            ])
        ),
        None => format!(
            "No pasta found for this slug.\nExisting pasta slugs:`{:?}`",
            pad.with(|pad| { pad.pastas.keys().cloned().collect::<Vec<String>>() })
        ),
    };

    send_clean_message(ctx, msg.channel_id, &response, CreateAllowedMentions::new()).await?;
    Ok(())
}
