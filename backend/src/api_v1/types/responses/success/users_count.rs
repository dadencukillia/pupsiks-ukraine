use serde::Serialize;

#[derive(Serialize)]
pub struct StatsUserCountResponse {
    count: u64,
}

impl StatsUserCountResponse {
    pub fn new(count: u64) -> Self {
        Self { 
            count
        }
    }
}
