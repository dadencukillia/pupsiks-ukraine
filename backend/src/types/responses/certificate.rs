use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use short_uuid::ShortUuid;

#[derive(Serialize)]
pub struct CertificateResponse {
    pub message: String,
    pub id: String,
    pub name: String,
    pub title: String,
}

impl CertificateResponse {
    pub fn new(id: &Uuid, name: String, title: String) -> Self {
        Self {
            message: "".to_string(),
            id: ShortUuid::from_uuid(id).to_string(),
            name,
            title
        }
    }
}
