use sea_orm::prelude::Uuid;
use serde::Serialize;
use short_uuid::ShortUuid;

#[derive(Serialize)]
pub struct CertIdResponse {
    pub id: String,
}

impl CertIdResponse {
    pub fn new(id: &Uuid) -> Self {
        Self {
            id: ShortUuid::from_uuid(id).to_string(),
        }
    }
}
