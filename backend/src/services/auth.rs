use crate::database::db;
use crate::models;
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use rusqlite::params;

pub fn auth_request(req: models::LoginRequest) -> Result<models::AuthResponse, String> {
    // Get DB connection
    let conn = db::get_db_connection().map_err(|e| format!("Database connection error: {}", e))?;

    // Query to fetch user details
    let user = conn
        .query_row(
            "SELECT user_id, username, password_hash, email, user_type, first_name, last_name
             FROM users WHERE email = ?1",
            params![req.email],
            |row| {
                Ok(models::User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    password_hash: row.get(2)?,
                    email: row.get(3)?,
                    user_type: row.get(4)?,
                    first_name: row.get(5)?,
                    last_name: row.get(6)?,
                })
            },
        )
        .map_err(|_| "User not found".to_string())?;

    println!("Found user with hash: {}", user.password_hash); // Debug line
    println!("Comparing with password: {}", req.password); // Debug line

    // Verify password
    let is_valid = verify(&req.password, &user.password_hash)
        .map_err(|e| format!("Password verification error: {}", e))?;

    println!("Password validation result: {}", is_valid); // Debug line

    if !is_valid {
        return Err("Invalid password".to_string());
    }

    Ok(models::AuthResponse {
        token: "10281".to_string(),
        user: models::UserResponse {
            id: user.id.to_string(),
            email: req.email,
            username: req.username,
            first_name: req.first_name,
            last_name: req.last_name,
            permissions: user.user_type,
        },
    })
}

pub fn register(req: models::RegisterRequest) -> Result<models::AuthResponse, String> {
    let conn = db::get_db_connection().map_err(|e| format!("Database connection error: {}", e))?;

    let password_hash =
        hash(req.password, DEFAULT_COST).map_err(|e| format!("Password hashing error: {}", e))?;

    let sql = "INSERT INTO users
                   (username, password_hash, email, user_type, first_name, last_name)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                   RETURNING user_id"; // This will return the newly created user_id

    let user_id: i32 = conn
        .query_row(
            sql,
            params![
                req.username,
                password_hash,
                req.email,
                "parent".to_string(), // Assuming it is parent register
                req.first_name,
                req.last_name,
            ],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(models::AuthResponse {
        token: "10281".to_string(),
        user: models::UserResponse {
            id: user_id.to_string(),
            email: req.email,
            username: req.username,
            first_name: req.first_name,
            last_name: req.last_name,
            permissions: "parent".to_string(),
        },
    })
}
