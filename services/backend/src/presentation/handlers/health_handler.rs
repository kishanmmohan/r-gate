use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize};

#[derive(Serialize)]
struct ApiStatus {
    status: String,
    service: String,
}

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("IAM Rust Services is up!")
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(ApiStatus {
        status: "healthy".to_string(),
        service: "IAM Service".to_string(),
    })
}

