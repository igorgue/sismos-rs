use std::{env, fs::File, io::Read};

use chrono::{DateTime, Utc};
use log::info;
use serde_json::json;

use crate::fetch_data::{count_from_raw_sql, fetch_sismos_from_raw_sql};

const OPENAI_API_ENDPOINT: &str = "https://api.openai.com/v1/engines/text-davinci-003/completions";

pub async fn respond_with_ai(message: String) -> String {
    let message = message.to_lowercase();

    if message.contains("ayuda") || message.contains("help") {
        return "Comandos: [ayuda], escala:\n\n
        🌋: 0.0 - 2.9\n
        🌋🌋: 3.0 - 3.9\n
        🌋🌋🌋: 4.0 - 5.9\n
        🌋🌋🌋🌋: 6.0 - 6.9\n
        🌋🌋🌋🌋🌋: 7.0 - ...
        "
        .to_string();
    }

    let value: serde_json::Value =
        serde_json::from_str(do_request(message).await.unwrap().as_str()).unwrap();

    let raw_ai_sql_stmt = value["choices"][0]["text"].to_string();

    info!("Raw AI SQL statement: '{}'!!!", raw_ai_sql_stmt);

    let sql_stmt = raw_ai_sql_stmt.replace("\n\r", "").replace("\\n", " ");
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

    if clean_sql_stmt.to_uppercase().starts_with("SELECT COUNT") {
        let count = count_from_raw_sql(clean_sql_stmt.as_str()).await.unwrap();

        return format!("{} sismos encontrados", count);
    } else {
        let sismos = fetch_sismos_from_raw_sql(clean_sql_stmt.as_str()).await;

        if sismos.len() > 0 {
            let mut response = String::new();

            for sismo in sismos {
                response.push_str(
                    format!(
                        "{} - {}, {}, {}\n",
                        sismo.created.unwrap(),
                        sismo.richter.unwrap(),
                        sismo.location.unwrap(),
                        sismo.country.unwrap()
                    )
                    .as_str(),
                );
            }

            return response;
        }
    }

    String::from("No se encontraron sismos, intente con otra consulta")
}

async fn do_request(prompt: String) -> Result<String, reqwest::Error> {
    info!("User prompt: {}", prompt);

    let api_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let client = reqwest::Client::new();
    let params = json!({
        "prompt": get_ai_prompt(prompt),
        "temperature": 0.7,
        "max_tokens": 50
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
        _ => "🌎",
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
        _ => "WW",
    }
}

fn richter_to_emoji(richter: f32) -> &'static str {
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
        return format!("{} días", diff.num_days());
    }

    if diff.num_hours() > 0 {
        return format!("{} horas", diff.num_hours());
    }

    if diff.num_minutes() > 0 {
        return format!("{} minutos", diff.num_minutes());
    }

    if diff.num_seconds() > 0 {
        return format!("{} segundos", diff.num_seconds());
    }

    String::from("ahora!")
}

fn get_ai_prompt(user_prompt: String) -> String {
    let mut file = File::open("src/data/query.sismos.ai.txt").expect("File not found");

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let content = content.replace("$prompt", user_prompt.as_str());

    info!("AI prompt: {}", content.as_str());

    content
}
