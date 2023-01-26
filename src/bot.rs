use std::env;
use std::fs::File;
use std::io::Read;

const OPENAI_API_ENDPOINT: &str = "https://api.openai.com/v1/completions";

pub async fn respond_with_ai(message: String) -> String {
    let message = message.to_lowercase();

    do_request(message).await.unwrap()
}

async fn do_request(prompt: String) -> Result<String, reqwest::Error> {
    let api_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let client = reqwest::Client::new();
    let response = client
        .get(OPENAI_API_ENDPOINT)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_token))
        .body(format!(
            r#"{{
                "model": "text-davinci-002",
                "prompt": "{}",
                "max_tokens": 50,
                "temperature": 0.7,
            }}"#,
            get_ai_prompt(prompt)
        ))
        .send()
        .await?;

    Ok(response.text().await?)
}

fn get_ai_prompt(user_prompt: String) -> String {
    let mut file = File::open("data/query.sismos.ai.txt").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content.replace("$prompt", user_prompt.as_str())
}
