use serenity::all::Interaction;
use serenity::all::Ready;
use serenity::prelude::*;
use serenity::async_trait;
use super::events;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Handles the `ready` event.
    ///
    /// This function is called when the bot is ready to start receiving events.
    /// It takes a `Context` and a `Ready` struct as parameters.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the event.
    /// * `interaction` - The `Ready` struct representing the ready event.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn ready(&self, ctx: Context, interaction: Ready) {
    ///     events::ready::run(ctx, interaction).await;
    /// }
    /// ```
    async fn ready(&self, ctx: Context, interaction: Ready) {
        events::ready::run(ctx, interaction).await;
    }

    /// Handles the `interaction_create` event.
    ///
    /// This function is called when a user interacts with the bot.
    /// It takes a `Context` and an `Interaction` struct as parameters.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the event.
    /// * `interaction` - The `Interaction` struct representing the interaction event.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    ///     events::interaction_create::run(ctx, interaction).await;
    /// }
    /// ```
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction_create::run(ctx, interaction).await;
    }
}