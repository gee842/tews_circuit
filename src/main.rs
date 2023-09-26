mod battle;
mod history;
mod player;

mod constants;
mod db;
mod errors;

use std::{collections::HashSet, error::Error as StdError, time::Duration};

use battle::{challenge::*, start_match::*};
use history::pending_matches::pending_matches;

use constants::*;
use db::Database;

use poise::serenity_prelude::{GatewayIntents, UserId};
use tokio::time::sleep;
use tracing::{info, warn};

type Error = Box<dyn StdError + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

// User data, which is stored and accessible in all command invocations
// The connection to the database can be placed in here.
pub struct Data {
    database: Database,
}

async fn routine(db: Database) {
    loop {
        sleep(Duration::from_secs(60 * 5)).await;
        let _ = db.disqualify().await;
    }
}

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database = match Database::new().await {
        Ok(database) => database,
        Err(e) => {
            info!("{}", e.to_string());
            return;
        }
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut owners = HashSet::new();
    owners.insert(UserId(ALPHABETS));
    owners.insert(UserId(ALPHACOMS));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![challenge(), register(), pending_matches(), start_match()],
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
            info!("Tews is online.");
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                tokio::spawn(routine(database.clone()));
                Ok(Data { database })
            })
        });

    framework.run().await.unwrap();
}
