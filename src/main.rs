use actix_web::{web, App, HttpServer};
use database::sqlite::{SqliteDB, SqliteDBTrait};
use dotenv::dotenv;
use std::sync::{Arc, Mutex};

mod database;
mod file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize SQLite connection inside Arc<Mutex<>>
    let db = Arc::new(Mutex::new(SqliteDB::init().await.conn));

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Store shared DB connection
            .configure(|cfg| file::handler::file_routes(cfg, db.clone())) // Pass it to handlers
    })
    .bind(("127.0.0.1", 3000))? // Bind to localhost and port 3000
    .run()
    .await
}
