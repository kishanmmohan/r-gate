use actix_web::web;
use crate::presentation::handlers::health_handler::{health, root};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/v1").service(health).service(root));
}



