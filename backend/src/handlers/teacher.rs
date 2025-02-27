use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::params;
use serde_json::json;

use crate::database::db;
use crate::models::{ErrorResponse, NewDemeritRecord, TeacherRecord};

#[get("/teacher_data")]
pub async fn get_teacher_data() -> impl Responder {
    let records = vec![TeacherRecord {
        id: 1,
        student_name: String::from("John Doe"),
        category: String::from("Late to Class"),
        points: 1,
        date_issued: String::from("2024-01-20"),
    }];
    HttpResponse::Ok().json(records)
}

pub async fn add_demerit(req: web::Json<NewDemeritRecord>) -> impl Responder {
    println!("Received demerit request: {:?}", req);

    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            });
        }
    };

    // First verify the student exists
    let student_exists: bool = match conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM students WHERE student_id = ?1)",
        params![req.student_id],
        |row| row.get(0),
    ) {
        Ok(exists) => exists,
        Err(e) => {
            eprintln!("Error checking student existence: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: format!("Error verifying student: {}", e),
            });
        }
    };

    if !student_exists {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "Student not found".to_string(),
        });
    }

    // Verify the category exists
    let category_exists: bool = match conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM demerit_categories WHERE category_id = ?1)",
        params![req.category_id],
        |row| row.get(0),
    ) {
        Ok(exists) => exists,
        Err(e) => {
            eprintln!("Error checking category existence: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: format!("Error verifying category: {}", e),
            });
        }
    };

    if !category_exists {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "Category not found".to_string(),
        });
    }

    // Insert the demerit record
    match conn.execute(
        "INSERT INTO demerit_records (student_id, teacher_id, category_id, points, description)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            req.student_id,
            1, // TODO: Get actual teacher_id from session
            req.category_id,
            req.points,
            req.description
        ],
    ) {
        Ok(_) => {
            println!("Successfully added demerit record");
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Demerit record added successfully"
            }))
        }
        Err(e) => {
            eprintln!("Error inserting demerit record: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to add demerit: {}", e),
            })
        }
    }
}
