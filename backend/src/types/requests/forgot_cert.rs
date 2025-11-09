use serde::Deserialize;
use validator::Validate;

use crate::utils::smart_trim::smart_trim;

#[derive(Deserialize, Validate, Debug)]
pub struct ForgotCertRequest {
    #[validate(email)]
    pub email: String,
}

impl ForgotCertRequest {
    pub fn trim(&self) -> Self {
        Self {
            email: smart_trim(&self.email),
        }
    }
}
