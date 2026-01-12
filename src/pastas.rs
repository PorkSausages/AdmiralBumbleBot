use serenity::{client::Context, model::channel::Message};

const OK_PASTA: &str = "\"Ok\"? Are you fucking kidding me? I spent a decent portion of my life writing all of that and your response to me is \"Ok\"? Are you so mentally handicapped that the only word you can comprehend is \"Ok\" - or are you just some fucking asshole who thinks that with such a short response, he can make a statement about how meaningless what was written was? Well, I'll have you know that what I wrote was NOT meaningless, in fact, I even had my written work proof-read by several professors of literature. Don't believe me? I doubt you would, and your response to this will probably be \"Ok\" once again. Do I give a fuck? No, does it look like I give even the slightest fuck about two fucking letters? I bet you took the time to type those two letters too, I bet you sat there and chuckled to yourself for 20 hearty seconds before pressing \"send\". You're so fucking pathetic. I'm honestly considering directing you to a psychiatrist, but I'm simply far too nice to do something like that. You, however, will go out of your way to make a fool out of someone by responding to a well-thought-out, intelligent, or humorous statement that probably took longer to write than you can last in bed with a chimpanzee. What do I have to say to you? Absolutely nothing. I couldn't be bothered to respond to such a worthless attempt at a response. Do you want \"Ok\" on your gravestone?";
const BYE_PASTA: &str = "Bye. I'm leaving the server because I'm bored. Despite talking to all of you for a while I don't care about any of you at all. I lied. I'm not 14 I'm 13. And a girl. I can probably draw better than yall as well. Just sayin. Cya.";
const SHIRTLESS_PASTA: &str = "Me, Me when your girl, You, You when your girl, When your girl see, When your girl sees me, When, When Like, When I'm like shirtless, So, So, Soaking wet, Get";
const BASED_PASTA: &str = "Based? Based on what? In your dick? Please shut the fuck up and use words properly you fuckin troglodyte, do you think God gave us a freedom of speech just to spew random words that have no meaning that doesn't even correllate to the topic of the conversation? Like please you always complain about why no one talks to you or no one expresses their opinions on you because you're always spewing random shit like poggers based cringe and when you try to explain what it is and you just say that it's funny like what? What the fuck is funny about that do you think you'll just become a stand-up comedian that will get a standing ovation just because you said \"cum\" in the stage? HELL NO YOU FUCKIN IDIOT, so please shut the fuck up and use words properly you dumb bitch.";
const NFT_PASTA: &str = "You think it’s funny to take screenshots of people’s NFTs, huh? You must be a very immature person to steal someone’s property that they PAID for. Yeah, I said it. You’re the kind of person who thinks that property theft (a seriously illegal offence) is a joke. I don’t even know why you took that screenshot, because you didn’t pay 1000 dollars for it. I did. The blockchain doesn’t lie. Even if you try to save it, it’s my property. You’re just angry that you couldn’t afford this priceless masterpiece. Even if you could, your fingers couldn’t even click fast enough to get one of the 10000 NFTs sold. You’re just mad you don’t own what I own. So, delete that screenshot, or I swear, you’ll be hearing from my lawyers. /s";
const JENKEM_PASTA: &str = "every free man deserves the right to choose his own destiny and toil, his own soil and means of dealing with said soil. exactly why I DIYed my own septic tank a few years ago - to stop the Big Shit™☭ fucking commies stealing my rightfully owned, god-given jenkem. in the name of greatest people to have ever trod this earth, i urge you all to toss the gauntlet before the feet of tyranny and dynamite your sewerage line and install your own septic tank system in peaceful protest against the Big Shit™☭ commies i'm glad i hooked my janktank up to my HVAC before winter bc i've been burning the fumes to heat my house, every surface is now coated with a slightly-sticky film which i believe has strong insulative properties, and all my belongings smell like shit and piss (but in a good way). been running my car on jenk and saving the planet from bad emissions and my wallet from gas prices, started a refinery business and been selling my refined jenkem on the foreign market for Big Baller Bucks. also tried fermenting my jenk further to brew craft beers but people said it tasted a bit too shitty, dumb hipster fucks don't know what's good. i guess you can't please everyone... Also started a cryptocurrency and duped a fuckload of dumb investors with my innovative ideas, every time someone flushes their toilet it gets written to the blockchain. this toilet-flush data will be the lifeblood of jenkem futures and derivatives. i've just about finished my pump-and-dump scheme and i'm about to dump all my shitcoins and make a trillion dollars - but not before i take another dump to top up my jenkem supply. this has been a tough 12 months alright, but when life gives you shit, make jenkem.";
const WDHM_PASTA: &str = "Surge";

const FL_IMG: &str = "https://media.discordapp.net/attachments/1230211453311778931/1230211520856592424/IMG_6716.jpg?ex=66327ec2&is=662009c2&hm=1b96ee2d73ba638b9b01972b999e568a77c47707a4b247e1d608eb55b18aa107&=&format=webp&width=758&height=505";
const BITWIG_VID: &str = "https://cdn.discordapp.com/attachments/784498744506580992/1210284372310237244/trim.66E02CFF-5FD2-4175-AA21-4372AA841015.mov";
const SENNHEISER_VID: &str = "https://cdn.discordapp.com/attachments/784498744506580992/1224166761692397660/Snapsave.app_434596257_1501537313768606_3029272559969168436_n.mp4";

pub async fn copypastas(ctx: &Context, msg: &Message) {
    if msg.channel_id.0 == get_env!("ABB_SHITPOST_CHANNEL", u64) {
        return;
    }

    pasta(ctx, msg, OK_PASTA, "Ok").await;

    if d20::roll_dice("1d20").unwrap().total > 10 {
        return;
    }

    //Pastas
    pasta(ctx, msg, "ok", OK_PASTA).await;
    pasta(ctx, msg, "bye", BYE_PASTA).await;
    pasta(ctx, msg, "me", SHIRTLESS_PASTA).await;
    pasta(ctx, msg, "based", BASED_PASTA).await;
    pasta(ctx, msg, "nft", NFT_PASTA).await;
    pasta(ctx, msg, "jenkem", JENKEM_PASTA).await;
    pasta(ctx, msg, "what did he mean by that", WDHM_PASTA).await;

    //Image/Video Responses
    fl_is_fine(ctx, msg).await;
    bitwig(ctx, msg).await;
    sennheiser(ctx, msg).await;
}

pub async fn pasta(ctx: &Context, msg: &Message, trigger: &str, pasta: &str) {
    if msg.content.eq_ignore_ascii_case(trigger)
        && msg.author.id.0 != get_env!("ABB_BOT_USER_ID", u64)
    {
        msg.channel_id
            .say(&ctx.http, pasta)
            .await
            .expect("Error sending message");
    }
}

pub async fn response(ctx: &Context, msg: &Message, trigger: &str, response: &str) {
    if msg.content.eq_ignore_ascii_case(trigger)
        && msg.author.id.0 != get_env!("ABB_BOT_USER_ID", u64)
    {
        msg.channel_id
            .say(
                &ctx.http,
                format!("<@{}> {}", msg.author.id.as_u64(), response),
            )
            .await
            .expect("Error sending message");
    }
}

async fn fl_is_fine(ctx: &Context, msg: &Message) {
    response(ctx, msg, "fl is fine", FL_IMG).await;
    response(ctx, msg, "fl studio is fine", FL_IMG).await;
    response(ctx, msg, "fl is good", FL_IMG).await;
    response(ctx, msg, "fl studio is good", FL_IMG).await;
    response(ctx, msg, "there's nothing wrong with fl studio", FL_IMG).await;
    response(ctx, msg, "there is nothing wrong with fl studio", FL_IMG).await;
    response(ctx, msg, "there's nothing wrong with fl", FL_IMG).await;
    response(ctx, msg, "there is nothing wrong with fl", FL_IMG).await;
}

async fn bitwig(ctx: &Context, msg: &Message) {
    response(ctx, msg, "i use bitwig", BITWIG_VID).await;
}

async fn sennheiser(ctx: &Context, msg: &Message) {
    response(ctx, msg, "sennheiser", SENNHEISER_VID).await;
}
