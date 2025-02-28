use crate::database::db;
use crate::models::{ErrorResponse, StudentRecord};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StudentOption {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserIdRequest {
    pub user_id: i32,
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
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to fetch students: {}", e),
        }),
    }
}

#[derive(Serialize)]
pub struct StudentDemeritDetail {
    pub demerit_id: i32,
    pub category_name: String,
    pub points: i32,
    pub teacher_name: String,
    pub description: String,
    pub date_issued: String,
}

#[get("/student_demerits/{student_id}")]
pub async fn get_student_demerits(path: web::Path<i32>) -> impl Responder {
    let student_id = path.into_inner();

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
            dr.demerit_id,
            c.category_name,
            dr.points,
            (SELECT first_name || ' ' || last_name FROM users WHERE user_id = t.user_id) as teacher_name,
            dr.description,
            dr.date_issued
        FROM
            demerit_records dr
        JOIN
            demerit_categories c ON dr.category_id = c.category_id
        JOIN
            teachers t ON dr.teacher_id = t.teacher_id
        WHERE
            dr.student_id = ?1
        ORDER BY
            dr.date_issued DESC
    "#;

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let demerits = match stmt.query_map(params![student_id], |row| {
        Ok(StudentDemeritDetail {
            demerit_id: row.get(0)?,
            category_name: row.get(1)?,
            points: row.get(2)?,
            teacher_name: row.get(3)?,
            description: row.get(4)?,
            date_issued: row.get(5)?,
        })
    }) {
        Ok(mapped) => {
            let collected: Result<Vec<StudentDemeritDetail>, _> = mapped.collect();
            match collected {
                Ok(demerits) => demerits,
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

    HttpResponse::Ok().json(demerits)
}

#[post("/my_demerits")]
pub async fn get_my_demerits(req: web::Json<UserIdRequest>) -> impl Responder {
    let user_id = req.user_id;

    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Get student_id from the user email
    let student_id: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT student_id FROM students WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    );

    let student_id = match student_id {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                message: "Student not found".to_string(),
            })
        }
    };

    // Now get the demerits for this student
    let query = r#"
            SELECT
                dr.demerit_id,
                c.category_name,
                dr.points,
                (SELECT first_name || ' ' || last_name FROM users WHERE user_id = t.user_id) as teacher_name,
                dr.description,
                dr.date_issued
            FROM
                demerit_records dr
            JOIN
                demerit_categories c ON dr.category_id = c.category_id
            JOIN
                teachers t ON dr.teacher_id = t.teacher_id
            WHERE
                dr.student_id = ?1
            ORDER BY
                dr.date_issued DESC
        "#;

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let demerits = match stmt.query_map(params![student_id], |row| {
        Ok(StudentDemeritDetail {
            demerit_id: row.get(0)?,
            category_name: row.get(1)?,
            points: row.get(2)?,
            teacher_name: row.get(3)?,
            description: row.get(4)?,
            date_issued: row.get(5)?,
        })
    }) {
        Ok(mapped) => {
            let collected: Result<Vec<StudentDemeritDetail>, _> = mapped.collect();
            match collected {
                Ok(demerits) => demerits,
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

    HttpResponse::Ok().json(demerits)
}

// Also add an endpoint to get basic student info
#[post("/my_student_info")]
pub async fn get_my_student_info(req: web::Json<UserIdRequest>) -> impl Responder {
    let user_id = req.user_id;

    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Get student info from the user email
    let result = conn.query_row(
        "SELECT s.student_id, s.grade_level, s.class_section
             FROM students s
             WHERE s.user_id = ?1",
        params![user_id],
        |row| {
            Ok(serde_json::json!({
                "student_id": row.get::<_, i32>(0)?,
                "grade_level": row.get::<_, i32>(1)?,
                "class_section": row.get::<_, String>(2)?
            }))
        },
    );

    match result {
        Ok(student_info) => HttpResponse::Ok().json(student_info),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "Student not found".to_string(),
        }),
    }
}
