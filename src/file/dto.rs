use actix_multipart::form::{text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

#[derive(Debug, MultipartForm)]
pub struct CreateFileRequest {
    pub file_name: Text<String>,
    pub file_url: Text<String>,
    // pub file: Vec<u8>,
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
