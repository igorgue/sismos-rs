use chrono::{DateTime, Utc};
use reqwest;
use sha2::Digest;
use soup::prelude::*;

use crate::models::ParsedSismo;

/// URL to get the data from
pub const DATA_URL: &str = "https://webserver2.ineter.gob.ni/geofisica/sis/events/sismos.php";

/// Get the data from the INETER API
pub async fn get_data_from_api(url: Option<&str>) -> Result<Vec<ParsedSismo>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url.unwrap_or(DATA_URL)).send().await?;

    Ok(parse_html(response.text().await?.as_str()))
}

/// Parse the HTML content to get the data
pub fn parse_html(html_content: &str) -> Vec<ParsedSismo> {
    let soup = Soup::new(html_content);
    let pres = soup.tag("pre").find_all();

    pres.map(|pre| parse_pre_item(pre.text())).collect()
}

fn parse_pre_item(pre: String) -> ParsedSismo {
    let parts = pre.split_whitespace().collect::<Vec<&str>>();

    let local_time = parts[0..2].join(" ");
    let local_time_with_timezone = local_time.to_string() + " -06:00";
    let created =
        DateTime::parse_from_str(local_time_with_timezone.as_str(), "%y/%m/%d %H:%M:%S %z")
            .unwrap()
            .with_timezone(&Utc);
    let lat = parts[2].to_string();
    let long = parts[3].to_string();
    let depth = parts[4].to_string();
    let richter = parts[5].to_string();
    let description = parts[6].to_string();
    let location = parts[7..].join(" ");
    let country = location.rsplit(", ").next().unwrap().to_string();
    let location = location
        .replace(&country, "")
        .replace(",", "")
        .trim()
        .to_string();
    let content_hash = get_content_hash(pre.to_owned());
    let partial_content_hash =
        get_partial_content_hash(local_time, &lat, &long, &depth, &richter, &description);

    ParsedSismo {
        created,
        lat,
        long,
        depth,
        richter,
        description,
        location,
        country,
        content_hash,
        partial_content_hash,
    }
}

fn get_content_hash(content: String) -> String {
    hex::encode(sha2::Sha256::digest(content.as_bytes()))
}

fn get_partial_content_hash(
    local_time: String,
    lat: &String,
    long: &String,
    depth: &String,
    richter: &String,
    description: &String,
) -> String {
    let content = format!(
        "{}{}{}{}{}{}",
        local_time, lat, long, depth, richter, description
    );

    hex::encode(sha2::Sha256::digest(content.as_bytes()))
}
