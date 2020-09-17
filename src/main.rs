use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {
        return;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{name} is connected!\nGuilds: {guilds}\nID: {id}",
            name=ready.user.name,
            guilds=ready.guilds.len(),
            id=ready.user.id
        );
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let token = env::var("TOKEN").expect("token");
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}