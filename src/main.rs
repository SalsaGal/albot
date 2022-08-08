use rand::seq::{SliceRandom, IteratorRandom};
use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;

struct Handler;

fn get_name() -> String {
    let contents = std::fs::read_to_string("src/words.txt").unwrap();
    let split: Vec<&str> = contents.split('\n').collect();
    split.choose(&mut rand::thread_rng()).unwrap().to_string()
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Start");

        let text = format!("Al is short for {}", get_name());
        println!("{}", &text);

        let guild = GuildId(935496615916077117);
        let channels = guild.channels(&ctx.http).await.unwrap();
        let channel = channels.iter().choose(&mut rand::thread_rng()).unwrap();
        println!("Trying to send to {}", channel.1.name);
        while let Err(err) = channel.0.send_message(&ctx.http, |m| {
            m.content(&text)
        }).await {
            println!("Failed: {err}");
        }
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
