use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileModel {
    pub id: Option<i64>,
    pub file_name: String,
    pub file_url: String,
}

pub fn new_file_model(file_name: String, file_url: String) -> FileModel {
    FileModel {
        id: None,
        file_name,
        file_url,
    }
}
