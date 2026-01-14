use {
    crate::{
        commands, logging, storage,
        storage_models::DatabaseLayer,
        util::{get_id_from_env, roll_dice},
    },
    serenity::{
        all::ActivityData,
        async_trait,
        model::{
            channel::Message,
            event::MessageUpdateEvent,
            guild::Member,
            id::{ChannelId, GuildId, MessageId},
            prelude::{Ready, User},
        },
        prelude::*,
    },
    similar::{Algorithm, ChangeTag, TextDiff},
    std::{collections::HashMap, sync::Arc, time},
};

pub struct Handler {
    pub storage: Arc<DatabaseLayer>,
    pub ignore_list: Arc<RwLock<HashMap<u64, u8>>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let join_roles: [u64; 2] = [
            get_id_from_env("ABB_JOIN_ROLE_1"),
            get_id_from_env("ABB_JOIN_ROLE_2"),
        ];

        new_member
            .add_role(
                &ctx.http,
                join_roles[roll_dice("1d2").unwrap() as usize - 1],
            )
            .await
            .expect("Error roling new user");

        logging::log(
            &ctx,
            format!("📥 User joined: <@!{}>", new_member.user.id.get()).as_str(),
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
            format!("📤 User left: <@!{}>`", user.id.get()).as_str(),
        )
        .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let arc = self.ignore_list.clone();
        commands::execute(&ctx, &msg, &self.storage, arc).await;

        let user_id = msg.author.id;
        let channel_id = msg.channel_id;
        let word_count = msg.content.split(' ').count() as u16;
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        storage::log_activity(
            user_id.get(),
            channel_id.get(),
            word_count,
            timestamp,
            &self.storage,
        );
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) {
        let (author_id, content) = {
            match ctx.cache.message(channel_id, message_id) {
                Some(message) => (message.author.id, message.content.clone()),
                None => return,
            }
        };
        let stripped_message = content.replace("`", "");
        logging::log(
            &ctx,
            format!(
                "🗑 Message deleted by <@!{}> in <#{}>:\n`{}`",
                author_id.get(),
                channel_id,
                stripped_message
            )
            .as_str(),
        )
        .await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(msg) = old_if_available {
            let new_content = match new {
                Some(ref n) => n.content.clone(),
                None => return,
            };

            if msg.content == new_content {
                return;
            }

            let old_stripped = &msg.content.replace("`", "");
            let new_stripped = &new_content.replace("`", "");
            let mut deletion_buffer = String::new();
            let mut insertion_buffer = String::new();
            let mut res = String::new();

            let diff = TextDiff::configure()
                .algorithm(Algorithm::Patience)
                .diff_words(old_stripped, new_stripped);

            //the diffing algorithm will splice spaces in as equal, we have to handle them like so
            for change in diff.iter_all_changes() {
                match change.tag() {
                    ChangeTag::Delete => {
                        //handle deleted words normally
                        deletion_buffer.push_str(change.as_str().unwrap());
                    }
                    ChangeTag::Insert => {
                        //handle new words normally
                        insertion_buffer.push_str(change.as_str().unwrap());
                    }
                    ChangeTag::Equal => {
                        let text = change.as_str().unwrap();

                        if text.trim().is_empty() {
                            //push spaces to both buffers so they're printed right
                            deletion_buffer.push_str(text);
                            insertion_buffer.push_str(text);
                        } else {
                            //unchanged word, flush both buffers before the word
                            if !deletion_buffer.trim().is_empty() {
                                res.push_str(&format!("~~{}~~ ", &deletion_buffer.trim()));
                                deletion_buffer.clear();
                            }
                            if !insertion_buffer.trim().is_empty() {
                                res.push_str(&format!("**{}** ", &insertion_buffer.trim()));
                                insertion_buffer.clear();
                            }
                            res.push_str(&format!("{} ", text));
                        }
                    }
                }
            }

            //handle the end of the sentence
            if !deletion_buffer.trim().is_empty() && !insertion_buffer.trim().is_empty() {
                //special case to print the last space if something is swapped at the end of the sentence
                res.push_str(&format!("~~{}~~ ", &deletion_buffer.trim()));
            } else if !deletion_buffer.trim().is_empty() {
                res.push_str(&format!("~~{}~~", &deletion_buffer.trim()));
            }
            if !insertion_buffer.trim().is_empty() {
                res.push_str(&format!("**{}**", &insertion_buffer.trim()));
            }

            logging::log(
                &ctx,
                format!(
                    "✏ Message edited by <@!{}> in <#{}>:\n ┣ {}",
                    msg.author.id.get(),
                    msg.channel_id,
                    res,
                )
                .as_ref(),
            )
            .await;
        }
    }

    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Some(ActivityData::playing(
            "Sonic: https://git.io/JfW94 🦔",
        )));
    }
}
