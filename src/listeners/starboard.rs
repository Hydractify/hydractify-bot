use poise::serenity_prelude::{
    self as serenity, ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage,
    Mentionable, Message, MessageId, Reaction, UserId,
};

use crate::{state::State, Error};

struct Starboard {
    /// The ID of the original message being starboarded.
    message_id: i64,
    /// The ID of the message in the starboard channel.
    starboard_id: Option<i64>,
    /// How many stars the message has.
    stars: i32,
}

/// Creates the template message for a new starboard entry in the server.
/// It only returns the builder required to pass as an argument to `<GuildChannel>.send_message`.
fn create_message(original_message: Message, starboard: &Starboard) -> CreateMessage {
    let image = if !original_message.attachments.is_empty() {
        &original_message.attachments.first().unwrap().url
    } else if !original_message.embeds.is_empty() {
        let embed = &original_message.embeds.first().unwrap();

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
        "**{}**🌟{}",
        starboard.stars,
        original_message.channel_id.mention()
    ))
}

pub async fn handle_reaction(
    ctx: &serenity::Context,
    state: &State,
    reaction: &Reaction,
) -> Result<(), Error> {
    let user_from_reaction = reaction.user(&ctx.http).await?;

    // If `None` it means that we are not receiving from a Guild.
    if reaction.guild_id.is_none() {
        return Ok(());
    }

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
    // if message.author.id == user_from_reaction.id {
    //     return Ok(());
    // }

    // The UserID of each user that has reacted with a valid reaction.
    let mut userids: Vec<UserId> = Vec::new();

    for reac in message.reactions.clone() {
        if valid_emotes
            .iter()
            .any(|e| *e == reac.reaction_type.to_string())
        {
            let mut users: Vec<UserId> = reaction
                .users(&ctx.http, reac.reaction_type, None, None::<UserId>)
                .await?
                .iter()
                .filter_map(|user| {
                    // Ignore reactions from bots and the author of the message.
                    if user.bot
                    /*|| message.author.id == user.id*/
                    {
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

    if userids.len() < 3 {
        return Ok(());
    }

    // Get the message ID and turn it onto a BIGINT equivalent for the query.
    let message_id: i64 = message.id.into();

    sqlx::query!(
        "INSERT INTO starboard AS sb (message_id, stars) VALUES ($1, $2)
        ON CONFLICT (message_id) DO UPDATE SET stars = $2 WHERE sb.message_id = $1;",
        message_id,
        userids.len() as i32
    )
    .execute(&state.database)
    .await?;

    let starboard = sqlx::query_as!(
        Starboard,
        "SELECT * FROM starboard WHERE message_id = $1",
        message_id
    )
    .fetch_one(&state.database)
    .await?;

    let guild_channels = reaction.guild_id.unwrap().channels(&ctx.http).await?;

    let starboard_channel = guild_channels
        .get_key_value(&ChannelId::new(430532424280178688)) // #private-testing
        .unwrap()
        .1;

    // Hardcoded star threshold for now, but it
    // should be configurable eventually
    /*if starboard.starboard_id.is_some() && starboard.stars < 3 {
        let starboard_message = starboard_channel
            .message(
                &ctx.http,
                MessageId::new(starboard.starboard_id.unwrap() as u64),
            )
            .await?;

        // Delete the message since it's below the threshold;
        starboard_message.delete(&ctx.http).await?;

        // then remove it from database too.
        sqlx::query!(
            "UPDATE starboard SET starboard_id = NULL WHERE message_id = $1",
            starboard.message_id
        )
        .execute(&state.database)
        .await?;
    } else*/
    if starboard.starboard_id.is_none() {
        let new_message = starboard_channel
            .send_message(&ctx.http, create_message(message, &starboard))
            .await?;

        sqlx::query!(
            "UPDATE starboard SET starboard_id = $1 WHERE message_id = $2",
            i64::from(new_message.id),
            starboard.message_id
        )
        .execute(&state.database)
        .await?;
    } else {
        let mut starboard_message = starboard_channel
            .message(
                &ctx.http,
                MessageId::new(starboard.starboard_id.unwrap() as u64),
            )
            .await?;

        starboard_message
            .edit(
                &ctx.http,
                EditMessage::new().content(format!(
                    "**{}**🌟{}",
                    starboard.stars,
                    message.channel_id.mention()
                )),
            )
            .await?;
    }

    sqlx::query!(
        "UPDATE starboard SET stars = $1 WHERE message_id = $2",
        starboard.stars,
        starboard.message_id
    )
    .execute(&state.database)
    .await?;

    Ok(())
}
