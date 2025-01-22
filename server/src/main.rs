use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WellcomeMessage {
    message: String,
}

#[get("/")]
//Wellcome api endpoint
async fn wellcome() -> impl Responder {
    let response = WellcomeMessage {
        message: "Wellcome to DeFimetrics.API".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(wellcome)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

