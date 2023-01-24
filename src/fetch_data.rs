use std::env;

use log::info;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;

use crate::models::Sismo;

pub async fn fetch_data() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Connecting to {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();

    do_fetch_data(pool).await
}

async fn do_fetch_data(pool: Pool<Sqlite>) {
    let sismos = sqlx::query_as!(
        Sismo,
        "SELECT * from sismos"
    ).fetch_all(&pool).await;

    info!("First sismo in db: {:?}", sismos.unwrap()[0]);
}
