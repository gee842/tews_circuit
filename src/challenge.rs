use super::*;

use poise::serenity_prelude::{self as serenity, MessageBuilder};

#[poise::command(slash_command, prefix_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "Challenge selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = match user {
        Some(user) => user,
        None => return Ok(()),
    };

    let uuid = ctx.id();

    ctx.channel_id().to_channel(&ctx).await?;
    let announcement = MessageBuilder::new()
        .push_bold_safe(user.clone())
        .push(" you have been challanged. Do you accept?")
        .build();

    // Create and send the menu.
    ctx.send(|m| {
        m.content(announcement).components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Accept")
                        .custom_id(uuid)
                })
                .create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Reject")
                        .custom_id(uuid + 1)
                })
            })
        })
    })
    .await?;

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .author_id(user.id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        let custom_id = &mci.data.custom_id;
        let accept = custom_id == &uuid.to_string();
        let reject = custom_id == &(uuid + 1).to_string();

        if accept {
            ctx.send(|m| m.content("Challenge accepted!")).await?;
        } else if reject {
            mci.message.delete(&ctx).await?;
            ctx.send(|m| m.content("The request was rejected.")).await?;

            mci.create_interaction_response(ctx, |ir| {
                ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        }
    }

    Ok(())
}
