use actix_web::{get, middleware, post, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::env;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[get("/")]
async fn root() -> impl Responder {
    // TODO: Add a real response
    HttpResponse::Ok().body("Sismos root!")
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

fn handle_args(args: Vec<String>) -> std::io::Result<()> {
    info!("Args: {:?}", &args[1..]);

    if args[1] == "fetch-data" {
        info!("Fetching data...");
    } else {
        info!("Invalid argument: {}", args[1]);
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env::<_>(env_logger::Env::default().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        return handle_args(args);
    }

    let is_ssl = env::var("SSL").is_ok();
    let host = env::var("HOST").unwrap_or_else(|_| HOST.to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| PORT.to_string())
        .parse::<u16>()
        .unwrap_or(PORT);
    let protocol = if is_ssl { "https" } else { "http" };

    info!("Starting server at {}://{}:{}", protocol, host, port);

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(ai_response)
            .service(whatsapp_incoming)
            .service(whatsapp_status)
            .wrap(middleware::Logger::default())
    })
    .bind((host, port))?
    .run()
    .await
}
