use rusqlite::{Connection, Result};

pub fn get_db_connection() -> Result<Connection> {
    Connection::open("demerit.db")
}

pub fn execute_sql(conn: Connection, query: &str) -> Result<()> {
    match conn.execute(query, [1i32]) {
        Ok(updated) => {
            println!("Updated {} rows", updated);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            Err(e)
        }
    }
}
