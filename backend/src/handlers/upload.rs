use crate::database::db;
use crate::models::ErrorResponse;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use csv::Reader;
use futures::{StreamExt, TryStreamExt};
use rand::{distributions::Alphanumeric, Rng};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct StudentCsvRecord {
    name: String,
    grade: i32,
    class: String,
    demerits: i32,
}

#[derive(Debug, Serialize)]
struct CsvProcessingResult {
    status: String,
    message: String,
    success_count: usize,
    failure_count: usize,
    errors: Vec<String>,
    generated_passwords: Vec<String>, // Add this field
}

// Function to generate a secure random password
fn generate_random_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// Internal function to process CSV data
async fn process_csv_data(file_path: &str) -> Result<CsvProcessingResult, String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File does not exist: {}", file_path));
    }

    // Open the file
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to open file: {}", e));
        }
    };

    // Create CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    // Get database connection
    let mut conn = match db::get_db_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(format!("Database connection error: {}", e));
        }
    };

    // Start a transaction
    let tx = match conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            return Err(format!("Failed to start transaction: {}", e));
        }
    };

    let mut success_count = 0;
    let mut failure_count = 0;
    let mut errors = Vec::new();
    let mut generated_passwords = Vec::new();

    // Find the teacher for "Migration" demerits
    let teacher_id: i32 = match tx.query_row(
        "SELECT teacher_id FROM teachers
         JOIN users ON teachers.user_id = users.user_id
         LIMIT 1",
        [],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(e) => {
            return Err(format!("Failed to find a teacher: {}", e));
        }
    };

    // Find the demerit category for "Migration"
    let category_id: i32 = match tx.query_row(
        "SELECT category_id FROM demerit_categories
         WHERE category_name = 'Late to Class' LIMIT 1",
        [],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(e) => {
            return Err(format!("Failed to find a demerit category: {}", e));
        }
    };

    // Process each record
    for result in rdr.deserialize() {
        let record: StudentCsvRecord = match result {
            Ok(record) => record,
            Err(e) => {
                failure_count += 1;
                errors.push(format!("Error parsing CSV record: {}", e));
                continue;
            }
        };

        // Format the student name for username and email
        let name_parts: Vec<&str> = record.name.split_whitespace().collect();
        let username = record.name.replace(" ", "_").to_lowercase();
        let email = format!("{}@school.edu", username);

        let first_name = name_parts.first().unwrap_or(&"").to_string();
        let last_name = name_parts.get(1).unwrap_or(&"").to_string();

        let password = generate_random_password(12); // 12 characters is reasonably secure
        let password_hash = match hash(&password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => {
                failure_count += 1;
                errors.push(format!(
                    "Failed to hash password for {}: {}",
                    record.name, e
                ));
                continue;
            }
        };

        generated_passwords.push(format!("{}: {}", record.name, password));
        println!("Generated password for {}: {}", record.name, password);

        // 1. Insert user
        let user_id: i32;

        // First check if the user already exists
        match tx.query_row(
            "SELECT user_id FROM users WHERE username = ? OR email = ?",
            params![username, email],
            |row| row.get::<_, i32>(0),
        ) {
            Ok(existing_id) => {
                // User already exists
                user_id = existing_id;
            }
            Err(_) => {
                // User doesn't exist, create a new one
                match tx.query_row(
                    "INSERT INTO users (username, password_hash, email, user_type, first_name, last_name)
                     VALUES (?, ?, ?, 'student', ?, ?)
                     RETURNING user_id",
                    params![username, password_hash, email, first_name, last_name],
                    |row| row.get::<_, i32>(0),
                ) {
                    Ok(new_id) => {
                        user_id = new_id;
                    }
                    Err(e) => {
                        failure_count += 1;
                        errors.push(format!("Failed to create user {}: {}", record.name, e));
                        continue;
                    }
                }
            }
        }

        // 2. Check if student record exists
        let student_id: i32;

        match tx.query_row(
            "SELECT student_id FROM students WHERE user_id = ?",
            params![user_id],
            |row| row.get::<_, i32>(0),
        ) {
            Ok(existing_id) => {
                // Student record already exists
                student_id = existing_id;

                // Update grade and class
                if let Err(e) = tx.execute(
                    "UPDATE students SET grade_level = ?, class_section = ? WHERE student_id = ?",
                    params![record.grade, record.class, student_id],
                ) {
                    failure_count += 1;
                    errors.push(format!(
                        "Failed to update student record for {}: {}",
                        record.name, e
                    ));
                    continue;
                }
            }
            Err(_) => {
                // Student doesn't exist, create a new one
                match tx.query_row(
                    "INSERT INTO students (user_id, grade_level, class_section)
                     VALUES (?, ?, ?)
                     RETURNING student_id",
                    params![user_id, record.grade, record.class],
                    |row| row.get::<_, i32>(0),
                ) {
                    Ok(new_id) => {
                        student_id = new_id;
                    }
                    Err(e) => {
                        failure_count += 1;
                        errors.push(format!(
                            "Failed to create student record for {}: {}",
                            record.name, e
                        ));
                        continue;
                    }
                }
            }
        }

        // 3. Add demerits if specified
        if record.demerits > 0 {
            match tx.execute(
                "INSERT INTO demerit_records (student_id, teacher_id, category_id, points, description)
                 VALUES (?, ?, ?, ?, 'Data migration')",
                params![student_id, teacher_id, category_id, record.demerits],
            ) {
                Ok(_) => {}
                Err(e) => {
                    failure_count += 1;
                    errors.push(format!("Failed to add demerits for {}: {}", record.name, e));
                    continue;
                }
            }
        }

        success_count += 1;
    }

    // Commit the transaction
    if let Err(e) = tx.commit() {
        return Err(format!("Failed to commit transaction: {}", e));
    }

    Ok(CsvProcessingResult {
        status: "success".to_string(),
        message: "CSV processing completed".to_string(),
        success_count,
        failure_count,
        errors,
        generated_passwords,
    })
}

// Web handler for direct processing of a CSV file path
#[post("/process_csv")]
pub async fn process_csv_file(file_path: web::Json<String>) -> impl Responder {
    match process_csv_data(&file_path).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error_msg) => {
            HttpResponse::InternalServerError().json(ErrorResponse { message: error_msg })
        }
    }
}

// Upload and process CSV file
#[post("/upload_csv")]
pub async fn upload_csv(mut payload: Multipart) -> impl Responder {
    // Create uploads directory if it doesn't exist
    let upload_dir = "uploads";
    if !Path::new(upload_dir).exists() {
        fs::create_dir_all(upload_dir).unwrap();
    }

    // Process the multipart form data
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Check if this is a CSV file
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "unknown.csv".to_string());
        let file_ext = Path::new(&filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if file_ext != "csv" {
            return HttpResponse::BadRequest().json(ErrorResponse {
                message: "Only CSV files are allowed".to_string(),
            });
        }

        // Generate unique filename
        let uuid = Uuid::new_v4();
        let filepath = format!("{}/{}_{}", upload_dir, uuid, filename);

        // Create a file to write to
        let mut f = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(e) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: format!("Failed to create file: {}", e),
                });
            }
        };

        // Write the file
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        message: format!("Error while uploading file: {}", e),
                    });
                }
            };

            if let Err(e) = f.write_all(&data) {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: format!("Failed to write file: {}", e),
                });
            }
        }

        // Process the uploaded file
        match process_csv_data(&filepath).await {
            Ok(result) => {
                return HttpResponse::Ok().json(result);
            }
            Err(error_msg) => {
                return HttpResponse::InternalServerError()
                    .json(ErrorResponse { message: error_msg });
            }
        }
    }

    HttpResponse::BadRequest().json(ErrorResponse {
        message: "No file provided".to_string(),
    })
}
