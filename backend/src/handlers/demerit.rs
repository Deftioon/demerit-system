use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::database::db;
use crate::models::ErrorResponse;

#[derive(Serialize)]
pub struct CategoryOption {
    pub id: i32,
    pub name: String,
    pub default_points: i32,
}

pub async fn get_demerit_categories() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    let mut stmt = match conn.prepare(
        "SELECT category_id, category_name, default_points
         FROM demerit_categories",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let categories: Result<Vec<CategoryOption>, _> = stmt
        .query_map([], |row| {
            Ok(CategoryOption {
                id: row.get(0)?,
                name: row.get(1)?,
                default_points: row.get(2)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    match categories {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to fetch categories: {}", e),
        }),
    }
}
