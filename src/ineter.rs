use chrono::{DateTime, Utc};
use soup::prelude::*;

pub struct Sismo {
    pub created: DateTime<Utc>,
    pub lat: String,
    pub long: String,
    pub depth: String,
    pub richter: String,
    pub location: String,
    pub content_hash: String,
    pub partial_content_hash: String,
    pub country: String,
}

pub fn parse_html(html_content: &str) -> Vec<Sismo> {
    let soup = Soup::new(html_content);
    let pres = soup.tag("pre").find_all();

    // return empty vector of strings
    pres.map(|pre| parse_pre_item(pre.text())).collect()
}

fn parse_pre_item(pre: String) -> Sismo {
    let parts = pre.split_whitespace().collect::<Vec<&str>>();
    let local_time = parts.as_slice()[0..2].join(" ") + " -06:00";

    Sismo {
        created: DateTime::parse_from_str(local_time.as_str(), "%y/%m/%d %H:%M:%S %z").unwrap().with_timezone(&Utc),
        lat: "".to_string(),
        long: "".to_string(),
        depth: "".to_string(),
        richter: "".to_string(),
        location: pre,
        content_hash: "".to_string(),
        partial_content_hash: "".to_string(),
        country: "".to_string(),
    }
}
