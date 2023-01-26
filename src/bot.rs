use std::env;
use std::fs::File;
use std::io::Read;

use log::info;
use serde_json::json;

use crate::fetch_data::run_raw_sql_stmt;

const OPENAI_API_ENDPOINT: &str = "https://api.openai.com/v1/engines/text-davinci-003/completions";

pub async fn respond_with_ai(message: String) -> String {
    let message = message.to_lowercase();
    let value: serde_json::Value =
        serde_json::from_str(do_request(message).await.unwrap().as_str()).unwrap();

    let raw_ai_sql_stmt = value["choices"][0]["text"].to_string();

    info!("Raw AI SQL statement: '{}'!!!", raw_ai_sql_stmt);

    let sql_stmt = raw_ai_sql_stmt
        .replace("\n\r", "")
        .replace("\\n", " ");
    let sql_stmt = sql_stmt
        .strip_prefix("\"").unwrap_or(sql_stmt.as_str());
    let sql_stmt = sql_stmt
        .strip_suffix("\"").unwrap_or(sql_stmt)
        .trim()
        .to_string();

    let clean_sql_stmt = match sql_stmt.find("select") {
        Some(index) => sql_stmt[index..].to_string(),
        None => match sql_stmt.find("SELECT") {
            Some(index) => sql_stmt[index..].to_string(),
            None => panic!("AI did not return a valid SQL statement"),
        },
    };

    info!("Clean SQL statement: {}", clean_sql_stmt);

    run_raw_sql_stmt(clean_sql_stmt.as_str()).await
}

async fn do_request(prompt: String) -> Result<String, reqwest::Error> {
    info!("Requesting AI response for: {}", prompt);

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

fn get_ai_prompt(user_prompt: String) -> String {
    let mut file = File::open("src/data/query.sismos.ai.txt").expect("File not found");

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let content = content.replace("$prompt", user_prompt.as_str());

    info!("AI prompt: {}", content.as_str());

    content
}
