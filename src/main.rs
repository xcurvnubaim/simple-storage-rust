use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use database::mongo::MongoDBTrait;

mod database;
mod file;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = database::mongo::MongoDB::init().await.db;    

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .configure(|cfg| {
                cfg.service(file::handler::file_routes(db.clone()));
            })
    })
    .bind(("127.0.0.1", 8080))? // Bind to localhost and port 8080
    .run()
    .await
}