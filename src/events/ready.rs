use serenity::all::{Context, GuildId};
use std::env;
use super::handlers::command::register_commands;

/// Runs the logic when the bot is ready to start.
///
/// # Arguments
///
/// * `ctx` - The context of the bot.
/// * `ready` - The `Ready` event containing information about the bot's readiness.
pub async fn run(ctx: Context, ready: serenity::all::Ready) {

    // Get the guild ID from the environment
    // The ID of the guild.
    let guild_id = env::var("GUILD_ID")
        .expect("❌ - Guild ID not found!")
        .parse()
        .expect("❌ - Guild ID must be an integer");

    // Create a new GuildId
    let guild = GuildId::new(guild_id);

    match guild
        .set_commands(&ctx.http, register_commands())
        .await
    {
        Ok(list) => println!("☑️  - {} Commands loaded!", list.len()),
        Err(_) => println!("❌ - Unable to load commands!"),
    };


    println!("✅ - {} started successfully!", ready.user.name);
}
