use {
    super::common,
    crate::{
        logging, storage,
        storage_models::Scratchpad,
        util::{get_id_from_env, roll_dice},
    },
    serenity::{model::channel::Message, prelude::Context},
};

pub async fn give_admin(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    if !common::in_bot_channel(msg)? {
        return Ok(());
    }

    let guild_id = msg.guild_id.expect("BumbleBot does not support DMs");
    let author = &msg.author;
    let has_jenkem = storage::locate_jenkem(pad) == get_id_from_env("ABB_BOT_USER_ID")?;
    let dice_roll = roll_dice("2d20")?;
    let grownups = [
        get_id_from_env("ABB_PORKSAUSAGES_ID")?,
        get_id_from_env("ABB_WRL_ID")?,
        get_id_from_env("ABB_M4X_ID")?,
    ];
    let is_grownup = grownups.contains(&author.id.get());

    if dice_roll >= 39 && !has_jenkem {
        msg.channel_id
            .say(
                &ctx.http,
                "Maybe if I had some high quality jenk I'd feel a little more generous...",
            )
            .await?;

        return Ok(());
    }

    if common::has_wuss_role(ctx, author, guild_id).await? {
        msg.channel_id.say(&ctx.http, "get fucked nerd").await?;

        return Ok(());
    }

    if is_grownup || (dice_roll >= 39 && has_jenkem) {
        guild_id
            .member(&ctx.http, author.id)
            .await?
            .add_role(&ctx.http, get_id_from_env("ABB_ADMIN_ROLE")?)
            .await?;

        let log_text = format!("👑 <@!{}> was promoted by me!", author.id);

        msg.channel_id.say(&ctx.http, &log_text).await?;

        logging::log(ctx, &log_text).await;
    }
    Ok(())
}
