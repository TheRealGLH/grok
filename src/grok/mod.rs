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
    ":dogbrow:"
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
