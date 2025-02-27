use bcrypt::{hash, DEFAULT_COST};
use rusqlite::{params, Connection, Result};
use std::fs;
use std::path::Path;

pub fn create_admin_account(conn: &rusqlite::Connection) -> Result<()> {
    // Check if admin account already exists
    let admin_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = 'admin@edu.my')",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !admin_exists {
        let password_hash = hash("admin123", DEFAULT_COST)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        conn.execute(
            "INSERT INTO users (username, password_hash, email, user_type, first_name, last_name)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                "admin",
                password_hash,
                "admin@edu.my",
                "admin",
                "System",
                "Administrator"
            ],
        )?;

        println!("Admin account created successfully!");
    }

    Ok(())
}

pub fn create_sample_teacher(conn: &rusqlite::Connection) -> Result<()> {
    // Check if teacher account already exists
    let teacher_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = 'teacher@edu.my')",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !teacher_exists {
        // Create teacher user
        let password_hash = hash("teacher123", DEFAULT_COST)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        conn.execute(
            "INSERT INTO users (username, password_hash, email, user_type, first_name, last_name)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                "teacher",
                password_hash,
                "teacher@edu.my",
                "teacher",
                "John",
                "Smith"
            ],
        )?;

        // Get the user_id for the newly created teacher
        let user_id: i32 = conn.query_row(
            "SELECT user_id FROM users WHERE email = 'teacher@edu.my'",
            [],
            |row| row.get(0),
        )?;

        // Create the teacher record
        conn.execute(
            "INSERT INTO teachers (user_id, subject, department)
             VALUES (?1, ?2, ?3)",
            params![user_id, "Math", "Science"],
        )?;

        println!("Teacher account created successfully!");
    }

    Ok(())
}

pub fn initialize_database() -> Result<()> {
    let db_path = "demerit.db";

    // Check if database already exists
    if !Path::new(db_path).exists() {
        // Create database directory if it doesn't exist
        fs::create_dir_all("database").expect("Failed to create database directory");
        println!("Creating database");

        // Create a new database connection
        let conn = Connection::open(db_path)?;
        println!("Connecting...");

        // Read the schema SQL file
        let schema =
            fs::read_to_string("src/database/schema.sql").expect("Failed to read schema file");
        println!("Reading Schema");

        // Execute the schema SQL
        conn.execute_batch(&schema)?;
        println!("Executing Schema");

        //  Create admin account
        create_admin_account(&conn)?;
        create_sample_teacher(&conn)?;

        println!("Database initialized successfully!");
    }

    Ok(())
}

pub fn reset_database() -> Result<()> {
    let db_path = "database/demerit.db";

    // Remove existing database if it exists
    if Path::new(db_path).exists() {
        fs::remove_file(db_path).expect("Failed to remove existing database");
    }

    // Initialize fresh database
    initialize_database()
}
