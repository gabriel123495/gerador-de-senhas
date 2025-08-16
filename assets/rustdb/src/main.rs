use rusqlite::prelude::*;

fn main() -> Result<()> {
    let connection = Connection::open("sqlite.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY NOT NULL,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
        )",
        [],
    )?;
    Ok(())
}