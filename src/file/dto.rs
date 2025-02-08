use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateFileRequest {
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize)]
pub struct CreateFileResponse {
    pub id: String,
    pub file_name: String,
    pub file_url: String,
}

#[derive(Serialize)]
pub struct FindOneFileResponse {
    pub id: i64,
    pub file_name: String,
    pub file_url: String,
}

#[derive(Serialize)]
pub struct FindAllFileResponse {
    pub files: Vec<FindOneFileResponse>,
}
