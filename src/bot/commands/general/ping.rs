use crate::bot::commands::prelude::*;

#[command]
#[description = "Pong!"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_message(msg.reply(ctx, "Pong!").await);
    Ok(())
}
