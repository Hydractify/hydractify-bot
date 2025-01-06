use sqlx::{Pool, Postgres};

pub struct State {
    pub database: Pool<Postgres>,
    pub star_threshold: usize,
}

impl State {
    pub async fn load(config: crate::Configuration) -> Self {
        Self {
            database: crate::database::connect(config.database_url).await.unwrap(),
            star_threshold: config.star_threshold,
        }
    }
}
