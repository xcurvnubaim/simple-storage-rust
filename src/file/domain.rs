use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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