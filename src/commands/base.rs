use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionResponseFlags, Timestamp};

pub async fn run(ctx: Context, command: CommandInteraction) {

  // Create an embed
  let embed = CreateEmbed::default()
    .title("My Embed")
    .timestamp(Timestamp::now());

  // Create an interaction response message with the embed and set the flags to EPHEMERAL
  let data = CreateInteractionResponseMessage::new().embed(embed).flags(
    InteractionResponseFlags::EPHEMERAL
  );
  
  // Create an interaction response with the message data
  let builder = CreateInteractionResponse::Message(data);
  command.create_response(&ctx.http, builder).await.unwrap();


}

// Register the command
pub fn register() -> CreateCommand {
    CreateCommand::new("base").description("A base command")
}