use {
    crate::{commands::bee_sting, storage, storage_models::Scratchpad, util::get_id_from_env},
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
};

pub async fn pass_jenkem(
    ctx: &Context,
    msg: &Message,
    target: &str,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let author = &msg.author;
    let recipient = match target.parse() {
        Ok(id) => UserId::new(id),
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please specify a victim.")
                .await?;
            return Ok(());
        }
    };

    let allergic = [
        get_id_from_env("ABB_CONNER_ID")?,
        get_id_from_env("ABB_WRL_ID")?,
    ];
    let is_allergic = allergic.contains(&recipient.get());

    if !(jenkem_possession_check(ctx, msg, author.id.get(), pad).await? && author.id != recipient) {
        return Ok(());
    }

    if is_allergic {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "{} is allergic to jenkem!",
                    recipient.to_user(&ctx.http).await?.name
                ),
            )
            .await?;

        bee_sting::bee_sting(ctx, msg, pad).await?;
        return Ok(());
    }

    let huff_count = storage::pass_jenkem(recipient.get(), pad)?;
    storage::update_jenkem_streak(huff_count, pad)?;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "{} passed the jenkem to {}! The jenkem has been huffed {} time(s).",
                author.name,
                recipient.to_user(&ctx.http).await?.name,
                huff_count
            ),
        )
        .await?;

    Ok(())
}

pub async fn brew_jenkem(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let author_name = &msg.author.name;
    let author_id = msg.author.id.get();
    storage::init_jenkem(author_id, pad)?;

    msg.channel_id
        .say(
            &ctx.http,
            format!("{} brewed a new batch of jenkem!", author_name),
        )
        .await?;

    Ok(())
}

pub async fn locate_jenkem(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let jenkem_holder = storage::locate_jenkem(pad);
    if jenkem_holder == 0 {
        msg.channel_id
            .say(
                &ctx.http,
                "Oh no, I've lost the jenkem! You'd better brew some more...",
            )
            .await?
    } else {
        let jenkem_holder = UserId::new(jenkem_holder).to_user(&ctx.http).await?;

        msg.channel_id
            .say(&ctx.http, format!("{} has the jenkem!", jenkem_holder.name))
            .await?
    };

    Ok(())
}

pub async fn reject_jenkem(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let message: &str;

    if jenkem_possession_check(ctx, msg, msg.author.id.get(), pad).await? {
        message = match storage::reject_jenkem(pad)? {
            Ok(()) => "The jenkem has been returned!",
            Err(()) => "Can't return the jenkem! You'll have to pass it...",
        };

        msg.channel_id.say(&ctx.http, message).await?;
    };
    Ok(())
}

pub async fn jenkem_streak(
    ctx: &Context,
    msg: &Message,
    pad: &Scratchpad,
) -> Result<(), anyhow::Error> {
    let streak = storage::get_jenkem_streak(pad);

    msg.channel_id
        .say(
            &ctx.http,
            format!("The highest jenkem streak is {}!", streak),
        )
        .await?;
    Ok(())
}

async fn jenkem_possession_check(
    ctx: &Context,
    msg: &Message,
    author_id: u64,
    pad: &Scratchpad,
) -> Result<bool, anyhow::Error> {
    let current_holder = storage::locate_jenkem(pad);

    if current_holder != author_id {
        msg.channel_id
            .say(&ctx.http, "You do not have the jenkem!")
            .await?;
        return Ok(false);
    }

    Ok(true)
}
