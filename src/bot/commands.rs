mod general;
mod help;
mod owner;

pub(crate) use help::*;

use general::*;
use owner::*;

/// Has everything you need to get started with writing commands
pub(crate) mod prelude {
    pub(crate) use crate::utils::error::check_message;
    pub(crate) use serenity::{
        framework::standard::{macros::command, CommandResult},
        model::prelude::*,
        prelude::*,
    };
}

use serenity::framework::standard::{macros::group, StandardFramework};

/// Super-group that encompasses all the other groups for ease
/// of adding them to the bot
#[group]
#[sub_groups(General, Owner)]
struct All;

/// Add all sub-groups of `All` to the bot framework.
/// This should be called in `main`.
pub(crate) fn add_all_groups(mut framework: StandardFramework) -> StandardFramework {
    for group in ALL_GROUP_OPTIONS.sub_groups {
        framework = framework.group(group);
    }
    framework
}
