mod bot;
mod utils;

use bot::{commands::*, data::ShardManagerContainer, handler::Handler, hooks};

use serenity::{framework::standard::StandardFramework, http::Http, prelude::*};
use std::{env, error::Error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::from_filename(".env").expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect(
        "Expected an environment variable named `DISCORD_TOKEN` containing the bot client's token",
    );

    let http = Http::new_with_token(&token);

    let (id, owners) = match http.get_current_application_info().await {
        Ok(info) => {
            let id = info.id;
            let owners = vec![info.owner.id].into_iter().collect();
            (id, owners)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let prefix = env::var("COMMAND_PREFIX")
        .expect("Expected an environment variable named `COMMAND_PREFIX` containing the bot's command prefix");

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix).on_mention(Some(id)))
        .after(hooks::after)
        .on_dispatch_error(hooks::dispatch_error)
        .help(&MY_HELP);
    let framework = add_all_groups(framework);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    client.start().await?;

    Ok(())
}
