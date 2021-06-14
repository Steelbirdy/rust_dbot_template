use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use tracing::error;

/// If the command returned an error, display it in the console
#[hook]
pub(crate) async fn after(
    _: &Context,
    _: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(why) = command_result {
        error!("Command `{}` returned error: {:?}", command_name, why);
    }
}
