mod challenge;
use challenge::*;

mod utils;
use utils::*;

mod db;

use std::collections::HashSet;

use poise::serenity_prelude::{self as serenity, UserId};
use serenity::GatewayIntents;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn database_setup() {
    db::Connection::new();
}

#[tokio::main]
async fn main() {
    database_setup();
    bot().await;
}

async fn bot() {
    database_setup();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut owners = HashSet::new();
    // Main account
    owners.insert(UserId(275797064674312193));
    // Test account
    owners.insert(UserId(1112188266024812544));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![challenge(), register(), match_started()],
            skip_checks_for_owners: false,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("-".into()),
                mention_as_prefix: true,
                ignore_bots: true,
                ..Default::default()
            },
            owners,
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
