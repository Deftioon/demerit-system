use actix_cors::Cors;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

mod database;
mod handlers;
mod middleware;
mod models;
mod services;

use crate::database::{db, init_db};
use crate::services::auth;
use models::{
    AdminUserRecord, AuthResponse, DemeritRecord, ErrorResponse, LoginRequest, NewDemeritRecord,
    ParentRecord, RegisterRequest, StudentRecord, TeacherRecord, UserResponse,
};

#[derive(Serialize)]
struct TimeResponse {
    current_time: String,
}

#[derive(Serialize)]
struct StudentOption {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct CategoryOption {
    id: i32,
    name: String,
    default_points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct StudentInfo {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AdminUserData {
    user_id: i32,
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    user_type: String,
    children: Vec<StudentInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUserRoleRequest {
    user_id: i32,
    new_role: String,
}

async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);

    match auth::auth_request(req.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}

async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    println!("Login attempt for email: {}", req.email);
    match auth::register(req.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let current_time = Local::now().to_string();

    web::Json(TimeResponse { current_time })
}

fn initialize_db() {
    init_db::initialize_database().unwrap();
}

fn connectto_db() {
    match db::get_db_connection() {
        Ok(_) => println!("Connected to the database"),
        Err(e) => eprintln!("Failed to connect to the database: {}", e),
    }
}

#[get("/student_data")]
async fn get_student_data() -> impl Responder {
    let records = vec![StudentRecord {
        id: 1,
        category: String::from("Incomplete Homework"),
        points: 1,
        teacher_name: String::from("Mrs. Smith"),
        date_issued: String::from("2024-01-19"),
    }];
    HttpResponse::Ok().json(records)
}

#[get("/parent_data")]
async fn get_parent_data() -> impl Responder {
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

// Rust
#[put("/update_user")]
async fn update_user(req: web::Json<AdminUserRecord>) -> impl Responder {
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Execute update for user general details
    let result = conn.execute(
        "UPDATE users SET first_name = ?1, last_name = ?2, email = ?3, username = ?4, user_type = ?5 WHERE user_id = ?6",
        rusqlite::params![
            req.first_name,
            req.last_name,
            req.email,
            req.username,
            req.user_type,
            req.user_id
        ],
    );

    match result {
        Ok(updated) if updated > 0 => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User updated successfully"
        })),
        Ok(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "User not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to update user: {}", e),
        }),
    }
}

async fn get_demerit_categories() -> impl Responder {
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

#[put("/update_user_role")]
async fn update_user_role(req: web::Json<UpdateUserRoleRequest>) -> impl Responder {
    println!("Received request to update role: {:?}", req);
    let conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: format!("Database connection error: {}", e),
            })
        }
    };

    // Execute update for user role
    let result = conn.execute(
        "UPDATE users SET user_type = ?1 WHERE user_id = ?2",
        rusqlite::params![req.new_role, req.user_id],
    );

    match result {
        Ok(updated) if updated > 0 => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User role updated successfully"
        })),
        Ok(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "User not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: format!("Failed to update user role: {}", e),
        }),
    }
}

async fn get_students() -> impl Responder {
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

async fn add_demerit(req: web::Json<NewDemeritRecord>) -> impl Responder {
    println!("Received demerit request: {:?}", req); // Add logging

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

// Add these endpoints and register them in your main function

#[derive(Debug, Serialize, Deserialize)]
struct ParentData {
    parent_id: i32,
    name: String,
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParentStudentRelationship {
    parent_id: i32,
    student_id: i32,
}

async fn get_parents() -> impl Responder {
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

async fn add_parent_student(req: web::Json<ParentStudentRelationship>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_db();
    connectto_db();
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(get_time)
            .service(get_student_data)
            .service(get_parent_data)
            .service(handlers::admin::get_admin_data)
            .service(handlers::admin::update_user)
            .service(update_user_role)
            .service(handlers::demerit::get_demerit_history)
            .service(handlers::teacher::get_teacher_data)
            .service(handlers::teacher::get_student_demerit_summary)
            .service(handlers::student::get_student_demerits)
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .service(handlers::student::get_my_demerits)
            .service(handlers::student::get_my_student_info)
            .service(handlers::parent::get_parent_children_summary)
            //TODO: HANDLERS currently do not return AuthResponse as required.
            // .service(handlers::auth::login)
            // .service(handlers::auth::register)
            .route("/add_demerit", web::post().to(add_demerit))
            .route("/students", web::get().to(get_students))
            .route("/demerit-categories", web::get().to(get_demerit_categories))
            .route("/parents", web::get().to(get_parents))
            .route("/add_parent_student", web::post().to(add_parent_student))
            .route(
                "/update_parent_students",
                web::post().to(handlers::parent::update_parent_students),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
