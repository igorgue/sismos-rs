use std::env;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenvy;
use log::info;

use sismos_api::api::{ai_response, root, whatsapp_incoming, whatsapp_status};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 1972;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap_or_default();
    env_logger::init_from_env::<_>(env_logger::Env::default().default_filter_or("info"));

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
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .service(root)
            .service(ai_response)
            .service(whatsapp_incoming)
            .service(whatsapp_status)
    })
    .bind((host, port))?
    .run()
    .await
}
