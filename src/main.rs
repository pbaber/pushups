use chrono::{self, Datelike, Local, NaiveDateTime, TimeZone};
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
    Week,
    Month,
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
        Cli::Week => {
            let weeks_pushups = weeks_pushups(&conn)?;
            println!("We've done {weeks_pushups} pushups this week");
        }
        Cli::Month => {
            let months_pushups = months_pushups(&conn)?;
            println!("We've done {months_pushups} pushups this month");
        }
    }

    Ok(())
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

fn pushups_in_timeperiod(
    conn: &Connection,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<u32, rusqlite::Error> {
    let start_local = Local.from_local_datetime(&start).unwrap();
    let end_local = Local.from_local_datetime(&end).unwrap();

    let total: Option<i64> = conn.query_row(
        "SELECT SUM(reps) FROM pushups WHERE timestamp >= ?1 AND timestamp < ?2",
        params![start_local.to_rfc3339(), end_local.to_rfc3339()],
        |row| row.get(0),
    )?;

    Ok(total.unwrap_or(0) as u32)
}

fn todays_pushups(conn: &Connection) -> Result<u32, rusqlite::Error> {
    let start = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
    let end = start + chrono::Duration::days(1);
    pushups_in_timeperiod(conn, start, end)
}

fn weeks_pushups(conn: &Connection) -> Result<u32, rusqlite::Error> {
    let now = Local::now();
    let current_date = now.date_naive();

    // Get days since Monday (0 = Monday, 1 = Tuesday, ...)
    let days_since_monday = current_date.weekday().num_days_from_monday();

    let monday = current_date - chrono::Duration::days(days_since_monday as i64);
    let start = monday.and_hms_opt(0, 0, 0).unwrap();

    let end = start + chrono::Duration::days(7);

    pushups_in_timeperiod(conn, start, end)
}

fn months_pushups(conn: &Connection) -> Result<u32, rusqlite::Error> {
    let now = Local::now();
    let current_date = now.date_naive();

    let first_day_of_this_month = current_date.with_day(1).unwrap();
    let start = first_day_of_this_month.and_hms_opt(0, 0, 0).unwrap();

    let next_month = if current_date.month() == 12 {
        current_date
            .with_year(current_date.year() + 1)
            .unwrap()
            .with_month(1)
            .unwrap()
    } else {
        current_date.with_month(current_date.month() + 1).unwrap()
    };

    let first_day_next_month = next_month.with_day(1).unwrap();
    let end = first_day_next_month.and_hms_opt(0, 0, 0).unwrap();

    pushups_in_timeperiod(conn, start, end)
}
