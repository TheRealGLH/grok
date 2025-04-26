use rand::{thread_rng, Rng};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{async_trait, utils};

pub struct Handler;
const ANSWERS: &'static [&str] = &[
    "It is certain.",
    "It is decidedly so.",
    "Without a doubt.",
    "Yes, definitely",
    "You may rely on it",
    "As I see it, yes.",
    "Most likely",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",
    "kys",
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Don't count on it.",
    "FAKE NEWS!!",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good",
    "Very doubtful",
    "Hop on Guilty Gear",
    "Hop on Rivals, kitten 😈",
    ":dogbrow:",
    "Check Community Notes.",
    "Buy $DOGE PLEEEAAAAASE!! 😭😭😭",
    "iT'S a rOMan SalUte!",
    "Alternative facts",
    "Uhmmmm, it's called 𝕏™️: The Everything App. Not \"Twitter\".",
    "Skill issue.",
    "🤏🍆   😂😂😂",
    "What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little \"clever\" comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo.",
    "idk",
];
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Message received: {}:{}", msg.author, msg.content);
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }
        for ping in msg.content.split_whitespace() {
            if let Some(p) = utils::parse_user_mention(ping) {
                if p != ctx.cache.current_user().id {
                    break;
                };
                parse_message(&ctx.http, &msg).await;
            }
        }
    }
}

async fn parse_message(cache_http: impl CacheHttp, message: &Message) {
    let lowercase = message.content.to_lowercase();
    if !lowercase.contains("is this true") && !lowercase.contains("is this real") {
        println!("invalid message. ignoring");
        return;
    }
    if let Err(error_reason) = message.reply(cache_http, pick_true_message()).await {
        eprintln!("Error sending message: {error_reason:?}");
    }
}

fn pick_true_message() -> String {
    String::from(ANSWERS[thread_rng().gen_range(0..ANSWERS.len())])
}
