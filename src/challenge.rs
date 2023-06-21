use std::time::Duration;

use super::*;
use db::Connection;

use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateSelectMenu, CreateSelectMenuOption, MessageBuilder,
};
use serenity::{CollectComponentInteraction, InteractionResponseType};
use serenity::futures::StreamExt;

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
pub async fn match_started(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: Create a select menu
    // https://github.com/serenity-rs/serenity/blob/9fc3f1180ddee5526a78c6e72deaaa9dd7be1f03/examples/e17_message_components/src/main.rs
    let caller = ctx.author().id;

    let mut opts = CreateSelectMenuOption::new("Commissions", "Comissions");
    opts.description("Hello, what's up dude.");

    let mut select_menu = CreateSelectMenu::default();
    select_menu.custom_id(ctx.id() + 20);
    select_menu.placeholder("Choose a user.");
    select_menu.options(|options| options.add_option(opts));

    // Unsure of how to get message.
    while let Some(mci) = CollectComponentInteraction::new(ctx)
        .author_id(caller)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(60 * 5))
        .await
    {
        // mci.defer(ctx).await?;
        let msg = mci.message.clone();
        let mut interaction_stream = msg
            .await_component_interactions(&ctx)
            .timeout(Duration::from_secs(60 * 3))
            .build();

        while let Some(interaction) = interaction_stream.next().await {
            // Acknowledge the interaction and send a reply
            interaction
                .create_interaction_response(&ctx, |r| {
                    // This time we dont edit the message but reply to it
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            d.ephemeral(true)
                                .content("Hi")
                        })
                })
                .await
                .unwrap();
        }
    }

    Ok(())
}
