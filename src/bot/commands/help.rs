use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::collections::HashSet;
use tracing::{error, warn};

#[help]
#[individual_command_tip = "Use `help <command-name>` to get help with a specific command."]
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough`~~ commands are unavailable because they are limited to servers."]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough`~~ commands are unavailable because they are limited to DMs."]
#[command_not_found_text = "Could not find command: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "hide"]
#[lacking_conditions = "hide"]
#[lacking_role = "nothing"]
#[wrong_channel = "strike"]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let result =
        help_commands::with_embeds(ctx, msg, args.clone(), help_options, groups, owners.clone())
            .await;

    if result.is_none() {
        warn!("Failed to send embed help info. Attempting to send a plaintext version...");

        let result = help_commands::plain(ctx, msg, args, help_options, groups, owners).await;
        match result {
            Some(_) => warn!("Successfully send plaintext help info."),
            None => error!("Failed to send help info."),
        }
    }

    Ok(())
}
