use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::database::db;
use crate::models::{ErrorResponse, ParentRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdRequest {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildSummary {
    pub student_id: i32,
    pub student_name: String,
    pub total_points: i32,
    pub recent_demerit: Option<String>,
    pub grade_level: Option<i32>,
    pub class_section: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkParentStudentRelationship {
    pub parent_id: i32,
    pub student_ids: Vec<i32>,
}

pub async fn update_parent_students(
    req: web::Json<BulkParentStudentRelationship>,
) -> impl Responder {
    let mut conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Start a transaction
    let tx = match conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to start transaction: {}", e),
            })
        }
    };

    // First, remove all existing relationships for this parent
    match tx.execute(
        "DELETE FROM parent_student WHERE parent_id = ?1",
        params![req.parent_id],
    ) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to remove existing relationships: {}", e),
            })
        }
    }

    // Then add all the new ones
    for student_id in &req.student_ids {
        match tx.execute(
            "INSERT INTO parent_student (parent_id, student_id) VALUES (?1, ?2)",
            params![req.parent_id, student_id],
        ) {
            Ok(_) => {}
            Err(e) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: format!(
                        "Failed to add relationship for student {}: {}",
                        student_id, e
                    ),
                })
            }
        }
    }

    // Commit the transaction
    match tx.commit() {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": format!("Updated parent-student relationships for parent ID {}", req.parent_id),
            "added_students": req.student_ids.len()
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to commit transaction: {}", e),
        })
    }
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

#[post("/parent_children_summary")]
pub async fn get_parent_children_summary(req: web::Json<UserIdRequest>) -> impl Responder {
    let user_id = req.user_id;
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Step 1: Get the parent_id from the user_id
    let parent_id: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT parent_id FROM parents WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    );

    let parent_id = match parent_id {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                message: "Parent not found".to_string(),
            })
        }
    };

    // Step 2: Get all children associated with this parent
    let query = r#"
        SELECT
            s.student_id,
            (SELECT first_name || ' ' || last_name FROM users WHERE user_id = s.user_id) AS student_name,
            COALESCE(SUM(dr.points), 0) AS total_points,
            (SELECT category_name FROM demerit_categories c
             JOIN demerit_records dr2 ON c.category_id = dr2.category_id
             WHERE dr2.student_id = s.student_id
             ORDER BY dr2.date_issued DESC
             LIMIT 1) AS recent_demerit,
            s.grade_level,
            s.class_section
        FROM
            parent_student ps
        JOIN
            students s ON ps.student_id = s.student_id
        LEFT JOIN
            demerit_records dr ON s.student_id = dr.student_id
        WHERE
            ps.parent_id = ?1
        GROUP BY
            s.student_id
        ORDER BY
            total_points DESC
    "#;

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let children = match stmt.query_map(params![parent_id], |row| {
        Ok(ChildSummary {
            student_id: row.get(0)?,
            student_name: row.get(1)?,
            total_points: row.get(2)?,
            recent_demerit: row.get(3)?,
            grade_level: row.get(4)?,
            class_section: row.get(5)?,
        })
    }) {
        Ok(mapped) => {
            let collected: Result<Vec<ChildSummary>, _> = mapped.collect();
            match collected {
                Ok(children) => children,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Error collecting records: {}", e),
                    })
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query execution error: {}", e),
            })
        }
    };

    HttpResponse::Ok().json(children)
}
