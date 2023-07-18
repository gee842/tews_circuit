mod challenge;
use challenge::*;

mod utils;
use utils::*;

mod db;
use db::Database;

use std::{collections::HashSet, error::Error as StdError};

use poise::serenity_prelude::{self as serenity, CreateThread, UserId};
use serenity::GatewayIntents;

type Error = Box<dyn StdError + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
// The connection to the database can be placed in here.
pub struct Data {
    database: Database,
}

#[tokio::main]
async fn main() {
    let database = match Database::new().await {
        Ok(database) => database,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

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
            commands: vec![challenge(), register(), pending_matches()],
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
                tokio::spawn(check_matches(ctx.clone()));
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database })
            })
        });

    framework.run().await.unwrap();
}
