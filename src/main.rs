use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let channel = ChannelId(935501260352798751);
        println!("Start");
        channel.send_message(&ctx.http, |m| {
            m.content("da vinki?")
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
