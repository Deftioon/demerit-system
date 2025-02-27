use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::database::db;
use crate::models::{ErrorResponse, ParentRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentData {
    pub parent_id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentStudentRelationship {
    pub parent_id: i32,
    pub student_id: i32,
}

#[get("/parent_data")]
pub async fn get_parent_data() -> impl Responder {
    let records = vec![
        ParentRecord {
            id: 1,
            student_name: String::from("Jane Doe"),
            category: String::from("Dress Code Violation"),
            points: 1,
            teacher_name: String::from("Mr. Johnson"),
            date_issued: String::from("2024-01-18"),
        },
        ParentRecord {
            id: 2,
            student_name: String::from("Jane Doe"),
            category: String::from("Abuse of  E-Gadgets"),
            points: 4,
            teacher_name: String::from("Mr. Leo"),
            date_issued: String::from("2024-114-514"),
        },
    ];
    HttpResponse::Ok().json(records)
}

pub async fn get_parents() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    let mut stmt = match conn.prepare(
        "SELECT p.parent_id, u.first_name || ' ' || u.last_name as full_name, p.user_id
         FROM parents p
         JOIN users u ON p.user_id = u.user_id",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let parents: Result<Vec<ParentData>, _> = stmt
        .query_map([], |row| {
            Ok(ParentData {
                parent_id: row.get(0)?,
                name: row.get(1)?,
                user_id: row.get(2)?,
            })
        })
        .and_then(|mapped| mapped.collect());

    match parents {
        Ok(parents) => HttpResponse::Ok().json(parents),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to fetch parents: {}", e),
        }),
    }
}

pub async fn add_parent_student(req: web::Json<ParentStudentRelationship>) -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Check if relationship already exists
    let exists: bool = match conn.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM parent_student 
            WHERE parent_id = ?1 AND student_id = ?2
         )",
        params![req.parent_id, req.student_id],
        |row| row.get(0),
    ) {
        Ok(exists) => exists,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Error checking relationship: {}", e),
            })
        }
    };

    if exists {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "This parent-student relationship already exists".to_string(),
        });
    }

    // Add relationship
    match conn.execute(
        "INSERT INTO parent_student (parent_id, student_id) VALUES (?1, ?2)",
        params![req.parent_id, req.student_id],
    ) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Parent-student relationship added successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to add relationship: {}", e),
        }),
    }
}
