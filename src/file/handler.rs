use actix_web::{web, Scope, Error};
use mongodb::Database;

use crate::file::{
    dto::{CreateFileRequest, CreateFileResponse},
    repository::{FileRepository, FileRepositoryTrait},
    usecase::{FileUsecase, FileUsecaseTrait},
};

use super::dto::FindAllFileResponse;

pub struct FileHandler {
    pub usecase: FileUsecase,
}

impl FileHandler {
    pub fn new(usecase: FileUsecase) -> Self {
        FileHandler { usecase }
    }

    pub async fn create(
        data: web::Data<Self>,
        file: web::Json<CreateFileRequest>,
    ) -> Result<web::Json<CreateFileResponse>, Error> {
        println!("Create file");
        let res = data.usecase.create(file.into_inner()).await;
        match res {
            Ok(data) => Ok(web::Json(data)),
            Err(e) => Err(e),
        }
    }

    pub async fn find_all(
        data: web::Data<Self>,
    ) -> Result<web::Json<FindAllFileResponse>, Error> {
        println!("Find all files");
        let res = data.usecase.find_all().await;
        match res {
            Ok(data) => Ok(web::Json(data)),
            Err(e) => Err(e),
        }
    }
}

pub fn file_routes(db: Database) -> Scope {
    let file_repository = FileRepository::new(db);
    let file_usecase = FileUsecase::new(file_repository);
    let file_handler = web::Data::new(FileHandler::new(file_usecase));

    web::scope("/file")
        .app_data(file_handler.clone())
        .route("", web::post().to(FileHandler::create))
        .route("", web::get().to(FileHandler::find_all))
}
