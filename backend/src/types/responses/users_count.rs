use serde::Serialize;

#[derive(Serialize)]
pub struct StatsUserCountResponse {
    message: String,
    count: u64,
}

impl StatsUserCountResponse {
    pub fn new(count: u64) -> Self {
        Self { 
            message: "".to_string(), 
            count
        }
    }
}
