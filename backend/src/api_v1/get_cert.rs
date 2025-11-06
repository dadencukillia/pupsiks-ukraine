use actix_web::{web, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};
use crate::{models::cert, types::{errors::Errors, responses::success::CertificateResponse}, utils::{log_error::ResultLogger, uuid::get_uuid}};

#[actix_web::get("/cert/{uuid}")]
pub async fn get_cert_endpoint(
    path: web::Path<(String,)>,
    db: web::Data<DatabaseConnection>
) -> Result<web::Json<CertificateResponse>, Errors> {
    let place_name = "GET /api/v1/cert/{uuid}";

    let uuid = match get_uuid(&path.0) {
        Some(uuid) => uuid,
        None => {
            return Err(Errors::BadRequest { what_invalid: "serial number" })
        }
    };

    let find_result = cert::Entity::find_by_id(uuid)
        .limit(1)
        .one(db.as_ref())
        .await
        .log_with_place_on_error(place_name)
        .map_err(|_| Errors::InternalServer { what: "DB" })?;

    if let Some(certificate) = find_result {
        Ok(web::Json(
            CertificateResponse::new(&uuid, certificate.name, certificate.title)
        ))
    } else {
        Err(Errors::ResourceNotFound { what: "certificate" })
    }
}
