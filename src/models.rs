use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

/// A sismo (earthquake) in the database
///
/// Table from sqlite:
///
/// CREATE TABLE sismos (
/// 	id INTEGER NOT NULL,
/// 	created DATETIME,
/// 	lat FLOAT,
/// 	long FLOAT,
/// 	depth FLOAT,
/// 	richter FLOAT,
/// 	description VARCHAR,
/// 	location VARCHAR,
/// 	country VARCHAR,
/// 	content_hash VARCHAR,
/// 	partial_content_hash VARCHAR,
/// 	PRIMARY KEY (id)
/// );
#[derive(Debug, Clone, FromRow)]
pub struct Sismo {
    pub id: i64,
    pub created: Option<NaiveDateTime>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub depth: Option<f64>,
    pub richter: Option<f64>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub country: Option<String>,
    pub content_hash: Option<String>,
    pub partial_content_hash: Option<String>,
}

/// A sismo (earthquake) parsed from HTML
#[derive(Debug)]
pub struct ParsedSismo {
    pub created: DateTime<Utc>,
    pub lat: String,
    pub long: String,
    pub depth: String,
    pub richter: String,
    pub description: String,
    pub location: String,
    pub country: String,
    pub content_hash: String,
    pub partial_content_hash: String,
}

/// A sismo (earthquake) as a JSON response
#[derive(Serialize, Debug)]
pub struct SismoResponse {
    pub id: i64,
    pub created: String,
    pub lat: f64,
    pub long: f64,
    pub depth: f64,
    pub richter: f64,
    pub description: String,
    pub location: String,
    pub country: String,
    pub content_hash: String,
    pub partial_content_hash: String,
}

impl From<Sismo> for SismoResponse {
    fn from(item: Sismo) -> Self {
        SismoResponse {
            id: item.id,
            created: item.created.unwrap().to_string(),
            lat: item.lat.unwrap(),
            long: item.long.unwrap(),
            depth: item.depth.unwrap(),
            richter: item.richter.unwrap(),
            description: item.description.to_owned().unwrap_or(String::new()),
            location: item.location.to_owned().unwrap(),
            country: item.country.to_owned().unwrap(),
            content_hash: item.content_hash.to_owned().unwrap(),
            partial_content_hash: item.partial_content_hash.to_owned().unwrap(),
        }
    }
}
