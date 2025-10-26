use actix_web::Responder;

#[actix_web::delete("/cert")]
pub async fn delete_cert_endpoint() -> impl Responder {
    "Hey!"
}
