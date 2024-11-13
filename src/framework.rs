use crate::commands;
use crate::State;

pub fn build_framework() -> poise::Framework<State, crate::Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::test::test()],
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
