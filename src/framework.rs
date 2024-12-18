use poise::serenity_prelude as serenity;

use crate::commands;
use crate::listeners::starboard;
use crate::Error;
use crate::State;

async fn listener(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, State, Error>,
    state: &State,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} is ready", data_about_bot.user.name);

            Ok(())
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            starboard::handle_reaction(ctx, state, add_reaction).await
        }
        serenity::FullEvent::ReactionRemove { removed_reaction } => {
            starboard::handle_reaction(ctx, state, removed_reaction).await
        }
        _ => Ok(()),
    }
}

pub fn build_framework() -> poise::Framework<State, crate::Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::test::test()],
            event_handler: |ctx, event, framework, state| {
                Box::pin(listener(ctx, event, framework, state))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State::load().await)
            })
        })
        .build()
}
