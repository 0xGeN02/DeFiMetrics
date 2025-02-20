use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use lib::simple_interest;

#[derive(Serialize, Deserialize)]
struct APIRequest {
    principal: f64,
    rate: f64,
    time: f64,
}

#[derive(Serialize, Deserialize)]
struct APIResponse {
    data: String,
}

#[get("/simple_interest")]
async fn get_simple_interest(req: web::Query<APIRequest>) -> impl Responder {
    let result = simple_interest(req.principal, req.rate, req.time);
    if !result {
        HttpResponse::Error()
    }
    let response = APIResponse {
        data: format!(result),
    };
    HttpResponse::Ok().json(response)
}
