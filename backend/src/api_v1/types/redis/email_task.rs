use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct EmailTask {
    pub purpose: String,
    pub email: String,
    pub replacements: HashMap<String, String>
}
