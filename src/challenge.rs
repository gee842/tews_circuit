use poise::serenity_prelude as serenity;

use super::Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn challenge(
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
