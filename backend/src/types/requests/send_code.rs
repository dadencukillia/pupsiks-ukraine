use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub enum SendCodePurposes {
    #[serde(rename = "creation")]
    ConfirmCreation,
    #[serde(rename = "deletion")]
    ConfirmDeletion,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SendCodeRequest {
    purpose: SendCodePurposes,
    #[validate(email)]
    email: String
}
