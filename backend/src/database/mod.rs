pub mod db;
pub mod init_db;

pub fn initialize_db() {
    init_db::initialize_database().unwrap();
}

pub fn connectto_db() {
    match db::get_db_connection() {
        Ok(_) => println!("Connected to the database"),
        Err(e) => eprintln!("Failed to connect to the database: {}", e),
    }
}
