use serenity::model::prelude::Message;
use tracing::error;

/// Instead of returning an error from the command, which often isn't
/// really necessary, we print the error to the console.
pub(crate) fn check_message(result: serenity::Result<Message>) {
    if let Err(why) = result {
        error!("Message failed to send with error: {:?}", why);
    }
}
