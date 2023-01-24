use chrono::{DateTime, NaiveDateTime, Utc};

// Table from sqlite:
//
// CREATE TABLE sismos (
// 	id INTEGER NOT NULL,
// 	created DATETIME,
// 	lat FLOAT,
// 	long FLOAT,
// 	depth FLOAT,
// 	richter FLOAT,
// 	description VARCHAR,
// 	location VARCHAR,
// 	country VARCHAR,
// 	content_hash VARCHAR,
// 	partial_content_hash VARCHAR,
// 	PRIMARY KEY (id)
// );
/// A sismo (earthquake) in the database
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
pub struct ParsedSismo {
    pub created: DateTime<Utc>,
    pub lat: String,
    pub long: String,
    pub depth: String,
    pub richter: String,
    pub description: String,
    pub location: String,
    pub content_hash: String,
    pub partial_content_hash: String,
    pub country: String,
}
