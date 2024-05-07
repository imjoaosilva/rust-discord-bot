use dotenv::dotenv;
use serenity::{all::GatewayIntents, Client};
use std::env;

use template_bot::handlers::event::Handler;

#[tokio::main]
async fn main() {

    // Load the .env file
    dotenv().ok();

    // Get the token from the environment    
    let token = env::var("DISCORD_TOKEN").expect("❌ - Token not found!");

    // Create the client
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("[❌] - Error creating client!");

    // Start the client
    if let Err(err) = client.start().await {
        println!("[❌] - Client error: {err:?}");
    }
}