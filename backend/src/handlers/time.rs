use actix_web::{get, web, Responder};
use chrono::Local;
use serde::Serialize;

#[derive(Serialize)]
pub struct TimeResponse {
    current_time: String,
}

#[get("/time")]
pub async fn get_time() -> impl Responder {
    let current_time = Local::now().to_string();
    web::Json(TimeResponse { current_time })
}
