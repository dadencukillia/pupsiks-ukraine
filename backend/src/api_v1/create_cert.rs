use actix_web::Responder;

#[actix_web::post("/cert")]
pub async fn create_cert_endpoint() -> impl Responder {
    "Hey"
}
