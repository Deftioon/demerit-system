use crate::database::db;
use crate::models::ErrorResponse;
use actix_web::{get, HttpResponse, Responder};
use rusqlite::params;
use serde::Serialize;

#[derive(Serialize)]
pub struct DemeritHistoryRecord {
    pub demerit_id: i32,
    pub student_name: String,
    pub category_name: String,
    pub points: i32,
    pub teacher_name: String,
    pub description: String,
    pub date_issued: String,
}

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

#[get("/demerit_history")]
pub async fn get_demerit_history() -> impl Responder {
    println!("Fetching demerit history");
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    let query = r#"
        SELECT
            d.demerit_id,
            (SELECT first_name || ' ' || last_name FROM users WHERE user_id = s.user_id) as student_name,
            c.category_name,
            d.points,
            (SELECT first_name || ' ' || last_name FROM users WHERE user_id = t.user_id) as teacher_name,
            d.description,
            d.date_issued
        FROM
            demerit_records d
        JOIN
            students s ON d.student_id = s.student_id
        JOIN
            teachers t ON d.teacher_id = t.teacher_id
        JOIN
            demerit_categories c ON d.category_id = c.category_id
        ORDER BY
            d.date_issued DESC
    "#;

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => {
            println!("Query prepared successfully");
            stmt
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let history_result = stmt.query_map([], |row| {
        let demerit_id: i32 = row.get(0)?;
        let student_name: String = row.get(1)?;
        let category_name: String = row.get(2)?;
        let points: i32 = row.get(3)?;
        let teacher_name: String = row.get(4)?;
        let description: String = row.get(5)?;
        let date_issued: String = row.get(6)?;

        println!(
            "Got record: ID={}, Student={}, Category={}",
            demerit_id, student_name, category_name
        );

        Ok(DemeritHistoryRecord {
            demerit_id,
            student_name,
            category_name,
            points,
            teacher_name,
            description,
            date_issued,
        })
    });

    match history_result {
        Ok(mapped_rows) => {
            let records: Result<Vec<DemeritHistoryRecord>, _> = mapped_rows.collect();
            match records {
                Ok(records) => {
                    println!("Found {} demerit records", records.len());
                    HttpResponse::Ok().json(records)
                }
                Err(e) => {
                    println!("Error collecting records: {}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to collect demerit records: {}", e),
                    })
                }
            }
        }
        Err(e) => {
            println!("Query execution failed: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query execution error: {}", e),
            })
        }
    }
}
