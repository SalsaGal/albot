use rand::seq::SliceRandom;
use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

fn get_name() -> String {
    let mut rng = rand::thread_rng();
    let contents = std::fs::read_to_string("src/words.txt").unwrap();
    let split: Vec<&str> = contents.split('\n').collect();
    split.choose(&mut rng).unwrap().to_string()
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Start");
        let text = format!("Al is short for {}", get_name());
        println!("{}", &text);

        let channel = ChannelId(935501260352798751);
        channel.send_message(&ctx.http, |m| {
            m.content(text)
        }).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new();
    let intents = GatewayIntents::GUILD_MESSAGES;

    let token = std::env::args().nth(1).unwrap();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await.unwrap();

    if let Err(err) = client.start().await {
        eprintln!("Client found error: {err}");
    }
}
