use chrono::{Datelike, Utc};
use rand::random;
use rand::seq::{IteratorRandom, SliceRandom};
use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildChannel, GuildId};
use serenity::prelude::*;
use std::time::Duration;

const CHANNELS: &[u64] = &[
    935499944176005170, // general-chill
    935499968922402906, // general-chaos
    935500031757271040, // memes
    977157487914549270, // quoth-the-raven
];
const MAX_WAIT_HOURS: u64 = 24;
const MAX_WAIT_MINS: u64 = MAX_WAIT_HOURS * 60;
const MAX_WAIT_SECS: u64 = MAX_WAIT_MINS * 60;

fn get_name() -> String {
    let contents = include_str!("words.txt");
    let split: Vec<&str> = contents.split('\n').collect();
    split.choose(&mut rand::thread_rng()).unwrap().to_string()
}

struct Handler {
    // Mutex feels like overkill, but like technically it's a memory safety thing
    done_birthday: std::sync::Mutex<bool>,
}

impl Handler {
    fn do_birthday(&self, month: u32, date: u32) -> bool {
        let is_birthday = month == 8 && date == 24;

        let mut done_birthday = self.done_birthday.lock().unwrap();

        if is_birthday {
            if *done_birthday {
                false
            } else {
                *done_birthday = true;
                true
            }
        } else {
            if *done_birthday {
                *done_birthday = false;
            }
            false
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Start");

        loop {
            let now = Utc::now();
            let text = if self.do_birthday(now.month(), now.day()) {
                format!("Al is short for {}", get_name())
            } else {
                "AL IS SHORT FOR ALLEGRA HAPPY BIRTHDAY!!!!".to_owned()
            };
            println!("{}", &text);

            let guild = GuildId(935496615916077117);
            let channels: Vec<GuildChannel> = guild
                .channels(&ctx.http)
                .await
                .unwrap()
                .into_iter()
                .filter(|(id, _)| CHANNELS.contains(&id.0))
                .map(|(_, channel)| channel)
                .collect();
            let channel = channels.iter().choose(&mut rand::thread_rng()).unwrap();
            println!("Trying to send to {}", channel.name);
            while let Err(err) = channel.send_message(&ctx.http, |m| m.content(&text)).await {
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
        .event_handler(Handler {
            done_birthday: std::sync::Mutex::new(false),
        })
        .framework(framework)
        .await
        .unwrap();

    if let Err(err) = client.start().await {
        eprintln!("Client found error: {err}");
    }
}

#[test]
fn test_birthday() {
    let handler = Handler {
        done_birthday: std::sync::Mutex::new(false),
    };

    assert!(!handler.do_birthday(5, 5));
    assert!(handler.do_birthday(8, 24));
    assert!(!handler.do_birthday(8, 24));
}
