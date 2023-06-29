use std::{
    io::{Error as IoError, ErrorKind},
    time::Duration,
};

use super::*;
use db::Connection;

use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateSelectMenu, CreateSelectMenuOption, MessageBuilder,
};

use serenity::{CollectComponentInteraction, InteractionResponseType};

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

    while let Some(mci) = CollectComponentInteraction::new(ctx)
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

            // TODO: This part should be modified. There should be a button for the user to press
            // then and only then their next message will be taken as the date and time of the
            // match.
            let msg = "Challenge accepted! Now, write the date and time of this match.";
            channel.say(&ctx, msg).await?;

            if let Some(answer) = user_challenged
                .await_reply(ctx)
                .timeout(Duration::from_secs(60 * 5))
                .await
            {
                let msg = format!("It is done. The challenge is on {}.", answer.content);
                channel.say(&ctx, msg).await?;

                let mut conn = Connection::new();
                conn.new_challenge(
                    &ctx.author().id.to_string(),
                    &user_challenged.id.to_string(),
                    &answer.content,
                )?;
            }
        } else if reject {
            channel.say(&ctx, "The request was rejected.").await?;
        }

        mci.message.delete(ctx).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn start_match(ctx: Context<'_>) -> Result<(), Error> {
    let caller = ctx.author().id;

    // Retrieves the caller's challenge list.
    let connection = Connection::new();
    let players = match connection.player_matches(&caller.to_string()) {
        Some(players) => players,
        None => {
            return Err(Box::new(IoError::new(
                ErrorKind::NotFound,
                "This user has challenged no one.",
            )))
        }
    };

    let mut options = vec![];
    for player in players {
        let user_id = player.parse::<u64>()?;
        let username = UserId(user_id).to_user(&ctx).await?;
        let option = CreateSelectMenuOption::new(username.name, user_id);
        options.push(option);
    }

    // Create the select menu.
    let mut select_menu = CreateSelectMenu::default();
    select_menu.custom_id(ctx.id() + 20);
    select_menu.placeholder("Select the person you will fight.");
    select_menu.options(|f| f.set_options(options));

    let mut action_row = CreateActionRow::default();
    action_row.add_select_menu(select_menu);

    // This sends the select menu
    let msg = ctx
        .channel_id()
        .send_message(&ctx, |m| {
            m.content("Select the person to be challenged today.")
                .components(|c| c.add_action_row(action_row.clone()))
        })
        .await?;

    // This part is responsible for responding to messages.
    let interaction = match msg
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60 * 5))
        .await
    {
        Some(x) => x,
        None => {
            msg.reply(&ctx, "Timed out").await?;
            return Ok(());
        }
    };

    let user_id = &interaction.data.values[0];
    let user_id = user_id.parse::<u64>()?;
    let user = UserId(user_id);

    let announcement_message = MessageBuilder::default()
        .mention(&user)
        .push(" it is time for the match. If you do not click the following button within five minutes you will be disqualified.")
        .build();

    // This doesn't actually ping the user. Will need to change the IRT
    // TODO: Update this portion of the code to send a new interaction with a new timeout.
    interaction
        .create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                // Will need to attach a button component to it.
                .interaction_response_data(|d| d.content(announcement_message))
        })
        .await?;

    Ok(())
}
