use rusqlite::{Connection, Result};
use std::fs;

pub fn init_db() -> Result<Connection> {
    // Just initialize db
    if let Err(e) = fs::create_dir_all("pub") {
        panic!("No se pudo crear la carpeta 'pub': {}", e);
    }

    let route_db: String = format!("pub/db");

    let conn = Connection::open(&route_db)?;
    Ok(conn)
}