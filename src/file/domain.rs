use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub file_name: String,
    pub file_url: String,
}

#[derive(Deserialize)]
pub struct NewItem {
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}
