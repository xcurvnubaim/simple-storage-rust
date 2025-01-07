use actix_web::{error::ErrorInternalServerError, Error};
use async_trait::async_trait;
use super::{domain::new_file_model, dto::{CreateFileRequest, CreateFileResponse}, repository::{FileRepository, FileRepositoryTrait}};


pub struct FileUsecase {
    pub repository: FileRepository,
}

#[async_trait]
pub trait FileUsecaseTrait {
    fn new(repository: FileRepository) -> Self;
    async fn create(&self, file: CreateFileRequest) -> Result<CreateFileResponse, Error>;
    // async fn find_all(&self) -> Vec<FileModel>;
}

#[async_trait]
impl FileUsecaseTrait for FileUsecase {
    fn new(repository: FileRepository) -> Self {
        FileUsecase { repository }
    }

    async fn create(&self, file: CreateFileRequest) -> Result<CreateFileResponse, Error> {
        // Validate input fields
        let file_name = file
            .file_name
            .ok_or_else(|| ErrorInternalServerError("file_name is required"))?;
        let file_url = file
            .file_url
            .ok_or_else(|| ErrorInternalServerError("file_url is required"))?;

        let file_model = new_file_model(file_name.clone(), file_url.clone());
        let res = self.repository.create(file_model).await;
        match res {
            Ok(_) => Ok(CreateFileResponse {
                id: "123".to_string(),
                file_name: file_name,
                file_url: file_url,
            }),
            Err(e) => Err(ErrorInternalServerError(format!(
                "Failed to create file: {}",
                e
            ))),
        }
    }

    // async fn find_all(&self) -> Vec<FileModel> {
    //     self.repository.find_all().await
    // }
}