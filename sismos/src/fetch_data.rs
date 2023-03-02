use std::env;

use futures::TryStreamExt;
use log::info;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{query, query_as, Pool, Row, Sqlite};

use crate::ineter::get_data_from_api;
use crate::models::{ParsedSismo, Sismo};

/// Fetches the latest 5 sismos from the database
pub async fn latest_5_sismos() -> Vec<Sismo> {
    do_latest_5_sismos(get_pool().await).await
}

/// Fetches data from the API and inserts it into the database
pub async fn fetch_data() {
    do_fetch_data(get_pool().await).await
}

pub async fn result_from_raw_sql(sql: &str) -> Result<String, sqlx::Error> {
    assert!(
        sql.to_uppercase().starts_with("SELECT COUNT")
            || sql.to_uppercase().starts_with("SELECT SUM")
            || sql.to_uppercase().starts_with("SELECT AVG")
    );

    let pool = get_pool().await;

    info!("Executing query: {}!!!", sql);

    let count: f32 = query(sql).fetch_one(&pool).await?.get(0);

    Ok(count.to_string())
}

pub async fn fetch_sismos_from_raw_sql(sql: &str) -> Vec<Sismo> {
    assert!(sql.to_uppercase().starts_with("SELECT *"));

    let pool = get_pool().await;

    info!("Executing query: {}!!!", sql);

    let mut results = Vec::new();
    let mut rows = query_as::<_, Sismo>(sql).fetch(&pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        match <Sismo as TryInto<Sismo>>::try_into(row) {
            Ok(sismo) => {
                results.push(sismo);
            }
            Err(_) => (),
        }
    }

    results
}

async fn get_pool() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to {}", database_url);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap()
}

async fn do_latest_5_sismos(pool: Pool<Sqlite>) -> Vec<Sismo> {
    query_as!(Sismo, "SELECT * FROM sismos ORDER BY created DESC LIMIT 5")
        .fetch_all(&pool)
        .await
        .unwrap()
}

async fn do_fetch_data(pool: Pool<Sqlite>) {
    let data = get_data_from_api(None).await.unwrap();

    for sismo in data {
        try_insert_sismo(&pool, sismo).await;
    }
}

async fn try_insert_sismo(pool: &Pool<Sqlite>, sismo: ParsedSismo) {
    match query_as!(
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
            try_insert_partial_sismo(pool, sismo).await;
        }
    }
}

async fn try_insert_partial_sismo(pool: &Pool<Sqlite>, sismo: ParsedSismo) {
    match query_as!(
        Sismo,
        "SELECT * FROM sismos WHERE partial_content_hash = ? LIMIT 1",
        sismo.partial_content_hash
    )
    .fetch_one(pool)
    .await
    {
        Ok(sismo_db) => {
            info!("Sismo already exists: {:?}, updating", sismo_db);

            let _ = query!(
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

            query!(
                "INSERT INTO sismos (created, lat, long, depth, richter, description, location, country, content_hash, partial_content_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                sismo.created,
                sismo.lat,
                sismo.long,
                sismo.depth,
                sismo.richter,
                sismo.description,
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
