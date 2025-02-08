use async_trait::async_trait;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

use super::domain::FileModel;

pub struct FileRepository {
    pub conn: Arc<Mutex<Connection>>,
}

#[async_trait]
pub trait FileRepositoryTrait {
    fn new(conn: Arc<Mutex<Connection>>) -> Self;
    async fn create(&self, file: FileModel) -> Result<i64>;
    async fn find_all(&self) -> Result<Vec<FileModel>>;
}

#[async_trait]
impl FileRepositoryTrait for FileRepository {
    fn new(conn: Arc<Mutex<Connection>>) -> Self {
        FileRepository { conn }
    }

    async fn create(&self, file: FileModel) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("INSERT INTO files (file_name, file_url) VALUES (?1, ?2)")?;
        stmt.execute(params![file.file_name, file.file_url])?;
        Ok(conn.last_insert_rowid())
    }

    async fn find_all(&self) -> Result<Vec<FileModel>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, file_name, file_url FROM files")?;
        let file_iter = stmt.query_map([], |row| {
            Ok(FileModel {
                id: Some(row.get(0)?),
                file_name: row.get(1)?,
                file_url: row.get(2)?,
            })
        })?;

        let mut files = vec![];
        for file in file_iter {
            files.push(file?);
        }
        Ok(files)
    }
}
