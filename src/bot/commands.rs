mod general;
mod help;
mod owner;

pub(crate) use help::*;

use general::*;
use owner::*;

pub(crate) mod prelude {
    pub(crate) use crate::utils::error::check_message;
    pub(crate) use serenity::{
        framework::standard::{macros::command, CommandResult},
        model::prelude::*,
        prelude::*,
    };
}

use serenity::framework::standard::{macros::group, StandardFramework};

#[group]
#[sub_groups(General, Owner)]
struct All;

pub(crate) fn add_all_groups(mut framework: StandardFramework) -> StandardFramework {
    for group in ALL_GROUP_OPTIONS.sub_groups {
        framework = framework.group(group);
    }
    framework
}
