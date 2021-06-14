mod quit;

use quit::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(quit)]
#[owners_only]
struct Owner;
