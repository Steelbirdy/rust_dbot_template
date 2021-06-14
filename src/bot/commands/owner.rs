mod quit;

use quit::*;

use serenity::framework::standard::macros::group;

/// Commands for debugging and management purposes.
#[group]
#[commands(quit)]
#[owners_only]
struct Owner;
