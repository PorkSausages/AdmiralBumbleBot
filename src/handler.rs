use {
    crate::{commands, logging, storage},
    serenity::{
        async_trait,
        model::{
            channel::Message,
            event::MessageUpdateEvent,
            guild::{Member},
            id::{ChannelId, GuildId, MessageId},
            prelude::{Activity, Ready, User},
        },
        prelude::*,
    },
    std::{collections::HashMap, sync::Arc, time},
};

pub struct Handler {
    pub storage: sled::Db,
    pub ignore_list: Arc<RwLock<HashMap<u64, u8>>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(
        &self,
        ctx: Context,
        mut new_member: Member,
    ) {
        let join_roles: Vec<u64> = vec![
            get_env!("ABB_JOIN_ROLE_1", u64),
            get_env!("ABB_JOIN_ROLE_2", u64),
        ];

        new_member
            .add_role(
                &ctx.http,
                join_roles[d20::roll_dice("1d2").unwrap().total as usize - 1],
            )
            .await
            .expect("Error roling new user");

        logging::log(
            &ctx,
            format!("📥 User joined: <@!{}>", new_member.user.id.as_u64()).as_str(),
        )
        .await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        _guild: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        logging::log(
            &ctx,
            format!("📤 User left: <@!{}>`", user.id.as_u64()).as_str(),
        )
        .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let arc = self.ignore_list.clone();
        commands::execute(&ctx, &msg, &self.storage, arc).await;

        let user_id = *msg.author.id.as_u64();
        let channel_id = *msg.channel_id.as_u64();
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        storage::log_activity(
            user_id,
            channel_id,
            msg.content.split(' ').count() as u16,
            timestamp,
            &self.storage,
        );
    }

    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId, _guild_id: Option<GuildId>) {
        let deleted_message = ctx.cache.message(channel_id, message_id);
        if let Some(message) = deleted_message {
            let stripped_message = message.content.replace("`", "");

            logging::log(
                &ctx,
                format!(
                    "🗑 Message deleted by <@!{}> in <#{}>:\n`{}`",
                    message.author.id.as_u64(),
                    channel_id,
                    stripped_message
                )
                .as_str(),
            )
            .await;
        }
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(msg) = old_if_available {
            let new_content = new.unwrap().content;
            if msg.content == new_content {
                //Embeds register as identical edits for some reason
                return;
            }

            let old_stripped = &msg.content.replace("`", "");
            let new_stripped = &new_content.replace("`", "");

            logging::log(
                &ctx,
                format!(
                    "✏️ Message edited by <@!{}> in <#{}>:\n` ┣ Original: {}`\n` ┗ Edited:   {}`",
                    msg.author.id.as_u64(),
                    msg.channel_id,
                    old_stripped,
                    new_stripped
                )
                .as_ref(),
            )
            .await;
        }
    }

    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::playing(
            "See my insides at https://git.io/JfW94 😘",
        ))
        .await;
    }
}
