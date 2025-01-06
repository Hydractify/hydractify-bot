mod commands {
    pub mod test;
}
mod listeners {
    pub mod starboard;
}
mod database;
mod framework;
mod state;

#[cfg(test)]
mod tests;

use std::fs;

use poise::serenity_prelude as serenity;
use serde::Deserialize;
use state::State;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[derive(Deserialize)]
struct Configuration {
    database_url: String,
    discord_token: String,
    star_threshold: usize,
}

#[tokio::main]
async fn main() {
    let config: Configuration;

    match fs::read_to_string("./config.toml") {
        Ok(content) => config = toml::from_str(&content).unwrap(),
        Err(_) => panic!("There is no 'config.toml' in this path."),
    }

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(&config.discord_token, intents)
        .framework(framework::build_framework(config))
        .await;

    client.unwrap().start().await.unwrap();
}
