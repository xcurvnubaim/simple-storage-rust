use std::fs;
use uuid::Uuid;
use super::{
    domain::new_file_model,
    dto::{CreateFileRequest, CreateFileResponse, FindAllFileResponse, FindOneFileResponse},
    repository::{FileRepository, FileRepositoryTrait},
};
use actix_web::{error::ErrorInternalServerError, Error};
use async_trait::async_trait;

pub struct FileUsecase {
    pub base_url: String,
    pub repository: FileRepository,
}

#[async_trait]
pub trait FileUsecaseTrait {
    fn new(repository: FileRepository) -> Self;
    async fn create(&self, file: CreateFileRequest) -> Result<CreateFileResponse, Error>;
    async fn find_all(&self) -> Result<FindAllFileResponse, Error>;
}

#[async_trait]
impl FileUsecaseTrait for FileUsecase {
    fn new(repository: FileRepository) -> Self {
        FileUsecase { 
            base_url: std::env::var("STORAGE_BASE_URL").unwrap_or("http://localhost:8080".to_string()),
            repository 
        }
    }

    async fn create(&self, file: CreateFileRequest) -> Result<CreateFileResponse, Error> {
        println!("{:?}", file);
        let id = Uuid::new_v4();
        let path = format!("public/{}_{}", id, file.file.file_name.unwrap());
        let base_url = &self.base_url;
        let mut dest = fs::File::create(&path)?;
        let mut src = file.file.file.reopen()?; // Reopen the temp file
        std::io::copy(&mut src, &mut dest)?;
        let file_model = new_file_model(file.file_name.clone(), path.clone());
        let res = self.repository.create(file_model).await;
        match res {
            Ok(id) => Ok(CreateFileResponse {
                id: id.to_string(),
                file_name: file.file_name.to_string(),
                file_url: base_url.to_string() + &path,
            }),
            Err(e) => Err(ErrorInternalServerError(format!(
                "Failed to create file: {}",
                e
            ))),
        }
    }

    async fn find_all(&self) -> Result<FindAllFileResponse, Error> {
        let files = self.repository.find_all().await;
        let base_url = &self.base_url;
        match files {
            Ok(rows) => {
                let mut files = vec![];
                for row in rows {
                    files.push(FindOneFileResponse {
                        id: row.id.unwrap(),
                        file_name: row.file_name,
                        file_url:  base_url.clone() + &row.file_url,
                    });
                }
                Ok(FindAllFileResponse { files })
            }
            Err(e) => Err(ErrorInternalServerError(format!(
                "Failed to find all files: {}",
                e
            ))),
        }
    }
}
