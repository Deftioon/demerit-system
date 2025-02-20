use rusqlite::{Connection, Result};

pub fn get_db_connection() -> Result<Connection> {
    Connection::open("demerit.db")
}
