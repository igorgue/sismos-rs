use std::env;

use actix_web::{middleware, App, HttpServer};
use dotenvy;
use log::info;

use sismos::{
    api::{ai_response, root, whatsapp_incoming, whatsapp_status},
    fetch_data::fetch_data,
};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 1972;

/// Starts the server or with an argument `fetch-data` to fetch data from the API
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    env_logger::init_from_env::<_>(env_logger::Env::default().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();


    if args.len() >= 2 {
        return handle_arg(args).await;
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

async fn handle_arg(args: Vec<String>) -> std::io::Result<()> {
    let last_arg = args.last().unwrap();
    info!("Arg: {:?}", args);

    if last_arg == "fetch-data" {
        fetch_data().await;
    } else {
        info!("Invalid argument: {}", last_arg);
    }

    Ok(())
}
