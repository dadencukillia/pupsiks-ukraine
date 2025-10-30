use serde::Serialize;

#[derive(Serialize)]
pub struct CodeSentResponse {
    message: String,
    email: String,
    token: String,
    expires_at: u64
}

impl CodeSentResponse {
    pub fn new(email: String, token: String, expires_at: u64) -> Self {
        Self {
            message: "".to_string(),
            email,
            token,
            expires_at
        }
    }
}
