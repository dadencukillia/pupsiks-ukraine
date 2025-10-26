use actix_web::{web, Responder};

#[actix_web::get("/cert/{uuid}")]
pub async fn get_cert_endpoint(path: web::Path<(String,)>) -> impl Responder {
    format!("Hello, world, {}", path.0)
}
