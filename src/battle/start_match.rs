use std::sync::Arc;

use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton};
use poise::serenity_prelude::{CacheHttp, CollectComponentInteraction, MessageBuilder};
use poise::serenity_prelude::{MessageComponentInteraction, User};

use crate::*;
use player::Player;

async fn ongoing_match_menu(
    ctx: Context<'_>,
    player_one: User,
    player_two: User,
) -> Result<(), Error> {
    let mut action_row = CreateActionRow::default();

    let player_one_wins = format!("{} wins!", player_one.name);
    let player_one_button = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label(player_one_wins)
        .custom_id(player_one.id)
        .clone();

    let player_two_wins = format!("{} wins!", player_two.name);
    let player_two_button = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label(player_two_wins)
        .custom_id(player_two.id)
        .clone();

    action_row.add_button(player_one_button);
    action_row.add_button(player_two_button);

    ctx.send(|m| m.content("").components(|c| c.add_action_row(action_row)))
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn start_match(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().database.clone();

    // The person who ran the `start_match` command.
    let caller = ctx.author();

    let (other_player, date) = match db.closest_matches(&caller.id.to_string()).await {
        Ok(other_player) => other_player,
        Err(_) => {
            ctx.say("You don't have any pending matches.").await?;
            return Ok(());
        }
    };

    let http = ctx.http();
    let other_player = http.get_user(other_player.parse().unwrap()).await?;

    let mut msg = MessageBuilder::new()
        .push("A match has started between ")
        .push_bold_safe(caller.clone())
        .push(" and ")
        .push_bold_safe(other_player.clone())
        .push(". When the match has concluded, please select the winner.")
        .build();

    ctx.channel_id().say(&ctx, msg).await?;

    // When the match has finished get them to confirm who wins/loses.
    ongoing_match_menu(ctx, caller.clone(), other_player.clone()).await?;

    while let Some(mci) = CollectComponentInteraction::new(ctx)
        .channel_id(ctx.channel_id())
        .await
    {
        mci.defer(ctx).await?;

        let winner_id: u64 = mci.data.custom_id.parse().unwrap();
        let winner_points = db.points_data(winner_id).await?;
        let winner = ctx.http().get_user(winner_id).await?;

        let mut winner = Player::new(winner, winner_points).await;

        let loser = if winner.id() == caller.id.0.to_string() {
            other_player.clone()
        } else {
            caller.clone()
        };

        let loser_points = db.points_data(loser.id.0).await?;
        let mut loser = Player::new(loser, loser_points).await;

        // Calculate new point total
        let (add, minus) = if winner.rank == loser.rank {
            (25, 25)
        } else if winner.rank > loser.rank {
            (10, 15)
        } else {
            // Winner rank less than loser rank
            (25, 30)
        };

        let winner_ori_rank = winner.rank.clone();
        let loser_ori_rank = loser.rank.clone();

        let winner_new_points = winner.add(add, &db).await?;
        let winner_rank_status = winner_ori_rank.current_status(&winner.rank);
        db.update_points(winner_new_points, winner.id()).await?;

        let loser_new_points = loser.minus(minus, &db).await?;
        let loser_rank_status = loser_ori_rank.current_status(&loser.rank);
        db.update_points(loser_new_points, loser.id()).await?;

        let winner_id = winner.id();
        let loser_id = loser.id();

        winner.mark_win(&db).await?;
        loser.mark_loss(&db).await?;

        db.match_finished(&winner_id, &loser_id, &date).await?;

        let winner_msg = format!(
            "Winner: {}, {winner_points} -> {winner_new_points}. {winner_rank_status}",
            winner.name()
        );

        let loser_msg = format!(
            "Loser: {}, {loser_points} -> {loser_new_points}. {loser_rank_status}",
            loser.name(),
        );

        let final_msg = format!("\n{}\n{}", winner_msg, loser_msg);

        info!("{}", final_msg);

        msg = MessageBuilder::new()
            .push("The winner is ")
            .mention(&winner.user())
            .push(final_msg)
            .build();

        ctx.say(msg).await?;

        mci.message
            .reply(&ctx, "The command has finished executing.")
            .await?;
    }

    Ok(())
}
