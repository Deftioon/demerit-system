use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest};
use crate::services::auth;

pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);

    match auth::auth_request(req.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}

pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);
    match auth::register(req.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}
