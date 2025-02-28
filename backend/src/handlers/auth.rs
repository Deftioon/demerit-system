use actix_web::{cookie::time::Duration, cookie::Cookie, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest};
use crate::services::auth;

#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);

    match auth::auth_request(req.into_inner()) {
        Ok(response) => {
            // Create a cookie with the user email and proper configuration
            // Using the correct way to set max age with seconds
            let email_cookie = Cookie::build("user_email", response.user.email.clone())
                .path("/")
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Lax) // Allow cookies in same-site requests
                .max_age(Duration::hours(24)) // 24 hours in seconds: 24*60*60
                .finish();

            // Also create a cookie for user type for permission checks
            let user_type_cookie = Cookie::build("user_type", response.user.permissions.clone())
                .path("/")
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Lax)
                .max_age(Duration::hours(24)) // 24 hours in seconds
                .finish();

            // Print that we're setting these cookies
            println!(
                "Setting cookies for user: {} type: {}",
                response.user.email, response.user.permissions
            );

            // Return the response with cookies
            HttpResponse::Ok()
                .cookie(email_cookie)
                .cookie(user_type_cookie)
                .json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}

#[post("/register")]
pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);
    match auth::register(req.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}
