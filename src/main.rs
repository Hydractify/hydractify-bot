mod commands {
    pub mod test;
}
mod database;
mod framework;
mod state;

use poise::serenity_prelude as serenity;
use state::State;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework::build_framework())
        .await;

    client.unwrap().start().await.unwrap();
}
