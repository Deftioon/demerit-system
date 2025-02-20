use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use chrono::Local;
use serde::Serialize;

mod database;
use crate::database::{db, init_db};

#[derive(Serialize)]
struct TimeResponse {
    current_time: String,
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let current_time = Local::now().to_string();

    web::Json(TimeResponse { current_time })
}

fn initialize_db() {
    init_db::initialize_database().unwrap();
}

fn connectto_db() {
    match db::get_db_connection() {
        Ok(_) => println!("Connected to the database"),
        Err(e) => eprintln!("Failed to connect to the database: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_db();
    connectto_db();
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(get_time)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
