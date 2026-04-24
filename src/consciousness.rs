use {
    crate::{
        storage_models::{BlockModel, Scratchpad},
        util::{get_id_from_env, random_string},
        CLEVERBOT_DELAY_SECONDS, CLEVERBOT_LIMIT,
    },
    serenity::{model::channel::Message, prelude::Context},
    std::{
        collections::HashMap,
        env,
        time::{SystemTime, UNIX_EPOCH},
    },
};

pub async fn consciousness(ctx: &Context, msg: &Message, pad: &Scratchpad) {
    if msg.channel_id != get_id_from_env("ABB_BOT_CHANNEL")
        || !(msg
            .content
            .starts_with(&format!("<@!{}>", get_id_from_env("ABB_BOT_USER_ID")))
        || msg
            .content
            .starts_with(&format!("<@{}>", get_id_from_env("ABB_BOT_USER_ID"))))
    {
        return;
    }
    let user_id = msg.author.id;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut block_info = pad.with(|pad| {
        pad.block_map
            .get(&user_id.get())
            .unwrap_or(&BlockModel::default())
            .clone()
    });

    if now - block_info.streak_start_seconds >= CLEVERBOT_DELAY_SECONDS {
        block_info.message_count = 0;
        block_info.streak_start_seconds = now;
    }

    if block_info.message_count >= CLEVERBOT_LIMIT {
        let response = format!(
            "<@{}> AdmiralBumbleBee recommends {}.",
            user_id,
            random_string(&[
                "touching grass",
                "finding love",
                "making friends",
                "turning off the computer",
                "going back to the DAW",
                "calling a loved one",
                "going back to school",
                "seeking fulfillment elsewhere",
                "impregnating a hedgehog",
                "going outside",
                "making something of yourself",
                "turning your life around"
            ])
        );
        msg.channel_id
            .say(&ctx.http, response)
            .await
            .expect("Error sending message");
        return;
    }

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
        .query(&params)
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

    pad.with_mut(|pad| {
        pad.block_map.insert(
            user_id.get(),
            BlockModel {
                message_count: block_info.message_count + 1,
                streak_start_seconds: block_info.streak_start_seconds,
            },
        );
    })
}
