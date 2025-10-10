use chrono::{self, Local};
use rusqlite::Connection;

fn main() {
    make_database();
}

struct PushupEntry {
    reps: u16,
    timestamp: chrono::DateTime<Local>,
}

fn make_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("./pushups.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pushups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            reps INTEGER NOT NULL,
            timestamp DATETIME NOT NULL,
            notes TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_pushups_timestamp ON pushups(timestamp)",
        [],
    )?;

    Ok(())
}
