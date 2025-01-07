use async_trait::async_trait;
use mongodb::{results::InsertOneResult, Collection, Database};

use super::domain::FileModel;

pub struct FileRepository {
    pub db: Database,
    pub collection: Collection<FileModel>,
}

#[async_trait]
pub trait FileRepositoryTrait {
    fn new(db: Database) -> Self;
    async fn create(&self, file: FileModel) -> Result<InsertOneResult, mongodb::error::Error>;
    // async fn find_all(&self) -> Vec<FileModel>;
}

#[async_trait]
impl FileRepositoryTrait for FileRepository {
    fn new(db: Database) -> Self {
        let collection = db.collection("files");
        FileRepository { db, collection }
    }

    async fn create(&self, file: FileModel) -> Result<InsertOneResult, mongodb::error::Error> {
        let res = self.collection.insert_one(file).await?;
        Ok(res)
    }

    // async fn find_all(&self) -> Vec<FileModel> {
    //     let cursor = self.collection.find(None, None).await.unwrap();
    //     cursor
    //         .map(|doc| bson::from_document(doc.unwrap()).unwrap())
    //         .collect()
    // }
}