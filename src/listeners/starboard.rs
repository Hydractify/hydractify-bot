use poise::serenity_prelude::{
    self as serenity, ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage,
    Mentionable, Message, MessageId, Reaction, UserId,
};

use crate::{state::State, Error};

/// Creates the template message for a new starboard entry in the server.
/// It only returns the builder required to pass as an argument to `<GuildChannel>.send_message`.
fn create_message(original_message: Message, stars: i32) -> CreateMessage {
    let image = if let Some(attachment) = original_message.attachments.first() {
        &attachment.url
    } else if let Some(embed) = original_message.embeds.first() {
        if let Some(image) = &embed.image {
            &image.url
        } else if let Some(image) = &embed.thumbnail {
            &image.url
        } else {
            ""
        }
    } else {
        ""
    };

    let embed = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(&original_message.author.name).icon_url(
                original_message
                    .author
                    .avatar_url()
                    .unwrap_or(String::from("")),
            ),
        )
        .field("Message", &original_message.content, false)
        .color(0xffcf05)
        .description(format!("[Original]({})", original_message.link()))
        .image(image);

    CreateMessage::new().embed(embed).content(format!(
        "**{}**🌟『{}』",
        stars,
        original_message.channel_id.mention()
    ))
}

pub async fn handle_reaction(
    ctx: &serenity::Context,
    state: &State,
    reaction: &Reaction,
) -> Result<(), Error> {
    // If `None` it means that we are not receiving from a Guild.
    if reaction.guild_id.is_none() {
        return Ok(());
    }

    let user_from_reaction = reaction.user(&ctx.http).await?;

    // We don't care about bot's reactions.
    if user_from_reaction.bot {
        return Ok(());
    }

    // Hardcoded for now, should be configurable.
    let valid_emotes = ["⭐", "✨", "🌠", "🌟", "<a:a_kirbyStar:894087344909606912>"];

    // If the reaction is not in the list of valid ones, don't process
    if !valid_emotes
        .iter()
        .any(|e| *e == reaction.emoji.to_string())
    {
        return Ok(());
    }

    let message = reaction.message(&ctx.http).await?;

    // If the author is trying to star react himself, deny it.
    if message.author.id == user_from_reaction.id {
        return Ok(());
    }

    // The UserID of each user that has reacted with a valid reaction.
    let mut userids: Vec<UserId> = Vec::new();

    for reac in &message.reactions {
        if valid_emotes
            .iter()
            .any(|e| *e == reac.reaction_type.to_string())
        {
            let mut users: Vec<UserId> = reaction
                .users(&ctx.http, reac.reaction_type.clone(), None, None::<UserId>)
                .await?
                .iter()
                .filter_map(|user| {
                    // Ignore reactions from bots and the author of the message.
                    if user.bot || message.author.id == user.id {
                        None
                    } else {
                        Some(user.id)
                    }
                })
                .collect();

            userids.append(&mut users);
        }
    }

    // Remove multiple entries of the same person,
    // you can use multiple star reactions.
    userids.dedup();

    // Get the message ID and turn it onto a BIGINT equivalent for the query.
    let message_id: i64 = message.id.into();

    let starboard = sqlx::query!(
        "INSERT INTO starboard AS sb (message_id, stars) VALUES ($1, $2)
        ON CONFLICT (message_id) DO UPDATE SET stars = $2 WHERE sb.message_id = $1
        RETURNING starboard_id, stars",
        message_id,
        userids.len() as i32
    )
    .fetch_one(&state.database)
    .await?;

    let starboard_channel = ChannelId::new(794949887028232192); // #starboard

    if userids.len() < state.star_threshold {
        // If there is an ID for it, a message was created
        if let Some(starboard_id) = starboard.starboard_id {
            sqlx::query!("DELETE FROM starboard WHERE message_id = $1", message_id)
                .execute(&state.database)
                .await?;

            starboard_channel
                .delete_message(&ctx.http, MessageId::new(starboard_id as u64))
                .await?;
        }

        return Ok(());
    }

    // Hardcoded star threshold for now, but it
    // should be configurable eventually
    match starboard.starboard_id {
        None => {
            let new_message = starboard_channel
                .send_message(&ctx.http, create_message(message, starboard.stars))
                .await?;

            sqlx::query!(
                "UPDATE starboard SET starboard_id = $1 WHERE message_id = $2",
                i64::from(new_message.id),
                message_id
            )
            .execute(&state.database)
            .await?;
        }
        Some(starboard_id) => {
            starboard_channel
                .edit_message(
                    &ctx.http,
                    MessageId::new(starboard_id as u64),
                    EditMessage::new().content(format!(
                        "**{}**🌟『{}』",
                        starboard.stars,
                        message.channel_id.mention()
                    )),
                )
                .await?;
        }
    }

    Ok(())
}
