use serenity::all::{Context, Interaction};
use super::commands;

pub async fn run(ctx: Context, interaction: Interaction) {

    // Check if the interaction is a command
    if let Interaction::Command(command) = interaction {

        // Match the command name
        match command.data.name.as_str() {
            "base" => commands::base::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        }
    }

}