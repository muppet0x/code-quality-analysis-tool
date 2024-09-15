use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

async fn analyze_code() -> impl Responder {
    HttpResponse::Ok().body("Code analysis in progress...\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initializing dotenv with better error handling
    if let Err(e) = dotenv() {
        eprintln!("Error loading .env file: {}", e);
    }
    
    // Fetching SERVER_ADDRESS with default value if missing and handling potential error better
    let server_address = match env::var("SERVER_ADDRESS") {
        Ok(val) => val,
        Err(_e) => {
            eprintln!("SERVER_ADDRESS not found, using default '127.0.0.1:8080'");
            "127.0.0.1:8080".to_string()
        },
    };
    println!("Starting server at: {}", &server_address);
    
    // Handling potential bind error more gracefully
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Welcome to the Rust Analysis Server!" }))
            .route("/analyze", web::get().to(analyze_code))
    })
    .bind(&server_address);
    
    match server {
        Ok(server_instance) => {
            server_instance.run().await
        },
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", &server_address, e);
            Err(e)
        }
    }
}