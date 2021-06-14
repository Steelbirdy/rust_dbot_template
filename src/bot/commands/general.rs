mod latency;
mod ping;

use latency::*;
use ping::*;

use serenity::framework::standard::macros::group;

/// Miscellaneous useful commands
#[group]
#[commands(latency, ping)]
struct General;
