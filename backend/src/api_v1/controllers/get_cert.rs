use actix_web::web;
use crate::{
    api_v1::{
        repos::CertRepo, 
        types::{
            errors::Errors, 
            responses::success::CertificateResponse
        }
    }, 
    utils::uuid::get_uuid
};

#[actix_web::get("/cert/{uuid}")]
pub async fn get_cert_endpoint(
    path: web::Path<(String,)>,
    cert_repo: web::Data<CertRepo>
) -> Result<web::Json<CertificateResponse>, Errors> {
    let uuid = match get_uuid(&path.0) {
        Some(uuid) => uuid,
        None => {
            return Err(Errors::BadRequest { what_invalid: "serial number" })
        }
    };

    let find_option = cert_repo.find_cert_by_id(uuid)
        .await
        .map_err(|_| Errors::InternalServer { what: "DB" })?;

    if let Some(certificate) = find_option {
        Ok(web::Json(
            CertificateResponse::new(&uuid, certificate.name, certificate.title)
        ))
    } else {
        Err(Errors::ResourceNotFound { what: "certificate" })
    }
}
