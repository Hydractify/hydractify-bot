use poise::serenity_prelude::{Channel, Mentionable};

#[poise::command(slash_command, subcommands("starboard"))]
pub async fn configuration(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn starboard(
    ctx: crate::Context<'_>,
    #[description = "The channel to serve as starboard for the server."] channel: Channel,
) -> Result<(), crate::Error> {
    if ctx.guild().is_none() {
        return Ok(());
    }

    let guild_id: i64 = ctx.guild_id().unwrap().into();
    let channel_id: i64 = channel.id().into();

    sqlx::query!(
        "INSERT INTO server_configuration AS sc (guild_id, starboard_channel) VALUES ($1, $2)
        ON CONFLICT (guild_id) DO UPDATE SET starboard_channel = $2 WHERE sc.guild_id = $1",
        guild_id,
        channel_id
    )
    .execute(&ctx.data().database)
    .await?;

    ctx.say(format!(
        "The starboard channel has been successfully updated to be {}.",
        channel.mention()
    ))
    .await?;

    Ok(())
}
