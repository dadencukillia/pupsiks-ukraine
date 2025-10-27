use actix_web::{web, Error, Responder};

use crate::types::{errors::Errors, requests::SendCodeRequest};

#[actix_web::post("/send_code")]
pub async fn send_code_endpoint(
    body: Result<web::Json<SendCodeRequest>, Error>
) -> Result<&'static str, Errors> {
    match body {
        Ok(_request) => {
            Ok("Hey!")
        },
        Err(_) => Err(Errors::BadRequest { what_invalid: "body" })
    }
}
