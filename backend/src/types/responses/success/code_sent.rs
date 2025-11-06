use serde::Serialize;

#[derive(Serialize)]
pub struct CodeSentResponse {
    email: String,
    token: String,
    expires_at: u64
}

impl CodeSentResponse {
    pub fn new(email: String, token: String, expires_at: u64) -> Self {
        Self {
            email,
            token,
            expires_at
        }
    }
}
