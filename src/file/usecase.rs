use super::{
    domain::new_file_model,
    dto::{CreateFileRequest, CreateFileResponse, FindAllFileResponse, FindOneFileResponse},
    repository::{FileRepository, FileRepositoryTrait},
};
use actix_web::{error::ErrorInternalServerError, Error};
use async_trait::async_trait;

pub struct FileUsecase {
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
        FileUsecase { repository }
    }

    async fn create(&self, file: CreateFileRequest) -> Result<CreateFileResponse, Error> {
        println!("{:?}", file);
        // Validate input fields
        // let file_name = file
        //     .file_name
        //     .ok_or_else(|| ErrorInternalServerError("file_name is required"))?;
        // let file_url = file
        //     .file_url
        //     .ok_or_else(|| ErrorInternalServerError("file_url is required"))?;

        let file_model = new_file_model(file.file_name.clone(), file.file_url.clone());
        let res = self.repository.create(file_model).await;
        match res {
            Ok(id) => Ok(CreateFileResponse {
                id: id.to_string(),
                file_name: file.file_name.as_str().to_string(),
                file_url: file.file_url.as_str().to_string(),
            }),
            Err(e) => Err(ErrorInternalServerError(format!(
                "Failed to create file: {}",
                e
            ))),
        }
    }

    async fn find_all(&self) -> Result<FindAllFileResponse, Error> {
        let files = self.repository.find_all().await;
        match files {
            Ok(rows) => {
                let mut files = vec![];
                for row in rows {
                    files.push(FindOneFileResponse {
                        id: row.id.unwrap(),
                        file_name: row.file_name,
                        file_url: row.file_url,
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
