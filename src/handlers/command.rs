use serenity::all::CreateCommand;
use super::commands;

/// Registers the commands for the bot.
///
/// This function returns a vector of `CreateCommand` objects that represent the commands to be registered.
pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        commands::base::register(),
    ]
}