use std::time::Duration;

use super::*;
use db::Connection;

use poise::serenity_prelude::{self as serenity, CreateActionRow, MessageBuilder};
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
    create_challenge_menu(ctx, &user_challenged).await?;

    while let Some(mci) = CollectComponentInteraction::new(ctx)
        .author_id(user_challenged.id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(60 * 5))
        .await
    {
        let channel = mci.channel_id;
        let custom_id = &mci.data.custom_id;
        channel
            .say(&ctx, "The challenged user has 5 minutes to respond.")
            .await?;

        let accept = custom_id == &accept_uuid.to_string();
        let reject = custom_id == &reject_uuid.to_string();

        if accept {
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
                );
            }
        } else if reject {
            channel.say(&ctx, "The request was rejected.").await?;
        }

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(InteractionResponseType::ChannelMessageWithSource)
        })
        .await?;

        mci.message.delete(ctx).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn match_started(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: Create a select menu
    // https://github.com/serenity-rs/serenity/blob/9fc3f1180ddee5526a78c6e72deaaa9dd7be1f03/examples/e17_message_components/src/main.rs
    
    ctx.say("Looks like you're ready. Please select the user to challenge.").await?;
    todo!("Create a select menu.");
    Ok(())
}
