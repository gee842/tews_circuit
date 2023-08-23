use super::*;
use rank::Rank;

use std::time::Duration;

use poise::serenity_prelude::{
    ButtonStyle, CollectComponentInteraction, CreateActionRow, CreateButton, MessageBuilder,
};

async fn create_challenge_menu(
    ctx: Context<'_>,
    accept_uuid: u64,
    reject_uuid: u64,
    user: &serenity::User,
) -> Result<(), Error> {
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

async fn ongoing_match_menu(
    ctx: Context<'_>,
    player_one_id: u64,
    player_two_id: u64,
    player_one_name: String,
    player_two_name: String,
) -> Result<(), Error> {
    let mut action_row = CreateActionRow::default();

    let player_one_wins = format!("{} wins!", player_one_name);
    let player_one_button = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label(player_one_wins)
        .custom_id(player_one_id)
        .clone();

    let player_two_wins = format!("{} wins!", player_two_name);
    let player_two_button = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label(player_two_wins)
        .custom_id(player_two_id)
        .clone();

    action_row.add_button(player_one_button);
    action_row.add_button(player_two_button);

    ctx.send(|m| m.content("").components(|c| c.add_action_row(action_row)))
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "User to challenge"] user_challenged: serenity::User,
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

        mci.message
            .reply(&ctx, "The command has finished executing.")
            .await?;
    }

    Ok(())
}
#[poise::command(slash_command)]
pub async fn start_match(ctx: Context<'_>) -> Result<(), Error> {
    let caller_id = ctx.author().id;
    let database = ctx.data().database.clone();
    let other_player = database
        .closest_matches(&caller_id.as_u64().to_string())
        .await?;

    let other_player = UserId(other_player.parse().unwrap());

    // Ping both users that a match has started.
    // convert user id to their names
    let caller_name = caller_id.to_user(&ctx).await?;
    let other_player_name = other_player.to_user(&ctx).await?;

    let mut msg = MessageBuilder::new()
        .push("A match has started between ")
        .push_bold_safe(caller_name.clone())
        .push(" and ")
        .push_bold_safe(other_player_name.clone())
        .push(". When the match has concluded, please select the winner.")
        .build();

    ctx.channel_id().say(&ctx, msg).await?;

    // When the match has finished get them to confirm who wins/loses.
    ongoing_match_menu(
        ctx,
        caller_id.0,
        other_player.0,
        caller_name.name,
        other_player_name.name,
    )
    .await?;

    let conn = ctx.data().database.clone();

    while let Some(mci) = CollectComponentInteraction::new(&ctx)
        .channel_id(ctx.channel_id())
        .await
    {
        let winner_id: u64 = mci.data.custom_id.parse().unwrap();
        let winner_name = UserId(winner_id).to_user(&ctx).await?;

        let loser = if winner_id == caller_id.0 {
            other_player.0
        } else {
            caller_id.0
        };

        let loser_name = UserId(loser).to_user(&ctx).await?;

        // Check point totals
        // TODO: Each player should be its own struct.
        let winner_points = conn.points_data(&winner_id.to_string()).await?;
        let winner_rank = Rank::from(winner_points);

        let loser_points = conn.points_data(&loser.to_string()).await?;
        let loser_rank = Rank::from(loser_points);

        info!("Winner rank: {}", winner_rank);
        info!("Loser rank: {}", loser_rank);

        // Me: Gold, Oppo: Gold
        let (winner_new_points, loser_new_points) = if winner_rank == loser_rank {
            (winner_points + 25, loser_points - 25)
        } else if winner_rank > loser_rank {
            (winner_points + 10, loser_points - 15)
        } else {
            // Winner rank less than loser rank
            (winner_points + 25, loser_points - 30)
        };

        let new_points = format!(
            "\n{}: {} -> {}\n{}: {} -> {}",
            winner_name,
            winner_points,
            winner_new_points,
            loser_name,
            loser_points,
            loser_new_points
        );

        msg = MessageBuilder::new()
            .push("The winner is ")
            .mention(&winner_name)
            .push(new_points)
            .build();

        ctx.say(msg).await?;

        mci.message
            .reply(&ctx, "The command has finished executing.")
            .await?;
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
