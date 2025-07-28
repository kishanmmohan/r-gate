mod presentation;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use crate::presentation::routers::health_routes::{routes as health_routes};





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().wrap(Logger::default()).configure(health_routes)
    }
    ).bind("0.0.0.0:4444")?.run().await
    
}

