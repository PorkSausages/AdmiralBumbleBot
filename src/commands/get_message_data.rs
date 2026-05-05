use {
    super::common,
    crate::{
        commands::get_message_quips::{
            channel2quip, conditional_quip, fav_quip, last_quip, snd_quip, total_quip, word_quip,
        },
        storage,
        storage_models::MessageModel,
        util::{
            channel2name, get_absolute_amount, get_id_from_env, id2channel, random_string,
            AbsoluteAmount, Channel,
        },
    },
    redb::Database,
    serenity::{
        all::ChannelId,
        model::{channel::Message, id::UserId},
        prelude::Context,
    },
    std::{
        cmp::Reverse,
        collections::HashMap,
        str::FromStr,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
};

const LONG_TIME: u64 = 31_536_000; //difference between first message and now before the bot says "it's over for them" in the intro - 1 year
const MID_TIME: u64 = 2_592_000; //difference between first message and now before the bot says "they can still be saved" in the intro - 1 month
const BIG_TOTAL: usize = 400; //how many messages in a channel to qualify as AbsoluteAmount::Big
const MID_TOTAL: usize = 40; //how many messages in a channel to qualify as AbsoluteAmount::Medium
const BIG_WORDS: usize = 10; //how many average words per message to qualify as AbsoluteAmount::Big
const MID_WORDS: usize = 5; //how many average words per message to qualify as AbsoluteAmount::Medium

fn get_word_amount(bucket: &Vec<MessageModel>) -> AbsoluteAmount {
    get_absolute_amount(
        bucket.iter().map(|msg| msg.words as usize).sum::<usize>() / bucket.len(),
        BIG_WORDS,
        MID_WORDS,
    )
}

fn get_total_amount(bucket: &Vec<MessageModel>) -> AbsoluteAmount {
    get_absolute_amount(bucket.len(), BIG_TOTAL, MID_TOTAL)
}

async fn send_no_data(channel: &ChannelId, username: &str, ctx: &Context) {
    channel
    .say(
        &ctx.http,
        format!(
            "{}, {}.\n\n{}",
            random_string(&[
                "I don't know enough about you",
                "I'm not sure I recall your name",
                "What's the rush? You'll get to know us soon enough"
            ]),
            username,
            "-# Talk in 3 or more main channels so we have enough data to ~~sell to advertisers~~ personalise your experience."
        ),
    )
    .await
    .unwrap();
}

pub async fn get_message_data(ctx: &Context, msg: &Message, target: &str, db: &Database) {
    if !common::in_bot_channel(msg) {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "This command only works in <#{}>.",
                    get_id_from_env("ABB_BOT")
                ),
            )
            .await
            .unwrap();
        return;
    }

    let user_id: u64 = match target.parse() {
        Ok(value) => value,
        Err(_) => msg.author.id.get(),
    };

    let username = UserId::new(user_id).to_user(&ctx.http).await.unwrap().name;
    let now: SystemTime = SystemTime::now();

    let data = storage::get_user_message_data(user_id, db);
    if data.is_empty() {
        send_no_data(&msg.channel_id, &username, ctx).await;
        return;
    }

    let first = data.first().unwrap().clone();
    let mut buckets: HashMap<Channel, Vec<MessageModel>> = HashMap::new();
    let delta = now.duration_since(UNIX_EPOCH).unwrap() - Duration::from_secs(first.time);

    data.into_iter().for_each(|msg| {
        let Some(channel) = id2channel(msg.channel) else {
            return;
        };
        buckets.entry(channel).or_default().push(msg);
    });

    if buckets.len() < 3 {
        send_no_data(&msg.channel_id, &username, ctx).await;
        return;
    }

    let mut sorted_buckets: Vec<(Channel, Vec<MessageModel>)> = buckets.into_iter().collect();
    sorted_buckets.sort_by_key(|(_channel, bucket)| Reverse(bucket.len()));

    let mut response = String::new();
    // i apologise in advance for everything below this line

    //we need to talk about user. it began with x words in the x channel, then went south
    //it's not too late, there's still hope
    response.push_str(&random_string(&[
        format!("When did it go wrong for **{}**? ", username).as_str(),
        format!("It's not looking good for **{}**. ", username).as_str(),
        format!("We need to address the **{}** situation. ", username).as_str(),
    ]));
    response.push_str(
        format!(
            "{} with {} innocent word{} in {} channel... then it {} from there. ",
            random_string(&["It all started", "It began", "The story starts"]),
            first.words,
            if first.words != 1 { "s" } else { "" },
            match id2channel(first.channel) {
                Some(channel) => format!("the {}", channel2name(&channel)),
                None => String::from_str("some random").unwrap(), //this needs to be a String instead of a &str?
            },
            random_string(&["went south", "quickly spiraled", "rapidly deteriorated"])
        )
        .as_str(),
    );
    if (delta.as_secs() / LONG_TIME) > 0 {
        response.push_str(
            format!(
                "{} - {}.",
                random_string(&[
                    "Sadly, it may already be too late",
                    "At this point, there's no turning back",
                    "I fear the brain damage may be terminal"
                ]),
                random_string(&[
                    "god help their soul",
                    "it's never been so over",
                    "it's truly bleak"
                ])
            )
            .as_str(),
        );
    } else if (delta.as_secs() / MID_TIME) > 0 {
        response.push_str(
            format!(
                "{} - {}.",
                random_string(&[
                    "It's not too late, though",
                    "They can still turn it back",
                    "They can still be saved"
                ]),
                random_string(&[
                    "but it's getting harder every second",
                    "but time's running out",
                    "if they turn back now"
                ])
            )
            .as_str(),
        );
    } else {
        response.push_str(
            format!(
                "{} - {}.",
                random_string(&[
                    "They're still in the early stages, though",
                    "It's still early for them",
                    "They're safe, for now"
                ]),
                random_string(&[
                    "but it'll turn sour, quick",
                    "they can still leave before it's too late",
                    "they can still fix this"
                ])
            )
            .as_str(),
        );
    }

    response.push_str("\n\n");

    //fav channel
    let (fav_channel, fav_bucket) = sorted_buckets.first().unwrap();
    let fav_total_amount = get_total_amount(fav_bucket);
    let fav_word_amount = get_word_amount(fav_bucket);
    response.push_str(
        format!(
            //strangely, they only care about xing in the x channel. a whopping x messages sent, but each message is short.
            //they like to yap about xing - but it's mostly just yapping. 
            //i don't get the big deal... why not just do y?
            "{} {} {} in the {} channel. {} {} messages sent, {} each message {}. {} {} - {} {}. {}... {}",
            random_string(&["For some reason,", "Confusingly,", "I'm not sure why, but"]),
            random_string(&[
                "they seem to prefer",
                "they're obsessed with",
                "they only seem interested in"
            ]),
            channel2quip(fav_channel),
            channel2name(fav_channel),
            total_quip(&fav_total_amount),
            fav_bucket.len(),
            conditional_quip(fav_total_amount == fav_word_amount),
            word_quip(&fav_word_amount),
            match fav_total_amount {
                AbsoluteAmount::Big => random_string(&[
                    "They love to yap about",
                    "They go on about",
                    "They live for"
                ]),
                AbsoluteAmount::Medium => random_string(&[
                    "They talk a fair bit about",
                    "They're here to chat about",
                    "They log on to"
                ]),
                AbsoluteAmount::Small => random_string(&[
                    "They pop in to talk about",
                    "They like a cheeky bit of",
                    "They might spend an afternoon"
                ]),
            },
            channel2quip(fav_channel),
            conditional_quip(fav_total_amount == fav_word_amount),
            if fav_total_amount > fav_word_amount {
                random_string(&[
                    "it's mostly just yapping",
                    "it's lacking substance",
                    "they're not saying anything important",
                ])
            } else if fav_total_amount == fav_word_amount {
                random_string(&[
                    "they keep it consistent",
                    "they know what they want",
                    "they don't pretend otherwise",
                ])
            } else {
                random_string(&[
                    "they consistently surprise us",
                    "they always have something to say",
                    "they make sure you'll remember what they said",
                ])
            },
            random_string(&[
                "I don't get the big deal",
                "I'm not sure I understand the point",
                "It's honestly quite baffling"
            ]),
            fav_quip(fav_channel)
        )
        .as_str(),
    );

    response.push_str("\n\n");

    //2nd fav channel
    let (snd_channel, snd_bucket) = sorted_buckets.get(1).unwrap();
    let snd_total_amount = get_total_amount(snd_bucket);
    let snd_word_amount = get_word_amount(snd_bucket);
    response.push_str(
        format!(
            //when they're not xing, they're ying in the y channel - but it's not the same
            //a measly y messages sent, and each message is short.
            //ironically, it seems like they're not as enthusisatic about ying as they are about xing, even if they yap about it more.
            //i can see why they prefer the x channel more though - ying just isn't as fun. 
            "{} {}, they're {} in the {} channel - {} {}. {} {} messages sent, {} each message {}. {}, {} {} as they are about {}, {} {}. {} the {} channel more, though - {}",
            random_string(&[
                "When they're not",
                "In between sessions of",
                "If they're not"
            ]),
            channel2quip(fav_channel),
            channel2quip(snd_channel),
            channel2name(snd_channel),
            conditional_quip(fav_total_amount == snd_total_amount),
            if fav_total_amount != snd_total_amount {
                random_string(&[
                    "they clearly don't enjoy it the same way",
                    "it just isn't the same",
                    "it's clear they don't like it as much",
                ])
            } else {
                random_string(&[
                    "they like it almost as much",
                    "it's almost as fun",
                    "they care almost as much about it",
                ])
            },
            total_quip(&snd_total_amount),
            snd_bucket.len(),
            conditional_quip(snd_total_amount == snd_word_amount),
            word_quip(&snd_word_amount),
            if snd_total_amount != snd_word_amount {
                random_string(&[
                    "Ironically",
                    "Curiously",
                    "Confusingly",
                ])
            } else {
                random_string(&[
                    "Obviously",
                    "Of course",
                    "Logically",
                ])
            },
            if fav_total_amount != snd_total_amount {
                random_string(&[
                    "they don't seem as interested in",
                    "they don't seem to care about",
                    "they're not really that passionate about",
                ])
            } else {
                random_string(&[
                    "they're just as crazed about",
                    "they're just as obsessed about",
                    "they're just as locked in when it comes to",
                ])
            },
            channel2quip(snd_channel),
            channel2quip(fav_channel),
            if fav_word_amount != snd_word_amount {
                random_string(&[
                    "even if",
                    "despite the fact that",
                ])
            } else {
                "and".to_string()
            },
            if snd_word_amount > fav_word_amount {
                random_string(&["they have more to say about it", "their messages are more thoughful", "they put more thought into each message"])
            } else if snd_word_amount == fav_word_amount {
                random_string(&["their messages contain just as much substance", "they have just as much to say about it", "their words match that energy"])
            } else {
                random_string(&["they're not meaningfully contributing to the conversation", "they like to keep it short", "they keep things brief"])
            },
            random_string(&["I can see why they like", "I can see why they prefer", "It's obvious why they enjoy"]),
            channel2name(fav_channel),
            snd_quip(snd_channel)
        )
        .as_str(),
    );

    response.push_str("\n\n");

    //least fav channel
    let (last_channel, last_bucket) = sorted_buckets.last().unwrap();
    let last_total_amount = get_total_amount(last_bucket);
    let last_word_amount = get_word_amount(last_bucket);
    response.push_str(
        format!(
            //their least favourite thing to do is zing in the z channel - even if they partake in it quite often.
            //compared to their love for ying, zing is barely anything to them - but each of their z messages ironically pack a lot of love.
            //it's funny that they barely post yet say more per message than in the x channel - but i think they should show the z channel more love
            //z is actually fun!
            "{} {} in the {} channel - {} {}. {} {}, {} {} - {} each of their {} messages {}. {} that {} {} they {} message when compared to the {} channel - but overall, {} the {} channel {}. {}",
            random_string(&[
                "Their least favourite thing to do is",
                "The thing they're interested in least is",
                "They're really not interested in"
            ]),
            channel2quip(last_channel),
            channel2name(last_channel),
            conditional_quip(last_total_amount == AbsoluteAmount::Small),
            match last_total_amount {
                AbsoluteAmount::Small => random_string(&["they're barely active in it", "nobody really sees them there", "they're not there often"]),
                AbsoluteAmount::Medium => random_string(&["they visit it occasionally", "you might recognise them there", "they're known to visit"]),
                AbsoluteAmount::Big => random_string(&["they're always in there", "they're quite active in there", "they show their face quite often."]),
            },
            random_string(&["Compared to their love for", "Compared to how much they love", "When you look at how much they love"]),
            channel2quip(fav_channel),
            channel2quip(last_channel),
            if last_total_amount != fav_total_amount {
                random_string(&[
                    "is barely anything to them",
                    "practically doesn't matter to them",
                    "means nothing to them",
                ])
            } else {
                random_string(&[
                    "is almost as important to them",
                    "matters almost as much to them",
                    "is on their mind almost as much",
                ])
            },
            conditional_quip(last_total_amount == last_word_amount),
            last_bucket.len(),
            match last_word_amount {
                AbsoluteAmount::Big => random_string(&[
                    "packs a lot of love",
                    "contains a lot of thought",
                    "has a lot to say"
                ]),
                AbsoluteAmount::Medium => random_string(&[
                    "says a fair bit",
                    "has something to say",
                    "makes a point"
                ]),
                AbsoluteAmount::Small => random_string(&[
                    "has nothing to say",
                    "is mostly just filler",
                    "doesn't contribute much"
                ]),
            },
            if last_word_amount >= fav_word_amount {
                random_string(&["It's funny", "It humours me", "I find it interesting"])
            } else {
                random_string(&["It's no surprise", "It won't surprise anyone to find", "It's clear"])
            },
            if last_total_amount == fav_total_amount {
                random_string(&["they post almost as much", "they're posting almost as much", "they talk almost as much"])
            } else {
                random_string(&["they barely post", "they're quite reserved", "they only post occasionally"])
            },
            conditional_quip(last_word_amount < fav_word_amount),
            if last_word_amount > fav_word_amount {
                random_string(&["say more per", "put more thought in per", "have more to say per"])
            } else if last_word_amount == fav_word_amount {
                random_string(&["have just as much to say per", "think just as much about each", "put just as much thought into each"])
            } else {
                random_string(&["put less thought into each", "have little to say per", "don't really think about each"])
            },
            channel2name(fav_channel),
            random_string(&["I think they should give", "It would be nice if they gave", "I personally would give"]),
            channel2name(last_channel),
            random_string(&["more love", "another go", "another shot"]),
            last_quip(last_channel)
        )
        .as_str(),
    );

    msg.channel_id.say(&ctx.http, &response).await.unwrap();
    response.clear();

    response.push_str("# **Ranked Activity:**\n");
    for (idx, (channel, bucket)) in sorted_buckets.iter().enumerate() {
        response.push_str(
            format!(
                "{}. {} in the {} channel (yapping level: {})\n",
                idx,
                channel2quip(channel),
                channel2name(channel),
                match get_word_amount(bucket) {
                    AbsoluteAmount::Small => random_string(&["low", "tiny", "minimal"]),
                    AbsoluteAmount::Medium => random_string(&["medium", "fair", "acceptable"]),
                    AbsoluteAmount::Big => random_string(&["huge", "unacceptable", "troubling"]),
                }
            )
            .as_str(),
        );
    }

    response.push_str(format!("\n-# Took {:.2?}", now.elapsed().unwrap()).as_str());
    msg.channel_id.say(&ctx.http, response).await.unwrap();
}
