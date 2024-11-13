use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("missing DATABASE_URL"))
        .await
}
