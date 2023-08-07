use super::*;

use std::{iter::repeat, str::FromStr, time::Duration};

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};

use poise::serenity_prelude::{
    ButtonStyle, ChannelId, CollectComponentInteraction, CreateActionRow, CreateButton,
    MessageBuilder, ScheduledEventType,
};

async fn create_challenge_menu(
    ctx: Context<'_>,
    accept_uuid: u64,
    reject_uuid: u64,
    user: &serenity::User,
) -> Result<(), Error> {
    ctx.channel_id().to_channel(&ctx).await?;
    let announcement = MessageBuilder::new()
        .push_bold_safe(user.clone())
        .push(" you have been challenged. Do you accept?")
        .build();

    let mut action_row = CreateActionRow::default();

    let accept = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label("Accept")
        .custom_id(accept_uuid)
        .clone();

    let reject = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label("Reject")
        .custom_id(reject_uuid)
        .clone();

    action_row.add_button(accept);
    action_row.add_button(reject);

    ctx.send(|m| {
        m.content(announcement)
            .components(|c| c.add_action_row(action_row))
    })
    .await?;

    Ok(())
}

/// A slash command to challenger another user.
/// - `user`: User to challenge.
#[poise::command(slash_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "Challenge selected user"] user_challenged: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;

    let accept_uuid = ctx.id();
    let reject_uuid = accept_uuid + 1;
    if &ctx.author().id == &user_challenged.id {
        ctx.say("You can't challenge yourself.").await?;
        return Ok(());
    };

    create_challenge_menu(ctx, accept_uuid, reject_uuid, &user_challenged).await?;

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

            let msg = "
            Challenge accepted! The challenged user will need to write the date and time of the match. Accepted date formats are as follows:
                - 8 Jul 2021 15:00\n- 9 Apr 2023 20:00\n- 1 Jan 2024 18:30";

            channel.say(&ctx, msg).await?;

            if let Some(answer) = user_challenged
                .await_reply(ctx)
                .timeout(Duration::from_secs(60 * 5))
                .await
            {
                let mut conn = ctx.data().database.clone();
                conn.add_new_challenge(
                    &ctx.author().id.to_string(),
                    &user_challenged.id.to_string(),
                    &answer.content,
                    None,
                )
                .await?;

                let msg = format!("It is done. The challenge is on {}. A public event is created to help you keep track of the time of the challenge.", answer.content);
                ctx.say(msg).await?;
            }
        } else if reject {
            ctx.say("The request was rejected.").await?;
        }

        mci.message.reply(&ctx, "The command has finished executing.").await?;
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
    dm.say(&ctx, options.join("\n")).await?;
    ctx.say("Your list of pending matches are sent via dm.")
        .await?;

    Ok(())
}
