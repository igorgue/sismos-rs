use actix_web::{get, post, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyJsonResponse {
    message: String,
}

#[get("/")]
async fn root() -> impl Responder {
    let obj = MyJsonResponse {
        message: "Hello, world!".to_string(),
    };
    HttpResponse::Ok().json(obj)
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
