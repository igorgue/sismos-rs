use std::env;

use chrono::{DateTime, Local, TimeZone, Utc};
use log::info;
use serde_json::json;

use crate::fetch_data::{fetch_sismos_from_raw_sql, result_from_raw_sql};
use crate::models::{ChatGPTMessage, Sismo};

const OPENAI_API_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub async fn respond_with_ai(message: String) -> String {
    let message = message.to_lowercase();

    if message.contains("ayuda") || message.contains("help") {
        return "Comandos: [ayuda], escala:\n\n\
        🌋: 0.0 - 2.9\n\
        🌋🌋: 3.0 - 3.9\n\
        🌋🌋🌋: 4.0 - 5.9\n\
        🌋🌋🌋🌋: 6.0 - 6.9\n\
        🌋🌋🌋🌋🌋: 7.0 - ..."
            .to_string();
    }

    let value: serde_json::Value =
        serde_json::from_str(do_request(message).await.unwrap().as_str()).unwrap();

    let raw_ai_sql_stmt = value["choices"][0]["message"]["content"].to_string();

    info!("Raw AI SQL statement: '{}'!!!", raw_ai_sql_stmt);

    let sql_stmt = raw_ai_sql_stmt.replace("\n\r", "").replace("\\n", " ");
    let sql_stmt = sql_stmt.replace("\\", "");
    let sql_stmt = sql_stmt.strip_prefix("\"").unwrap_or(sql_stmt.as_str());
    let sql_stmt = sql_stmt
        .strip_suffix("\"")
        .unwrap_or(sql_stmt)
        .trim()
        .to_string();

    let clean_sql_stmt = match sql_stmt.to_lowercase().find("select") {
        Some(index) => sql_stmt[index..].to_string(),
        None => panic!("AI did not return a valid SQL statement"),
    };

    info!("AI clean SQL statement: {}", clean_sql_stmt);

    if clean_sql_stmt.to_uppercase().starts_with("SELECT *") {
        let sismos = fetch_sismos_from_raw_sql(clean_sql_stmt.as_str()).await;

        format_sismos(sismos)
    } else {
        let result = result_from_raw_sql(clean_sql_stmt.as_str()).await.unwrap();

        format!("Respuesta: {}.", result)
    }
}

async fn do_request(prompt: String) -> Result<String, reqwest::Error> {
    info!("User prompt: {}", prompt);

    let api_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let client = reqwest::Client::new();
    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": ChatGPTMessage::get_messages_prompt(prompt.as_str()),
    });
    let response = client
        .post(OPENAI_API_ENDPOINT)
        .json(params.as_object().unwrap())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await?;

    Ok(response.text().await?)
}

fn country_to_flag_emoji(country: &str) -> &'static str {
    match country {
        "Nicaragua" => "🇳🇮",
        "Costa Rica" => "🇨🇷",
        "Panama" => "🇵🇦",
        "Panamá" => "🇵🇦",
        "Honduras" => "🇭🇳",
        "El Salvador" => "🇸🇻",
        "Guatemala" => "🇬🇹",
        "Mexico" => "🇲🇽",
        "México" => "🇲🇽",
        _ => "🌎", // Mr. Worldwide 😎
    }
}

fn country_to_abbr(country: &str) -> &'static str {
    match country {
        "Nicaragua" => "NI",
        "Costa Rica" => "CR",
        "Panama" => "PA",
        "Panamá" => "PA",
        "Honduras" => "HN",
        "El Salvador" => "SV",
        "Guatemala" => "GT",
        "Mexico" => "MX",
        "México" => "MX",
        _ => "WW", // Mr. Worldwide 😎
    }
}

fn richter_to_emoji(richter: f64) -> &'static str {
    if richter > 7.0 {
        return "🌋🌋🌋🌋🌋";
    }

    if richter > 6.0 {
        return "🌋🌋🌋🌋";
    }

    if richter > 5.0 {
        return "🌋🌋🌋";
    }

    if richter > 4.0 {
        return "🌋🌋";
    }

    if richter > 3.0 {
        return "🌋";
    }

    "🌋"
}

fn datetime_to_time_ago_in_spanish(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(*datetime);

    if diff.num_days() > 0 {
        return format!("hace {} días", diff.num_days());
    }

    if diff.num_hours() > 0 {
        return format!("hace {} horas", diff.num_hours());
    }

    if diff.num_minutes() > 0 {
        return format!("hace {} minutos", diff.num_minutes());
    }

    if diff.num_seconds() > 0 {
        return format!("hace {} segundos", diff.num_seconds());
    }

    String::from("ahora!")
}

fn format_sismos(sismos: Vec<Sismo>) -> String {
    let mut response = String::new();
    let header = "Sismos: \n\n";

    response.push_str(header);
    for sismo in sismos {
        response.push_str(format_sismo(sismo).as_str());
    }

    let footer = "\n\nFuente: INETER (Nicaragua)";
    response.push_str(footer);

    response
}

fn format_sismo(sismo: Sismo) -> String {
    let richter = sismo.richter.unwrap();
    let richter_emoji = richter_to_emoji(richter);
    let richter = format!("{:.1}", richter);

    let location = sismo.location.unwrap();
    let country = sismo.country.unwrap();
    let country_flag = country_to_flag_emoji(country.as_str());
    let country_abbr = country_to_abbr(country.as_str());

    let created = sismo.created.unwrap();
    let created = Local.from_local_datetime(&created);
    let time_ago = datetime_to_time_ago_in_spanish(&DateTime::from(created.unwrap()));

    format!(
        "{} {}: {} {}\n{}. {}\n",
        country_abbr, country_flag, richter, richter_emoji, location, time_ago
    )
}
