use crate::bot::{commands::prelude::*, data::ShardManagerContainer};
use serenity::client::bridge::gateway::ShardId;

#[command]
#[description = "Retrieves the current shard's latency."]
pub(super) async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = data
        .get::<ShardManagerContainer>()
        .expect("Expected `ShardManagerContainer` in bot data.");

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            check_message(msg.reply(ctx, "No shard found.").await);
            return Ok(());
        }
    };

    let latency = match runner.latency {
        Some(latency) => latency,
        None => {
            check_message(
                msg.reply(
                    ctx,
                    "Latency could not be retrieved. Try again in a minute or two.",
                )
                .await,
            );
            return Ok(());
        }
    };

    check_message(
        msg.reply(ctx, &format!("Average shard latency is {:?}.", latency))
            .await,
    );

    Ok(())
}
