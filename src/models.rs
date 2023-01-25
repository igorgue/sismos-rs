use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

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
#[derive(Debug)]
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

impl Sismo {
    pub fn as_json_response(&self) -> SismoResponse {
        SismoResponse {
            id: self.id,
            created: self.created.unwrap().to_string(),
            lat: self.lat.unwrap(),
            long: self.long.unwrap(),
            depth: self.depth.unwrap(),
            richter: self.richter.unwrap(),
            description: self.description.clone().unwrap_or("".to_string()),
            location: self.location.clone().unwrap(),
            country: self.country.clone().unwrap(),
            content_hash: self.content_hash.clone().unwrap(),
            partial_content_hash: self.partial_content_hash.clone().unwrap(),
        }
    }
}
