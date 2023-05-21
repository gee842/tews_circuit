use std::env;

use serenity::async_trait;
use serenity::cache::Cache;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Role;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "Say something, Tews!" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error: {:?}", why);
                    return;
                }
            };

            let response = MessageBuilder::new()
                .push("Fine. Something. You happy, ")
                .push_bold_safe(&msg.author.name)
                .push("?")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error: {:?}", why);
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is active!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "MTEwMTk1NzYwODA4MTk4NTU0Nw.GceCwl.FBcMQ8_pHXbhdJ8Y8-yMkEAZrZmYd07_wNNfqU";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

/// Will challenge another discord user. A user cannot be challenged
/// if it has the "On Hiatus" role, or is already challenged by someone
/// else.
#[command] 
async fn challenge(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Challenged.").await?;

    // let cache = Cache::new();
    // let roles = cache.guild_roles(msg.guild_id.unwrap()).unwrap();
    // println!("{:#?}", roles);

    // msg.reply(ctx, "Hello.").await?;

    Ok(())
}
