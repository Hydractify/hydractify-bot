#[derive(sqlx::FromRow)]
pub struct TestRow {
    test: Option<String>,
}

#[poise::command(slash_command)]
pub async fn test(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let rows = sqlx::query_as::<_, TestRow>("SELECT test FROM test;")
        .fetch_all(&ctx.data().database)
        .await?;

    for row in rows {
        ctx.say(row.test.unwrap()).await?;
    }

    Ok(())
}
