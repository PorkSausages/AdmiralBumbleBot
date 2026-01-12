use {
    crate::{commands::bee_sting, storage, storage_models::DatabaseLayer},
    serenity::{
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
};

pub async fn pass_jenkem(ctx: &Context, msg: &Message, target: &str, db: &DatabaseLayer) {
    let author = &msg.author;
    let recipient = match target.parse() {
        Ok(id) => UserId(id),
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please specify a victim.")
                .await
                .expect("Error sending message");
            return;
        }
    };

    let allergic = [get_env!("ABB_CONNER_ID", u64), get_env!("ABB_WRL_ID", u64)];
    let is_allergic = allergic.contains(&recipient.0);

    if jenkem_possession_check(ctx, msg, author.id.0, db).await && author.id.0 != recipient.0 {
        if is_allergic {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "{} is allergic to jenkem!",
                        recipient
                            .to_user(&ctx.http)
                            .await
                            .expect("Error getting username")
                            .name
                    ),
                )
                .await
                .expect("Error sending message");

            bee_sting::bee_sting(ctx, msg).await;
            return;
        }

        let huff_count = storage::pass_jenkem(recipient.0, db);
        storage::update_jenkem_streak(huff_count, db);

        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "{} passed the jenkem to {}! The jenkem has been huffed {} time(s).",
                    author.name,
                    recipient
                        .to_user(&ctx.http)
                        .await
                        .expect("Error getting recipient")
                        .name,
                    huff_count
                ),
            )
            .await
            .expect("Error sending message");
    }
}

pub async fn brew_jenkem(ctx: &Context, msg: &Message, db: &DatabaseLayer) {
    let author_name = &msg.author.name;
    let author_id = msg.author.id.0;
    storage::init_jenkem(author_id, db);

    msg.channel_id
        .say(
            &ctx.http,
            format!("{} brewed a new batch of jenkem!", author_name),
        )
        .await
        .expect("Error sending message");
}

pub async fn locate_jenkem(ctx: &Context, msg: &Message, db: &DatabaseLayer) {
    let jenkem_holder = storage::locate_jenkem(db);

    if jenkem_holder == 0 {
        msg.channel_id
            .say(
                &ctx.http,
                "Oh no, I've lost the jenkem! You'd better brew some more...",
            )
            .await
            .expect("Error sending message");
    } else {
        let jenkem_holder = UserId(jenkem_holder)
            .to_user(&ctx.http)
            .await
            .expect("Error getting jenkem holder");

        msg.channel_id
            .say(&ctx.http, format!("{} has the jenkem!", jenkem_holder.name))
            .await
            .expect("Error sending message");
    }
}

pub async fn reject_jenkem(ctx: &Context, msg: &Message, db: &DatabaseLayer) {
    let message: &str;

    if jenkem_possession_check(ctx, msg, msg.author.id.0, db).await {
        message = match storage::reject_jenkem(db) {
            Ok(()) => "The jenkem has been returned!",
            Err(()) => "Can't return the jenkem! You'll have to pass it...",
        };

        msg.channel_id
            .say(&ctx.http, message)
            .await
            .expect("Error rejecting jenkem");
    }
}

pub async fn jenkem_streak(ctx: &Context, msg: &Message, db: &DatabaseLayer) {
    let streak = storage::get_jenkem_streak(db);

    msg.channel_id
        .say(
            &ctx.http,
            format!("The highest jenkem streak is {}!", streak),
        )
        .await
        .expect("Error sending message");
}

async fn jenkem_possession_check(
    ctx: &Context,
    msg: &Message,
    author_id: u64,
    db: &DatabaseLayer,
) -> bool {
    let current_holder = storage::locate_jenkem(db);

    if current_holder != author_id {
        msg.channel_id
            .say(&ctx.http, "You do not have the jenkem!")
            .await
            .expect("Error sending message");
        return false;
    }

    true
}
