use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

async fn analyze_code() -> impl Responder {
    HttpResponse::Ok().body("Code analysis in progress...\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("Starting server at: {}", &server_address);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Welcome to the Rust Analysis Server!" }))
            .route("/analyze", web::get().to(analyze_code))
    })
    .bind(&server_address)?
    .run()
    .await
}