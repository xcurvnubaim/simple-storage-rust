use async_trait::async_trait;
use rusqlite::Connection;

pub struct SqliteDB {
    pub conn: Connection,
}

#[async_trait]
pub trait SqliteDBTrait {
    async fn init() -> Self;
}

#[async_trait]
impl SqliteDBTrait for SqliteDB {
    async fn init() -> Self {
        let conn =
            Connection::open("database/database.db").expect("Failed to open SQLite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                file_name TEXT NOT NULL,
                file_url TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create files table");

        SqliteDB { conn }
    }
}
