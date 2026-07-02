use poise::serenity_prelude::RoleId;
use tokio::process::Command;

#[poise::command(slash_command, subcommands("admin"))]
pub async fn unturned(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn admin(
    ctx: crate::Context<'_>,
    #[description = "The SteamID or in-game username of who to give admin to."] user: String,
) -> Result<(), crate::Error> {
    if ctx.guild().is_none() {
        return Ok(());
    }

    if let Some(member) = ctx.author_member().await {
        if !member.roles.contains(&RoleId::new(1257798842645086249)) {
            ctx.say("You have insufficient permissions to run this command.")
                .await?;
            return Ok(());
        }
    }

    let output = Command::new("tmux")
        .args(&[
            "-S",
            "/run/user/1000/tmux-1000/unturned",
            "send-keys",
            format!("Admin {user}").as_str(),
            "Enter",
        ])
        .output()
        .await?;

    print!("stderr: {:?}", output.stderr);

    ctx.say(format!("Successfully gave **{user}** admin permissions."))
        .await?;

    Ok(())
}
