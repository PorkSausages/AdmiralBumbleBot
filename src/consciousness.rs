use {
    crate::{util::get_id_from_env, CLEVERBOT_DELAY_SECONDS, CLEVERBOT_LIMIT},
    serenity::{
        model::channel::Message,
        prelude::{Context, RwLock},
    },
    std::{collections::HashMap, env, sync::Arc},
};

pub async fn consciousness(
    ctx: &Context,
    msg: &Message,
    ignore_list: Arc<RwLock<HashMap<u64, u8>>>,
) {
    if msg.channel_id != get_id_from_env("ABB_BOT_CHANNEL") {
        return;
    }

    //Limit snowdude abuse
    let (mut delay_seconds, mut message_limit) = (CLEVERBOT_DELAY_SECONDS, CLEVERBOT_LIMIT);
    if msg.author.id == get_id_from_env("ABB_SNOWDUDE_ID") {
        delay_seconds = 86400;
        message_limit = 5;
    }

    if msg
        .content
        .starts_with(&format!("<@!{}>", get_id_from_env("ABB_BOT_USER_ID")))
        || msg
            .content
            .starts_with(&format!("<@{}>", get_id_from_env("ABB_BOT_USER_ID")))
    //Wtf is this rustfmt
    {
        let user_id = msg.author.id;
        let current_ignore_count: Option<u8>;

        {
            let read_lock = ignore_list.read().await;

            current_ignore_count = match read_lock.get(&user_id.get()) {
                Some(count) => Some(count + 1),
                None => Some(1),
            };
        }

        if let Some(ignore_count) = current_ignore_count {
            if ignore_count < message_limit {
                {
                    let mut write_lock = ignore_list.write().await;
                    write_lock.insert(user_id.get(), current_ignore_count.unwrap());
                }

                tokio::spawn(async move {
                    let arc = ignore_list.clone();

                    tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;

                    let mut write_lock = arc.write().await;
                    let current_count = *write_lock.get(&user_id.get()).unwrap();
                    write_lock.insert(user_id.get(), current_count - 1);
                });

                let content = msg.content.split_once('>').unwrap().1.trim();

                let api_key = env::var("ABB_CLEVERBOT_API_KEY").expect("Missing API Key");
                let state = env::var("ABB_CLEVERBOT_STATE").expect("Missing State");
                let base_url = env::var("ABB_CLEVERBOT_URL").expect("Missing URL");
                let client = reqwest::Client::new();

                let params = [
                    ("key", api_key),
                    ("input", content.to_string()),
                    ("cs", state),
                ];

                let response = client
                    .get(&base_url)
                    .query(&params) // <--- THIS DOES THE MAGIC
                    .send()
                    .await
                    .expect("Bad response from Cleverbot");

                let response_message = format!(
                    "<@{}> {}",
                    msg.author.id,
                    response
                        .json::<HashMap<String, String>>()
                        .await
                        .expect("Can't parse response JSON")
                        .get("output")
                        .expect("No output in Response")
                );

                msg.channel_id
                    .say(&ctx.http, &response_message)
                    .await
                    .expect("Error sending message");
            } else {
                let response = format!("<@{}> HOLY SHIT GO OUTSIDE", user_id);

                msg.channel_id
                    .say(&ctx.http, response)
                    .await
                    .expect("Error sending message");
            }
        }
    }
}
