use sqlx::{Pool, Postgres};

pub struct State {
    pub database: Pool<Postgres>,
}

impl State {
    pub async fn load() -> Self {
        Self {
            database: crate::database::connect().await.unwrap(),
        }
    }
}
