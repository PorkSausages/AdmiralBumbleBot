use {
    crate::{CLEVERBOT_DELAY_SECONDS, CLEVERBOT_LIMIT},
    serenity::{
        model::channel::Message,
        prelude::{Context, RwLock},
    },
    std::{collections::HashMap, sync::Arc},
    url::form_urlencoded,
};

pub async fn consciousness(
    ctx: &Context,
    msg: &Message,
    ignore_list: Arc<RwLock<HashMap<u64, u8>>>,
) {
    if msg.channel_id.0 != get_env!("ABB_BOT_CHANNEL", u64) {
        return;
    }

    //Limit snowdude abuse
    let (mut delay_seconds, mut message_limit) = (CLEVERBOT_DELAY_SECONDS, CLEVERBOT_LIMIT);
    if msg.author.id.0 == get_env!("ABB_SNOWDUDE_ID", u64) {
        delay_seconds = 86400;
        message_limit = 5;
    }

    if msg
        .content
        .starts_with(&format!("<@!{}>", get_env!("ABB_BOT_USER_ID")))
        || msg
            .content
            .starts_with(&format!("<@{}>", get_env!("ABB_BOT_USER_ID")))
    //Wtf is this rustfmt
    {
        let user_id = msg.author.id.0;
        let current_ignore_count: Option<u8>;

        {
            let read_lock = ignore_list.read().await;

            current_ignore_count = match read_lock.get(&user_id) {
                Some(count) => Some(count + 1),
                None => Some(1),
            };
        }

        if let Some(ignore_count) = current_ignore_count {
            if ignore_count < message_limit {
                {
                    let mut write_lock = ignore_list.write().await;
                    write_lock.insert(user_id, current_ignore_count.unwrap());
                }

                tokio::spawn(async move {
                    let arc = ignore_list.clone();

                    tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;

                    let mut write_lock = arc.write().await;
                    let current_count = *write_lock.get(&user_id).unwrap();
                    write_lock.insert(user_id, current_count - 1);
                });

                let content = msg.content.split_once('>').unwrap().1.trim();

                let request_url = form_urlencoded::Serializer::new(format!(
                    "{}?key={}",
                    get_env!("ABB_CLEVERBOT_URL"),
                    get_env!("ABB_CLEVERBOT_API_KEY")
                ))
                .append_pair("input", content)
                .append_pair("cs", &get_env!("ABB_CLEVERBOT_STATE"))
                .finish();

                let response = reqwest::get(&request_url)
                    .await
                    .expect("Error making request to Cleverbot API")
                    .json::<HashMap<String, String>>()
                    .await
                    .expect("Error deserializing Cleverbot response");

                let response_message = format!("<@{}> {}", msg.author.id.0, response["output"]);

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
