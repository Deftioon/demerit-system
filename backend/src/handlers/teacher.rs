use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::database::db;
use crate::models::{ErrorResponse, NewDemeritRecord, TeacherRecord};

#[derive(Serialize, Deserialize)]
pub struct StudentDemeritSummary {
    pub student_id: i32,
    pub student_name: String,
    pub total_points: i32,
    pub recent_demerit: Option<String>,
    pub grade_level: Option<i32>,
    pub class_section: Option<String>,
}

#[get("/student_demerit_summary")]
pub async fn get_student_demerit_summary() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Query to get summarized demerit points by student
    let query = r#"
        SELECT
            s.student_id,
            u.first_name || ' ' || u.last_name AS student_name,
            SUM(dr.points) AS total_points,
            (SELECT category_name FROM demerit_categories c
             JOIN demerit_records dr2 ON c.category_id = dr2.category_id
             WHERE dr2.student_id = s.student_id
             ORDER BY dr2.date_issued DESC
             LIMIT 1) AS recent_demerit,
            s.grade_level,
            s.class_section
        FROM
            students s
        JOIN
            users u ON s.user_id = u.user_id
        LEFT JOIN
            demerit_records dr ON s.student_id = dr.student_id
        GROUP BY
            s.student_id, u.first_name, u.last_name, s.grade_level, s.class_section
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

    let summaries = match stmt.query_map([], |row| {
        Ok(StudentDemeritSummary {
            student_id: row.get(0)?,
            student_name: row.get(1)?,
            total_points: row.get::<_, Option<i32>>(2)?.unwrap_or(0),
            recent_demerit: row.get(3)?,
            grade_level: row.get(4)?,
            class_section: row.get(5)?,
        })
    }) {
        Ok(mapped) => {
            let collected: Result<Vec<StudentDemeritSummary>, _> = mapped.collect();
            match collected {
                Ok(summaries) => summaries,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Error collecting summaries: {}", e),
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

    HttpResponse::Ok().json(summaries)
}

#[get("/teacher_data")]
pub async fn get_teacher_data() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Get a specific teacher_id - in a real app you'd get this from session
    let teacher_id: i32 = match conn.query_row(
        "SELECT teacher_id FROM teachers
         JOIN users ON teachers.user_id = users.user_id
         WHERE users.email = 'teacher@edu.my'",
        [],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to get teacher ID: {}", e),
            })
        }
    };

    // Query to get demerit records associated with this teacher
    let query = r#"
        SELECT
            dr.demerit_id,
            (SELECT first_name || ' ' || last_name FROM users WHERE user_id = s.user_id) as student_name,
            c.category_name,
            dr.points,
            dr.date_issued
        FROM
            demerit_records dr
        JOIN
            students s ON dr.student_id = s.student_id
        JOIN
            demerit_categories c ON dr.category_id = c.category_id
        WHERE
            dr.teacher_id = ?1
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

    let records = match stmt.query_map(params![teacher_id], |row| {
        Ok(TeacherRecord {
            id: row.get(0)?,
            student_name: row.get(1)?,
            category: row.get(2)?,
            points: row.get(3)?,
            date_issued: row.get(4)?,
        })
    }) {
        Ok(mapped) => {
            let collected: Result<Vec<TeacherRecord>, _> = mapped.collect();
            match collected {
                Ok(records) => records,
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

    let teacher_id: i32 = match conn.query_row(
        "SELECT teacher_id FROM teachers
             JOIN users ON teachers.user_id = users.user_id
             WHERE users.email = 'teacher@edu.my'",
        [],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error retrieving teacher ID: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Could not determine teacher ID".to_string(),
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
            teacher_id, // TODO: Get actual teacher_id from session
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
