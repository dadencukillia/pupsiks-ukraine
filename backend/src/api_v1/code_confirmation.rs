use actix_web::Responder;

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint() -> impl Responder {
    "Hey"
}
