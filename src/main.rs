use poise::serenity_prelude::{self as serenity, CreateSelectMenu, SelectMenu, CreateSelectMenuOption};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn challenge(
    ctx: Context<'_>,
    #[description = "Challenge selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let channel = ctx.channel_id();

    let roles = guild_id.roles(ctx).await?;
    let tews_role: Vec<&serenity::Role> = roles
        .values()
        .filter(|r| r.name == "tews-circuit")
        .collect();

    if tews_role.len() == 0 {
        channel.say(ctx, "tews-circuit role not found. This should be impossible. Please contact the developer.").await?;
        return Ok(());
    }

    if !tews_role[0].permissions.manage_roles() {
        channel
            .say(
                ctx,
                "tews-circuit does not have permissions to manage roles.",
            )
            .await?;
        return Ok(());
    }

    let user = match user {
        Some(user) => user,
        None => return Ok(()),
    };

    let notice = format!(
        "You are challenged by {}. Do you accept?",
        ctx.author().name,
    );

    user.dm(ctx, |m| m.content(notice)).await?;
    let notice = format!("{} has been notified. Please be patient.", user.name);
    ctx.author().dm(ctx, |m| m.content(notice)).await?;

    Ok(())
}

#[poise::command(context_menu_command = "User information", slash_command)]
async fn user_info(
    ctx: Context<'_>,
    #[description = "Discord profile to query information about"] user: serenity::User,
) -> Result<(), Error> {
    let mut select_menu = CreateSelectMenu::default();
    select_menu.options(|options| {
        let opt = CreateSelectMenuOption::new("Hello", "Bro");
        options.set_options(vec![opt])});

    _ = select_menu.build();
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![challenge()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
