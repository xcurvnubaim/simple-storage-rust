use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize)]
pub struct CreateItemResponse {
    pub id: String,
    pub file_name: String,
    pub file_url: String,
}