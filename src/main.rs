use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

const TOKEN: &str = "MTAwNTc2MTI1NjQ5MTMzOTc4Ng.GEGPj-.1TmHuvFEFsRbFnS8voIVbwjhGvhv0JsEvYD3cs";

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let channel = ChannelId(935501260352798751);
        channel.send_message(&ctx.http, |m| {
            m.content("da vinki?")
        }).await.unwrap();
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
