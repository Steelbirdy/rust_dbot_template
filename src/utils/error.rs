use serenity::model::prelude::Message;
use tracing::error;

pub(crate) fn check_message(result: serenity::Result<Message>) {
    if let Err(why) = result {
        error!("Message failed to send with error: {:?}", why);
    }
}
