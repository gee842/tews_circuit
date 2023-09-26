use crate::*;

#[poise::command(slash_command)]
pub async fn pending_matches(ctx: Context<'_>) -> Result<(), Error> {
    let connection = ctx.data().database.clone();

    let caller = ctx.author().id;

    // Retrieves the caller's pending matches.
    let matches = connection.player_matches(&caller.to_string()).await?;

    let dm = caller.create_dm_channel(&ctx).await?;
    if matches.is_empty() {
        dm.say(&ctx, "You have no pending matches.").await?;
        return Ok(());
    }

    // Create options for select menu.
    let mut options = vec![];
    for info in matches {
        let user_id = info.0;
        let user_id = user_id.parse::<u64>()?;
        let username = UserId(user_id).to_user(&ctx).await?;

        let time = info.1;
        let label = format!("Vs. {} on {}", username.name, time);

        options.push(label);
    }

    let dm = caller.create_dm_channel(&ctx).await?;
    dm.say(&ctx, options.join("\n")).await?;
    ctx.say("Your list of pending matches are sent via dm.")
        .await?;

    Ok(())
}
