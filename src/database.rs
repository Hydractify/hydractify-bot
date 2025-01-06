use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect(database_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
