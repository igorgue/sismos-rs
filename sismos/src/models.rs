use std::{fs::File, io::Read};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
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
#[derive(Serialize, Deserialize, Debug)]
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

/// A data structure of ChatGPT message
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTMessage {
    pub role: String,
    pub content: String,
}

impl ChatGPTMessage {
    pub fn get_messages_prompt(user_content: &str) -> Vec<ChatGPTMessage> {
        let mut messages = Vec::new();
        let mut file = File::open("sismos/src/data/query.sismos.ai.txt")
            .expect("query.sismos.ai.txt not found");

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        for line in content.split("\n\n") {
            messages.push(ChatGPTMessage {
                role: String::from("system"),
                content: line.to_string(),
            });
        }

        messages.push(ChatGPTMessage {
            role: String::from("user"),
            content: user_content.to_string(),
        });

        messages
    }
}

impl From<Sismo> for SismoResponse {
    /// Convert a Sismo to a SismoResponse
    fn from(item: Sismo) -> Self {
        SismoResponse {
            id: item.id,
            created: item.created.unwrap().to_string(),
            lat: item.lat.unwrap(),
            long: item.long.unwrap(),
            depth: item.depth.unwrap(),
            richter: item.richter.unwrap(),
            description: item.description.to_owned().unwrap(),
            location: item.location.to_owned().unwrap(),
            country: item.country.to_owned().unwrap(),
            content_hash: item.content_hash.to_owned().unwrap(),
            partial_content_hash: item.partial_content_hash.to_owned().unwrap(),
        }
    }
}
