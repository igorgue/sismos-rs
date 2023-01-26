use actix_web::{get, post, HttpResponse, Responder};
use urlencoding::decode;

use crate::bot::respond_with_ai;
use crate::fetch_data::latest_5_sismos;
use crate::models::{Sismo, SismoResponse};

/// Gets latest 5 sismos from the database
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json(to_json_response(latest_5_sismos().await))
}

/// Gets an AI response to a prompt
#[get("/api")]
async fn ai_response(prompt: String) -> impl Responder {
    let encoded_prompt = decode(prompt.as_str()).expect("UTF-8");
    let response = respond_with_ai(encoded_prompt.to_string()).await;

    HttpResponse::Ok().body(response)
}

/// Incomming message to whatsapp
#[post("/whatsapp/incoming")]
async fn whatsapp_incoming(message: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/xml; charset=utf-8"))
        .body(to_whatsapp_xml_response(message))
}

/// Status message to whatsapp
#[post("/whatsapp/status")]
async fn whatsapp_status(_message: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/xml; charset=utf-8"))
        .body(to_whatsapp_xml_response(String::new()))
}

fn to_json_response(sismos: Vec<Sismo>) -> Vec<SismoResponse> {
    sismos
        .iter()
        .map(|sismo| sismo.as_json_response())
        .collect()
}

fn to_whatsapp_xml_response(message: String) -> String {
    let message = parse_twilio_whatsapp_message(message);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Response><Message>{}</Message></Response>"#,
        decode(message.as_str()).expect("UTF-8")
    )
    .to_string()
}

fn parse_twilio_whatsapp_message(message: String) -> String {
    let pairs = message.split("&");

    for pair in pairs {
        let kv = pair.split("=");

        let mut key = String::new();
        let mut value = String::new();

        for (i, part) in kv.enumerate() {
            if i == 0 {
                key = part.to_string();
            } else {
                value = part.to_string();
            }
        }

        if key == "Body" {
            return String::from(decode(value.as_str()).expect("UTF-8"));
        }
    }

    return String::new();
}
