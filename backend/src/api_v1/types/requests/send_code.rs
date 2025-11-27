use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::smart_trim::smart_trim;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SendCodePurposes {
    #[serde(rename = "create")]
    ConfirmCreation,
    #[serde(rename = "delete")]
    ConfirmDeletion{
        id: String,
    },
}

impl ToString for SendCodePurposes {
    fn to_string(&self) -> String {
        match self {
            &Self::ConfirmCreation => "create".to_string(),
            &Self::ConfirmDeletion { .. } => "delete".to_string()
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct SendCodeRequest {
    pub purpose: SendCodePurposes,
    #[validate(email)]
    pub email: String
}

impl SendCodeRequest {
    pub fn trim(&self) -> Self {
        Self {
            purpose: self.purpose.clone(),
            email: smart_trim(&self.email),
        }
    }
}
