use chrono::{self, Local, TimeZone};
use clap::Parser;
use rusqlite::{Connection, params};

#[derive(Parser)]
#[command(name = "pushups")]
#[command(about = "Track your pushups", long_about = None)]
enum Cli {
    /// add number of pushups for a set
    Add {
        reps: u32,
    },
    Today,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    make_database()?;
    let conn = Connection::open("./pushups.db")?;

    match cli {
        Cli::Add { reps } => {
            add_pushups(&conn, reps)?;
            println!("Added {reps} pushups");
        }
        Cli::Today => {
            let todays_pushups = todays_pushups(&conn)?;
            println!("We've done {todays_pushups} pushups today");
        }
    }

    Ok(())
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

fn days_pushups(conn: &Connection, time: chrono::DateTime<Local>) -> Result<u32, rusqlite::Error> {
    let start = time.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let start_local = Local.from_local_datetime(&start).unwrap();

    let end = start + chrono::Duration::days(1);
    let end_local = Local.from_local_datetime(&end).unwrap();

    let total: Option<i64> = conn.query_row(
        "SELECT SUM(reps) FROM pushups WHERE timestamp >= ?1 AND timestamp < ?2",
        params![start_local.to_rfc3339(), end_local.to_rfc3339()],
        |row| row.get(0),
    )?;

    Ok(total.unwrap_or(0) as u32)
}

fn todays_pushups(conn: &Connection) -> Result<u32, rusqlite::Error> {
    days_pushups(conn, Local::now())
}
