use chrono::{self, Local};
use rusqlite::{Connection, params};

fn main() {
    if let Err(e) = make_database() {
        eprintln!("Failed to initialize database: {e}");
        std::process::exit(1);
    }
}

struct PushupEntry {
    reps: u32,
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

fn add_pushups(conn: &Connection, reps: u32) -> Result<(), rusqlite::Error> {
    let now = Local::now();

    conn.execute(
        "INSERT INTO pushups (reps, timestamp, notes) VALUES (?1, ?2, ?3)",
        params![reps, now.to_rfc3339(), None::<String>],
    )?;

    Ok(())
}
