use std::{
    io::{Error as IoError, ErrorKind},
    time::Duration,
};

use super::*;

use poise::serenity_prelude::{
    self as serenity, CollectComponentInteraction, CreateActionRow, MessageBuilder,
    Context as SContext
};

async fn create_challenge_menu(ctx: Context<'_>, user: &serenity::User) -> Result<(), Error> {
    let accept_uuid = ctx.id();
    let reject_uuid = accept_uuid + 1;

    ctx.channel_id().to_channel(&ctx).await?;
    let announcement = MessageBuilder::new()
        .push_bold_safe(user.clone())
        .push(" you have been challenged. Do you accept?")
        .build();

    let mut action_row = CreateActionRow::default();
    let accept = create_button(serenity::ButtonStyle::Primary, "Accept", accept_uuid);
    let reject = create_button(serenity::ButtonStyle::Primary, "Reject", reject_uuid);
    action_row.add_button(accept);
    action_row.add_button(reject);

    ctx.send(|m| {
        m.content(announcement)
            .components(|c| c.add_action_row(action_row))
    })
    .await?;

    Ok(())
}

/// - `user`: User to challenge.
#[poise::command(slash_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "Challenge selected user"] user_challenged: serenity::User,
) -> Result<(), Error> {
    let accept_uuid = ctx.id();
    let reject_uuid = accept_uuid + 1;
    if &ctx.author().id == &user_challenged.id {
        ctx.say("You can't challenge yourself.").await?;
        return Ok(());
    };

    create_challenge_menu(ctx, &user_challenged).await?;

    while let Some(mci) = CollectComponentInteraction::new(&ctx)
        .author_id(user_challenged.id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(60 * 5))
        .await
    {
        // Gets discord to wait 15 minutes for a response from the
        // challenged user. This automatically creates an interaction
        // response by editing the interaction itself, which means you don't
        // need to handle mci.create_interaction_response.
        mci.defer(ctx).await?;

        let channel = mci.channel_id;
        let custom_id = &mci.data.custom_id;

        let accept = custom_id == &accept_uuid.to_string();
        let reject = custom_id == &reject_uuid.to_string();

        if accept {
            channel
                .say(&ctx, "The challenged user has 5 minutes to respond.")
                .await?;

            let msg = "Challenge accepted! The challenged user will need to write the date and time of the match.
                Accepted date formats are as follows:
                - 8 Jul 2021 15:00\n- 9 Apr 2023 20:00\n- 1 Jan 2024 18:30";
            channel.say(&ctx, msg).await?;

            if let Some(answer) = user_challenged
                .await_reply(ctx)
                .timeout(Duration::from_secs(60 * 5))
                .await
            {
                let mut conn = ctx.data().database.clone();
                conn.new_challenge(
                    &ctx.author().id.to_string(),
                    &user_challenged.id.to_string(),
                    &answer.content,
                )
                .await?;

                let msg = format!("It is done. The challenge is on {}.", answer.content);
                channel.say(&ctx, msg).await?;
            }
        } else if reject {
            channel.say(&ctx, "The request was rejected.").await?;
        }

        mci.message.delete(ctx).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn pending_matches(ctx: Context<'_>) -> Result<(), Error> {
    let caller = ctx.author().id;

    // Retrieves the caller's challenge list.
    let connection = ctx.data().database.clone();
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
    let msg = options.join("\n");
    dm.say(&ctx, msg).await?;
    Ok(())
}

/// Checks the database every five minutes then alerts users when
/// time for a match is near.
pub async fn check_matches(ctx: SContext) -> Result<(), IoError> {
    let mut timer = tokio::time::interval(Duration::from_secs(60 * 5));
    let connection = match Database::new().await {
        Ok(connection) => connection,
        Err(e) => return Err(IoError::new(ErrorKind::NotFound, e.to_string())),
    };

    loop {
        connection.time_for_match().await;
        timer.tick().await;
    }
}

// This will be automatically called when it is time for the match.
// #[poise::command(slash_command)]
// pub async fn start_match(ctx: Context<'_>) -> Result<(), Error> {
//     // https://github.com/serenity-rs/serenity/blob/current/examples/e17_message_components/src/main.rs#L72
//     let caller = ctx.author().id;

//     Ok(())
// }
