use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::database::db;
use crate::models::{ErrorResponse, StudentRecord};

#[derive(Serialize)]
pub struct StudentOption {
    pub id: i32,
    pub name: String,
}

#[get("/student_data")]
pub async fn get_student_data() -> impl Responder {
    let records = vec![StudentRecord {
        id: 1,
        category: String::from("Incomplete Homework"),
        points: 1,
        teacher_name: String::from("Mrs. Smith"),
        date_issued: String::from("2024-01-19"),
    }];
    HttpResponse::Ok().json(records)
}

pub async fn get_students() -> impl Responder {
    println!("Fetching students...");

    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    let mut stmt = match conn.prepare(
        "SELECT s.student_id, u.first_name || ' ' || u.last_name as full_name
         FROM students s
         JOIN users u ON s.user_id = u.user_id",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let students: Result<Vec<StudentOption>, _> = stmt
        .query_map([], |row| {
            Ok(StudentOption {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    match students {
        Ok(students) => {
            println!("Found {} students", students.len());
            HttpResponse::Ok().json(students)
        },
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to fetch students: {}", e),
        }),
    }
}
