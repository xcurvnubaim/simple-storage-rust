use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{http::header::{ContentDisposition, DispositionType}, web, Error, HttpRequest};
use rusqlite::Connection;
use std::{path::PathBuf, sync::{Arc, Mutex}};

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
        file: MultipartForm<CreateFileRequest>,
    ) -> Result<web::Json<CreateFileResponse>, Error> {
        println!("Create file {}", file.file_name.as_str());
        let res = data.usecase.create(file.into_inner()).await;
        match res {
            Ok(data) => Ok(web::Json(data)),
            Err(e) => Err(e),
        }
    }

    pub async fn find_all(data: web::Data<Self>) -> Result<web::Json<FindAllFileResponse>, Error> {
        println!("Find all files");
        let res = data.usecase.find_all().await;
        match res {
            Ok(data) => Ok(web::Json(data)),
            Err(e) => Err(e),
        }
    }

    pub async fn get_file(data: web::Data<Self>, req: HttpRequest) -> actix_web::Result<NamedFile> {
        println!("Find all files");
        let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    
        let file = NamedFile::open(path)?;
        let file = file.set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        });
    
        Ok(file)
    }
}

pub fn file_routes(cfg: &mut web::ServiceConfig, db: Arc<Mutex<Connection>>) {
    let file_repository = FileRepository::new(db.clone()); // Pass shared DB connection
    let file_usecase = FileUsecase::new(file_repository);
    let file_handler = web::Data::new(FileHandler::new(file_usecase));

    cfg.service(
        web::scope("/file")
            .app_data(file_handler.clone())
            .route("", web::post().to(FileHandler::create))
            .route("", web::get().to(FileHandler::find_all))
            .route("/{filename:.*}", web::get().to(FileHandler::get_file))
    );
}
