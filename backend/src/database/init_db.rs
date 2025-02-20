use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

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
