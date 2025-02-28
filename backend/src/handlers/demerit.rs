use crate::database::db;
use crate::models::ErrorResponse;
use actix_web::{get, HttpResponse, Responder};
use rusqlite::params;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct DemeritTimePoint {
    pub date: String,
    pub count: i32,
}

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

#[derive(Serialize)]
pub struct DemeritCategoryCount {
    pub category_name: String,
    pub count: i32,
}

#[derive(Serialize)]
pub struct GradeDemeritCount {
    pub grade: i32,
    pub count: i32,
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

#[get("/demerit_distribution")]
pub async fn get_demerit_distribution() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Get distribution by category
    let mut category_stmt = match conn.prepare(
        "SELECT c.category_name, COUNT(*) as count
         FROM demerit_records dr
         JOIN demerit_categories c ON dr.category_id = c.category_id
         GROUP BY c.category_name
         ORDER BY count DESC",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let categories: Result<Vec<DemeritCategoryCount>, _> = category_stmt
        .query_map([], |row| {
            Ok(DemeritCategoryCount {
                category_name: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    let categories = match categories {
        Ok(categories) => categories,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to fetch category distribution: {}", e),
            })
        }
    };

    // Get distribution by grade level
    let mut grade_stmt = match conn.prepare(
        "SELECT s.grade_level, COUNT(*) as count
         FROM demerit_records dr
         JOIN students s ON dr.student_id = s.student_id
         GROUP BY s.grade_level
         ORDER BY s.grade_level",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let grades: Result<Vec<GradeDemeritCount>, _> = grade_stmt
        .query_map([], |row| {
            Ok(GradeDemeritCount {
                grade: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    let grades = match grades {
        Ok(grades) => grades,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to fetch grade distribution: {}", e),
            })
        }
    };

    // Return both sets of data
    HttpResponse::Ok().json(json!({
        "categories": categories,
        "grades": grades,
    }))
}

#[get("/demerit_trend")]
pub async fn get_demerit_trend() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Get demerit counts grouped by date
    let mut stmt = match conn.prepare(
        "SELECT
            strftime('%Y-%m-%d', date_issued) as date,
            COUNT(*) as count
         FROM demerit_records
         GROUP BY date
         ORDER BY date ASC
         LIMIT 60", // Last 60 days with data
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let trend_data: Result<Vec<DemeritTimePoint>, _> = stmt
        .query_map([], |row| {
            Ok(DemeritTimePoint {
                date: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    let trend_data = match trend_data {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to fetch trend data: {}", e),
            })
        }
    };

    HttpResponse::Ok().json(trend_data)
}
