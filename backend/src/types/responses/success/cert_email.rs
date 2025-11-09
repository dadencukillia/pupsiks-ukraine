use serde::Serialize;

#[derive(Serialize)]
pub struct CertEmailResponse {
    pub email: String,
}

impl CertEmailResponse {
    pub fn new(email: String) -> Self {
        Self {
            email
        }
    }
}
