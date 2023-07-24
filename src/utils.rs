use super::*;

use poise::serenity_prelude::{ButtonStyle, CreateButton};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

pub fn create_button(style: ButtonStyle, label: &str, custom_id: u64) -> CreateButton {
    CreateButton::default()
        .style(style)
        .label(label)
        .custom_id(custom_id)
        .clone()
}

// pub fn 
