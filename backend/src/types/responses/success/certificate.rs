use sea_orm::prelude::Uuid;
use serde::Serialize;
use short_uuid::ShortUuid;

#[derive(Serialize)]
pub struct CertificateResponse {
    pub id: String,
    pub name: String,
    pub title: String,
}

impl CertificateResponse {
    pub fn new(id: &Uuid, name: String, title: String) -> Self {
        Self {
            id: ShortUuid::from_uuid(id).to_string(),
            name,
            title
        }
    }
}
