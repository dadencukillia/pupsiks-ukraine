use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub enum SendCodePurposes {
    #[serde(rename = "create")]
    ConfirmCreation,
    #[serde(rename = "delete")]
    ConfirmDeletion,
}

impl ToString for SendCodePurposes {
    fn to_string(&self) -> String {
        match self {
            &Self::ConfirmCreation => "create".to_string(),
            &Self::ConfirmDeletion => "delete".to_string()
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct SendCodeRequest {
    pub purpose: SendCodePurposes,
    #[validate(email)]
    pub email: String
}
