use rand::{thread_rng, Rng};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{async_trait, utils};

pub struct Handler;
const ANSWERS_TRUE_FALSE: &'static [&str] = &[
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

const ANSWERS_GENERIC: &'static [&str] = &[
    "Try asking some other time.",
    "_Message blocked in your jurisdiction for legal reasons.",
    "So, erm... basically. It's like... When you think about it, it's really not that complicated.",
    "To view answer, pay 56 Bitcoin to our wallet.",
    "What do *you* think?",
    "why doesn't she love me anymore...",
    "whatever, I don't care",
    "https://tenor.com/view/hog-exploding-exploding-hog-funny-idiot-gif-13702011850125732811",
    "mercy_(overwatch) feet -ai_generated wait wrong tab",
    "🫃",
    "*grabs you by your throat, my wolf teeth snarling* you're MY oomf!! and nobody else's!",
    "🎯",
    "looking into this",
    "🇬🇧🚬",
    "erm.. what the deuce?"
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
                parse_mentioned_message(&ctx.http, &msg).await;
            }
        }
    }
}

async fn parse_mentioned_message(cache_http: impl CacheHttp, message: &Message) {
    let lowercase = message.content.to_lowercase();
    let mut response: String = String::from("girl, idk.");
    if lowercase.contains("is this true")
        || lowercase.contains("is this real")
        || lowercase.contains("is that real")
        || lowercase.contains("is that true")
    {
        response = pick_true_message();
    } else if message.content.ends_with("?") {
        response = pick_generic_answer();
    }
    if let Err(error_reason) = message.reply(cache_http, response).await {
        eprintln!("Error sending message: {error_reason:?}");
    }
}

fn pick_true_message() -> String {
    String::from(ANSWERS_TRUE_FALSE[thread_rng().gen_range(0..ANSWERS_TRUE_FALSE.len())])
}

fn pick_generic_answer() -> String {
    String::from(ANSWERS_GENERIC[thread_rng().gen_range(0..ANSWERS_GENERIC.len())])
}
