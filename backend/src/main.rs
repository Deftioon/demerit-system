use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod database;
mod models;

use crate::database::{db, init_db};
use models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest, UserResponse};

#[derive(Serialize)]
struct TimeResponse {
    current_time: String,
}

async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);
    // Dummy authentication logic
    if req.email == "test@example.com" && req.password == "password123" {
        let response = AuthResponse {
            token: "dummy_jwt_token".to_string(),
            user: UserResponse {
                id: "1".to_string(),
                email: req.email.clone(),
                username: Some("testuser".to_string()),
                first_name: Some("Test".to_string()),
                last_name: Some("User".to_string()),
            },
        };
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            message: "Invalid credentials".to_string(),
        })
    }
}

async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);
    // Dummy registration logic
    if req.email.contains('@') {
        let response = AuthResponse {
            token: "new_dummy_jwt_token".to_string(),
            user: UserResponse {
                id: "2".to_string(),
                email: req.email.clone(),
                username: req.username.clone(),
                first_name: req.first_name.clone(),
                last_name: req.last_name.clone(),
            },
        };
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::BadRequest().json(ErrorResponse {
            message: "Invalid email format".to_string(),
        })
    }
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
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(get_time)
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
