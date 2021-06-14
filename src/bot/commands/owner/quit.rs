use crate::bot::{commands::prelude::*, data::ShardManagerContainer};

#[command]
#[aliases("shutdown")]
#[description = "Shuts down the bot."]
pub(super) async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        check_message(msg.reply(ctx, "Shutting down!").await);
        manager.lock().await.shutdown_all().await;
    } else {
        check_message(
            msg.reply(ctx, "There was a problem getting to the shard manager.")
                .await,
        );
    }

    Ok(())
}
