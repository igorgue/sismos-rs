use std::env;

use log::info;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

use crate::ineter::get_data_from_api;
use crate::models::{ParsedSismo, Sismo};

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
    let data = get_data_from_api(None).await.unwrap();

    for sismo in data {
        try_insert_sismo(&pool, sismo).await;
    }
}

async fn try_insert_sismo(pool: &Pool<Sqlite>, sismo: ParsedSismo) {
    match sqlx::query_as!(
        Sismo,
        "SELECT * FROM sismos WHERE content_hash = ? LIMIT 1",
        sismo.content_hash
    )
    .fetch_one(pool)
    .await
    {
        Ok(sismo) => {
            info!("Sismo already exists: {:?}", sismo);
        }
        Err(_) => {
            match sqlx::query_as!(
                Sismo,
                "SELECT * FROM sismos WHERE partial_content_hash = ? LIMIT 1",
                sismo.partial_content_hash
            )
            .fetch_one(pool)
            .await
            {
                Ok(sismo_db) => {
                    info!("Sismo already exists: {:?}, updating", sismo_db);

                    let _ = sqlx::query!(
                        "UPDATE sismos SET created = ?, lat = ?, long = ?, depth = ?, richter = ?, description = ?, location = ?, country = ?, content_hash = ? WHERE id = ?",
                        sismo.created,
                        sismo.lat,
                        sismo.long,
                        sismo.depth,
                        sismo.richter,
                        sismo.description,
                        sismo.location,
                        sismo.country,
                        sismo.content_hash,
                        sismo_db.id,
                    );
                }
                Err(_) => {
                    info!("Inserting sismo: {:?}", sismo);

                    sqlx::query!(
                        "INSERT INTO sismos (created, lat, long, depth, richter, location, country, content_hash, partial_content_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                        sismo.created,
                        sismo.lat,
                        sismo.long,
                        sismo.depth,
                        sismo.richter,
                        sismo.location,
                        sismo.country,
                        sismo.content_hash,
                        sismo.partial_content_hash
                    )
                    .execute(pool)
                    .await
                    .unwrap();
                }
            }
        }
    }
}
