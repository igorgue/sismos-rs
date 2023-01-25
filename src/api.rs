use actix_web::{get, post, HttpResponse, Responder};

use crate::fetch_data::latest_5_sismos;
use crate::models::{Sismo, SismoResponse};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json(to_json_response(latest_5_sismos().await))
}

#[get("/api")]
async fn ai_response(prompt: String) -> impl Responder {
    // TODO: Add a real response
    HttpResponse::Ok().body(prompt)
}

#[post("/whatsapp/incoming")]
async fn whatsapp_incoming(prompt: String) -> impl Responder {
    // TODO: Add a real response
    HttpResponse::Ok().body(prompt)
}

#[post("/whatsapp/status")]
async fn whatsapp_status(prompt: String) -> impl Responder {
    // TODO: Add a real response
    HttpResponse::Ok().body(prompt)
}

fn to_json_response(sismos: Vec<Sismo>) -> Vec<SismoResponse> {
    sismos
        .iter()
        .map(|sismo| sismo.as_json_response())
        .collect()
}
