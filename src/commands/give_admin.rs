use {
    super::common,
    crate::{
        logging, storage,
        storage_models::Scratchpad,
        util::{get_id_from_env, is_grownup, roll_dice},
    },
    serenity::all::{Context, Message},
};

pub async fn give_admin(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if !common::in_bot_channel(msg.channel_id.get())? {
        return Ok(());
    }

    let has_jenkem = storage::locate_jenkem(pad) == get_id_from_env("ABB_BOT_USER_ID")?;
    let dice_roll = roll_dice("2d20")?;

    if dice_roll >= 39 && !has_jenkem {
        msg.channel_id
            .say(
                &ctx.http,
                "Maybe if I had some high quality jenk I'd feel a little more generous...",
            )
            .await?;

        return Ok(());
    }

    if common::has_wuss_role(ctx, msg).await? {
        msg.channel_id.say(&ctx.http, "get fucked nerd").await?;
        return Ok(());
    }

    if is_grownup(msg.author.id.get())? || (dice_roll >= 39 && has_jenkem) {
        msg.guild_id
            .expect("BumbleBot does not support DMs")
            .member(&ctx.http, msg.author.id)
            .await?
            .add_role(&ctx.http, get_id_from_env("ABB_ADMIN_ROLE")?)
            .await?;

        let log_text = format!("👑 <@!{}> was promoted by me!", msg.author.id);

        msg.channel_id.say(&ctx.http, &log_text).await?;
        logging::log(ctx, &log_text).await;
    }
    Ok(())
}
