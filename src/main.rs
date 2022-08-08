use serenity::async_trait;
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

const TOKEN: &str = "MTAwNTc2MTI1NjQ5MTMzOTc4Ng.GEGPj-.1TmHuvFEFsRbFnS8voIVbwjhGvhv0JsEvYD3cs";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, _: Message) {
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Ready");
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new();
    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .framework(framework)
        .await.unwrap();

    if let Err(err) = client.start().await {
        eprintln!("Cliend found error: {err}");
    }
}
