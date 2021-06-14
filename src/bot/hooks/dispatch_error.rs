use crate::utils::fmt::{format_duration, format_list};
use serenity::{
    framework::standard::{buckets::RateLimitInfo, macros::hook, DispatchError, Reason},
    model::{channel::Message, Permissions},
    prelude::Context,
};
use tracing::{error, info};

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::CheckFailed(check_name, reason) => {
            check_failed(ctx, msg, check_name, reason).await;
        }
        DispatchError::OnlyForGuilds => {
            let _ = msg.reply(ctx, "This command can't be used in DMs.").await;
        }
        DispatchError::OnlyForDM => {
            let _ = msg
                .reply(ctx, "This command can only be used in DMs.")
                .await;
        }
        DispatchError::LackingPermissions(permissions) => {
            lacking_permissions(ctx, msg, permissions).await;
        }
        DispatchError::Ratelimited(info) => {
            ratelimited(ctx, msg, info).await;
        }
        _ => {
            error!("Uncaught dispatch error: {:?}", error);
        }
    }
}

/// If a check failed, we use the reason to determine where to display the error
async fn check_failed(ctx: &Context, msg: &Message, check_name: &str, reason: Reason) {
    match reason {
        Reason::User(err_msg) => {
            let _ = msg.reply(ctx, err_msg).await;
        }
        Reason::Log(log) => {
            info!("Check '{}' failed: {}", check_name, log);
        }
        Reason::UserAndLog { user: err_msg, log } => {
            info!("Check '{}' failed: {}", check_name, log);
            let _ = msg.reply(ctx, err_msg).await;
        }
        _ => {}
    }
}

/// Send a list of the permissions the bot is missing to run the command
/// to the channel.
async fn lacking_permissions(ctx: &Context, msg: &Message, permissions: Permissions) {
    let names: Vec<_> = permissions
        .get_permission_names()
        .into_iter()
        .map(|name| format!("`{}`", name))
        .collect();

    let content = match format_list(&names[..], ", ", Some(" and ")) {
        Some(names) => format!("You lack {} permissions", names),
        None => {
            error!("User '{}' is not lacking any permissions.", msg.author);
            return;
        }
    };

    let _ = msg.reply(ctx, content).await;
}

/// Tell the caller how long it will be until they can use the command again.
async fn ratelimited(ctx: &Context, msg: &Message, info: RateLimitInfo) {
    let duration_fmt = format_duration(info.rate_limit, false);

    let _ = msg
        .reply(
            ctx,
            format!(
                "You must wait `{}` to use that command again.",
                duration_fmt
            ),
        )
        .await;
}
