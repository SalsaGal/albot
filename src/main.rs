use rand::random;
use rand::seq::{SliceRandom, IteratorRandom};
use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildId, GuildChannel};
use serenity::prelude::*;
use std::time::Duration;

struct Handler;

const CHANNELS: &[u64] = &[
    935499944176005170, // general-chill
    935499968922402906, // general-chaos
    935500031757271040, // memes
    977157487914549270, // quoth-the-raven
];
const MAX_WAIT_MINS: u64 = 60;
const MAX_WAIT_SECS: u64 = MAX_WAIT_MINS * 60;

fn get_name() -> String {
    let contents = std::fs::read_to_string("src/words.txt").unwrap();
    let split: Vec<&str> = contents.split('\n').collect();
    split.choose(&mut rand::thread_rng()).unwrap().to_string()
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Start");

        loop {
            let text = format!("Al is short for {}", get_name());
            println!("{}", &text);

            let guild = GuildId(935496615916077117);
            let channels: Vec<GuildChannel> = guild.channels(&ctx.http).await.unwrap().into_iter().filter(
                |(id, _)| {
                    CHANNELS.contains(&id.0)
                }
            ).map(|(_, channel)| channel).collect();
            let channel = channels.iter().choose(&mut rand::thread_rng()).unwrap();
            println!("Trying to send to {}", channel.name);
            while let Err(err) = channel.send_message(&ctx.http, |m| {
                m.content(&text)
            }).await {
                println!("Failed: {err}");
            }
            tokio::time::sleep(Duration::from_secs(random::<u64>() % MAX_WAIT_SECS)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new();
    let intents = GatewayIntents::GUILD_MESSAGES;

    let token = include_str!("token.txt");
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await.unwrap();

    if let Err(err) = client.start().await {
        eprintln!("Client found error: {err}");
    }
}
