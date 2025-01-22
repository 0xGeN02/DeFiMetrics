use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
#[derive(Serialize, Deserialize)]
struct WellcomeMessage {
    message: String,
}

#[get("/")]
//Wellcome api endpoint
async fn wellcome() -> impl Responder {
    let response = WellcomeMessage {
        message: "Wellcome to DeFiMetrics.API".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("RUST_SERVER_PORT").expect("RUST_SERVER_PORT .env ERROR");
    println!();
    println!("\x1b[34mDeFiMetrics.API Server\x1b[0m");
    println!("Server running at: http://127.0.0.1:{}", port);
    println!("Local server address: http://localhost:{}", port);
    println!();
    HttpServer::new(|| {
        App::new()
            .service(wellcome)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

