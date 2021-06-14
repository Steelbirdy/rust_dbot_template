use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::ResumedEvent,
        gateway::{Activity, Ready},
        id::GuildId,
    },
};
use std::env;
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _: Context, guilds: Vec<GuildId>) {
        info!("Cache is ready! Found {} guilds", guilds.len());
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let prefix = env::var("COMMAND_PREFIX")
            .expect("Expected an environment variable named `COMMAND_PREFIX` containing the bot's command prefix");

        let activity_str = format!("{}help", prefix);
        ctx.set_activity(Activity::playing(activity_str)).await;

        info!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
