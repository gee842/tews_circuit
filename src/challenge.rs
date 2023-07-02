use std::time::Duration;

use super::*;

use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateActionRow, CreateButton, CreateSelectMenu,
    CreateSelectMenuOption, MessageBuilder,
};

use serenity::CollectComponentInteraction;

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

                let mut conn = ctx.data().database.clone();
                conn.new_challenge(
                    &ctx.author().id.to_string(),
                    &user_challenged.id.to_string(),
                    &answer.content,
                )
                .await?;
            }
        } else if reject {
            channel.say(&ctx, "The request was rejected.").await?;
        }

        mci.message.delete(ctx).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn my_pending_matches(ctx: Context<'_>) -> Result<(), Error> {
    let caller = ctx.author().id;

    // Retrieves the caller's challenge list.
    let connection = ctx.data().database.clone();
    let matches = connection.player_matches(&caller.to_string()).await?;

    if matches.is_empty() {
        // Propagate the error
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

        let option = CreateSelectMenuOption::new(label.clone(), label);
        options.push(option);
    }

    let mut select_menu = CreateSelectMenu::default();
    select_menu.custom_id(ctx.id() + 50); // Will need add an actual custom id.
    select_menu.options(|f| f.set_options(options));

    let mut action_row = CreateActionRow::default();
    action_row.add_select_menu(select_menu);

    let main_message = "Click to view a list of people you're set to fight.";
    // This sends the select menu
    let msg = ctx
        .channel_id()
        .send_message(&ctx, |m| {
            m.content(main_message)
                .components(|c| c.add_action_row(action_row.clone()))
        })
        .await?;

    let interaction = match msg
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60 * 5))
        .await
    {
        Some(interaction) => interaction,
        None => return Ok(()),
    };

    interaction
        .create_interaction_response(&ctx, |r| {
            r.kind(serenity::InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| d.content(main_message))
        })
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn start_match(ctx: Context<'_>) -> Result<(), Error> {
    // https://github.com/serenity-rs/serenity/blob/current/examples/e17_message_components/src/main.rs#L72
    let caller = ctx.author().id;

    Ok(())
}
