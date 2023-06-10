use chrono::prelude::*;
use std::time::Duration;

use super::*;

use poise::serenity_prelude::{self as serenity, MessageBuilder};

async fn create_challenge_menu(ctx: Context<'_>, user: &serenity::User) -> Result<(), Error> {
    let accept_uuid = ctx.id();
    let reject_uuid = accept_uuid + 1;

    ctx.channel_id().to_channel(&ctx).await?;
    let announcement = MessageBuilder::new()
        .push_bold_safe(user.clone())
        .push(" you have been challenged. Do you accept?")
        .build();

    ctx.send(|m| {
        m.content(announcement).components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Accept")
                        .custom_id(accept_uuid)
                })
                .create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Reject")
                        .custom_id(reject_uuid)
                })
            })
        })
    })
    .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "Challenge selected user"] user_challenged: serenity::User,
) -> Result<(), Error> {
    let accept_uuid = ctx.id();
    let reject_uuid = accept_uuid + 1;
    create_challenge_menu(ctx, &user_challenged).await?;

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .author_id(user_challenged.id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        let channel = mci.channel_id;
        let custom_id = &mci.data.custom_id;
        let accept = custom_id == &accept_uuid.to_string();
        let reject = custom_id == &reject_uuid.to_string();

        if accept {
            let msg = "Challenge accepted! Now, write the date and time of this match. Use whatever format you prefer.";
            channel.say(&ctx, msg).await?;
            if let Some(answer) = user_challenged
                .await_reply(ctx)
                .timeout(Duration::from_secs(10))
                .await
            {
                let msg = format!("It is done. The challenge is on {}.", answer.content);
                channel.say(&ctx, msg).await?;

                let mut conn = db::Connection::new();
                conn.new_challenge(ctx.author().id.to_string(), user_challenged.id.to_string(), "Jan 20th".to_string());
            }
        } else if reject {
            channel.say(&ctx, "The request was rejected.").await?;
        }

        mci.message.delete(ctx).await?;
        mci.create_interaction_response(ctx, |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())


}
#[poise::command(slash_command, prefix_command)]
pub async fn open_database(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let conn = db::Connection::new();

    Ok(())
}
