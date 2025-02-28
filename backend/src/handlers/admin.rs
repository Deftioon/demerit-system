use actix_web::{get, put, web, HttpResponse, Responder};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::database::db;
use crate::models::{AdminUserRecord, ErrorResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserData {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: String,
    pub created_at: String,            // Make sure this field exists
    pub grade_level: Option<i32>,      // Make sure this is Option<i32>
    pub class_section: Option<String>, // Make sure this is Option<String>
    pub total_demerits: i32,           // Make sure this field exists
    pub children: Vec<StudentInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub user_id: i32,
    pub new_role: String,
}

#[get("/admin_data")]
pub async fn get_admin_data() -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Create a more comprehensive query that includes all necessary fields
    // Note that we're using LEFT JOIN to get student-specific data for students
    let query = r#"
        SELECT
            u.user_id,
            u.username,
            u.email,
            u.first_name,
            u.last_name,
            u.user_type,
            u.created_at,
            s.grade_level,
            s.class_section,
            (SELECT COALESCE(SUM(dr.points), 0) FROM demerit_records dr
             JOIN students s2 ON dr.student_id = s2.student_id
             WHERE s2.user_id = u.user_id) as total_demerits
        FROM users u
        LEFT JOIN students s ON u.user_id = s.user_id
    "#;

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Query preparation error: {}", e),
            })
        }
    };

    let user_result = stmt.query_map([], |row| {
        // Extract the grade_level and class_section values explicitly
        let grade_level: Option<i32> = row.get(7)?;
        let class_section: Option<String> = row.get(8)?;
        let total_demerits: i32 = row.get(9)?;

        // Debug logging to see what's coming from the database
        println!(
            "User {} grade_level: {:?}, class_section: {:?}",
            row.get::<_, i32>(0)?,
            grade_level,
            class_section
        );

        Ok(AdminUserData {
            user_id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            user_type: row.get(5)?,
            created_at: row.get::<_, String>(6)?,
            grade_level,   // Include the grade_level
            class_section, // Include the class_section
            total_demerits,
            children: Vec::new(), // Will be populated for parents later
        })
    });

    let mut users = match user_result {
        Ok(mapped) => {
            let collected: Result<Vec<AdminUserData>, _> = mapped.collect();
            match collected {
                Ok(users) => users,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Error collecting users: {}", e),
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

    // Fetch children for parents
    for user in &mut users {
        if user.user_type == "parent" {
            let children_query = r#"
                SELECT
                    s.student_id,
                    u.first_name || ' ' || u.last_name as full_name
                FROM parent_student ps
                JOIN parents p ON ps.parent_id = p.parent_id
                JOIN students s ON ps.student_id = s.student_id
                JOIN users u ON s.user_id = u.user_id
                WHERE p.user_id = ?1
            "#;

            let mut children_stmt = match conn.prepare(children_query) {
                Ok(stmt) => stmt,
                Err(_) => continue, // Skip if query fails
            };

            let children_result = children_stmt.query_map([&user.user_id], |row| {
                Ok(StudentInfo {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            });

            if let Ok(mapped_children) = children_result {
                if let Ok(children) = mapped_children.collect::<Result<Vec<StudentInfo>, _>>() {
                    user.children = children;
                }
            }
        }
    }

    HttpResponse::Ok().json(users)
}

#[put("/update_user")]
pub async fn update_user(req: web::Json<AdminUserRecord>) -> impl Responder {
    println!(
        "UPDATE USER HANDLER CALLED with user_id: {} and user_type: {}",
        req.user_id, req.user_type
    );

    let mut conn = match db::get_db_connection() {
        Ok(conn) => {
            println!("Database connection successful");
            conn
        }
        Err(e) => {
            println!("Database connection failed: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            });
        }
    };

    // Start a transaction to ensure all operations succeed or fail together
    let tx = match conn.transaction() {
        Ok(tx) => {
            println!("Transaction started successfully");
            tx
        }
        Err(e) => {
            println!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to start transaction: {}", e),
            });
        }
    };

    // Step 1: Get the current user data to check if role is changing
    let current_user_type: String = match tx.query_row(
        "SELECT user_type FROM users WHERE user_id = ?1",
        params![req.user_id],
        |row| row.get(0),
    ) {
        Ok(user_type) => {
            println!("Current user_type: {}", user_type);
            user_type
        }
        Err(e) => {
            println!("User not found: {}", e);
            return HttpResponse::NotFound().json(ErrorResponse {
                message: format!("User not found: {}", e),
            });
        }
    };

    // Step 2: Update the users table
    match tx.execute(
        "UPDATE users SET first_name = ?1, last_name = ?2, email = ?3, username = ?4, user_type = ?5 WHERE user_id = ?6",
        params![
            req.first_name,
            req.last_name,
            req.email,
            req.username,
            req.user_type,
            req.user_id
        ],
    ) {
        Ok(updated) => {
                    println!("Updated {} rows in users table", updated);
                    if updated == 0 {
                        return HttpResponse::NotFound().json(ErrorResponse {
                            message: "User not found".to_string(),
                        });
                    }
                }
                Err(e) => {
                    println!("Failed to update user: {}", e);
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to update user: {}", e),
                    });
                }
        _ => {}
    }

    // Step 3: Handle role-specific changes if role has changed
    if current_user_type != req.user_type {
        println!(
            "Role changed from {} to {}",
            current_user_type, req.user_type
        );

        // Remove from previous role-specific table
        match current_user_type.as_str() {
            "teacher" => {
                if let Err(e) = tx.execute(
                    "DELETE FROM teachers WHERE user_id = ?1",
                    params![req.user_id],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to remove user from teachers table: {}", e),
                    });
                }
            }
            "student" => {
                if let Err(e) = tx.execute(
                    "DELETE FROM students WHERE user_id = ?1",
                    params![req.user_id],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to remove user from students table: {}", e),
                    });
                }
            }
            "parent" => {
                // Get parent_id first
                let parent_id: Result<i32, rusqlite::Error> = tx.query_row(
                    "SELECT parent_id FROM parents WHERE user_id = ?1",
                    params![req.user_id],
                    |row| row.get(0),
                );

                if let Ok(parent_id) = parent_id {
                    // Delete parent-student relationships
                    if let Err(e) = tx.execute(
                        "DELETE FROM parent_student WHERE parent_id = ?1",
                        params![parent_id],
                    ) {
                        return HttpResponse::InternalServerError().json(ErrorResponse {
                            message: format!(
                                "Failed to remove parent-student relationships: {}",
                                e
                            ),
                        });
                    }
                }

                // Delete from parents table
                if let Err(e) = tx.execute(
                    "DELETE FROM parents WHERE user_id = ?1",
                    params![req.user_id],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to remove user from parents table: {}", e),
                    });
                }
            }
            _ => {}
        }

        // Add to new role-specific table
        match req.user_type.as_str() {
            "teacher" => {
                if let Err(e) = tx.execute(
                    "INSERT INTO teachers (user_id, subject, department) VALUES (?1, ?2, ?3)",
                    params![req.user_id, "Not Set", "Not Set"],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to add user to teachers table: {}", e),
                    });
                }
            }
            "student" => {
                let grade_level = req.grade_level.unwrap_or(0);
                let class_section = req.class_section.as_deref().unwrap_or("Not Set");

                if let Err(e) = tx.execute(
                    "INSERT INTO students (user_id, grade_level, class_section) VALUES (?1, ?2, ?3)",
                    params![req.user_id, grade_level, class_section],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to add user to students table: {}", e),
                    });
                }
            }
            "parent" => {
                if let Err(e) = tx.execute(
                    "INSERT INTO parents (user_id) VALUES (?1)",
                    params![req.user_id],
                ) {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Failed to add user to parents table: {}", e),
                    });
                }
            }
            _ => {}
        }
    }
    // Handle student info update if still a student
    else if req.user_type == "student" {
        let grade_level = req.grade_level.unwrap_or(0);
        let class_section = req.class_section.as_deref().unwrap_or("Not Set");

        if let Err(e) = tx.execute(
            "UPDATE students SET grade_level = ?1, class_section = ?2 WHERE user_id = ?3",
            params![grade_level, class_section, req.user_id],
        ) {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Failed to update student information: {}", e),
            });
        }
    }

    // Commit the transaction
    if let Err(e) = tx.commit() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to commit transaction: {}", e),
        });
    }

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User updated successfully"
    }))
}
