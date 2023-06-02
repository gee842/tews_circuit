use super::*;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

pub async fn role_management(ctx: Context<'_>) {
    let guild_id = ctx.guild_id().unwrap();
    let channel = ctx.channel_id();

    let roles = guild_id.roles(ctx).await.unwrap();
    let tews_role: Vec<&serenity::Role> = roles
        .values()
        .filter(|r| r.name == "tews-circuit")
        .collect();

    if tews_role.len() == 0 {
        channel.say(ctx, "tews-circuit role not found. This should be impossible. Please contact the developer.").await.unwrap();
    }

    if !tews_role[0].permissions.manage_roles() {
        channel
            .say(
                ctx,
                "tews-circuit does not have permissions to manage roles.",
            )
            .await.unwrap();
    }
}

// Example for creating a select menu is here.
#[poise::command(slash_command, prefix_command)]
pub async fn foo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Executing").await?;

    let uuid_boop = ctx.id();

    // Create and send the menu.
    ctx.send(|m| {
        m.content("The current scoreboard").components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Boop me!")
                        .custom_id(uuid_boop)
                })
            })
        })
    })
    .await?;

    // The responsive part
    let mut boop_count = 0;
    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid_boop.to_string())
        .await
    {
        boop_count += 1;

        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| m.content(format!("Boop count: {}", boop_count)))
            .await?;

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}
