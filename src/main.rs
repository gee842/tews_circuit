use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, prelude::Role},
    prelude::*,
};

#[group]
#[commands(challenge)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
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

/// Will challenge another discord user. A user cannot be challenged
/// if it has the "On Hiatus" role, or is already challenged by someone
/// else.
#[command]
async fn challenge(ctx: &Context, msg: &Message) -> CommandResult {
    if let None = msg.guild_id {
        msg.channel_id.say(ctx, "No.").await?;
        return Ok(());
    };

    // Checks if tews has the right perms.
    let guild_id = msg.guild_id.unwrap();
    let roles = guild_id.roles(ctx).await?;

    let tews_role: Vec<&Role> = roles
        .values()
        .filter(|r| r.name == "tews-circuit")
        .collect();

    if tews_role.len() == 0 {
        msg.channel_id.say(ctx, "tews-circuit role not found. This should be impossible. Please contact the developer.").await?;
        return Ok(());
    }

    if !tews_role[0].permissions.manage_roles() {
        msg.channel_id
            .say(
                ctx,
                "tews-circuit does not have permissions to manage roles.",
            )
            .await?;
        return Ok(());
    }

    let mentions = msg.mentions.len();
    if mentions > 1 {
        msg.channel_id
            .say(ctx, "You may only challenge one person at a time.")
            .await?;

        return Ok(());
    } else if mentions == 0 {
        msg.channel_id
            .say(ctx, "You can't challenge no one.")
            .await?;

        return Ok(());
    }

    let user_to_challenge = &msg.mentions[0];
    let notice = format!(
        "You have a challenger! You are challenged by {}.",
        msg.author
    );

    user_to_challenge
        .direct_message(&ctx, |m| m.content(notice))
        .await?;

    let notice = format!(
        "{} has been notified of this challenge.",
        user_to_challenge.name
    );
    msg.reply(&ctx, notice).await?;

    Ok(())
}
