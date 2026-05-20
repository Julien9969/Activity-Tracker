use crate::shared::structs::{ActivityEntry, GroupedEntry};
use duckdb::{params, Connection, Result};
use once_cell::sync::OnceCell;
use std::{path::{Path, PathBuf}, sync::Mutex};
use crate::shared::paths;
use tracing::{debug, info, warn};

static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init_database(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    if DB.get().is_some() {
        return Ok(());
    }
    
    let db_path = paths::get_exe_dir(Some(app)).join("activity_records.duckdb");
    let conn = initialize_database(&db_path)?;

    info!("Database path: {}", db_path.display());

    let _ = DB.set(Mutex::new(conn));
    Ok(())
}

/// TODO save the DB
fn initialize_database(db_path: &Path) -> Result<Connection> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activityRecords (
            start_time TIMESTAMP,
            end_time TIMESTAMP,
            title TEXT,
            process_path TEXT,
            app_name TEXT,
            is_idle BOOLEAN,
            is_audio_playing BOOLEAN
        )",
        params![],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_start_time ON activityRecords(start_time)",
        params![],
    )?;
    info!("Database initialized successfully");
    Ok(conn)
}

pub fn insert_activity_entry(entry: &ActivityEntry) -> Result<()> {
    let conn = DB
        .get()
        .expect("Database not initialized")
        .lock()
        .unwrap();
    conn.execute(
        "INSERT INTO activityRecords 
        (start_time, end_time, title, process_path, app_name, is_idle, is_audio_playing) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            entry.start_time,
            entry.end_time,
            entry.title,
            entry.process_path.to_str().unwrap_or_default(),
            entry.app_name,
            entry.is_idle,
            entry.is_audio_playing,
        ],
    )?;
    Ok(())
}

pub fn get_latest_entry() -> Result<Option<ActivityEntry>> {
    let conn = DB
        .get()
        .expect("Database not initialized")
        .lock()
        .unwrap();

    let latest_entry = conn.query_row(
        "SELECT start_time, end_time, title, process_path, app_name, is_idle, is_audio_playing
        FROM activityRecords ORDER BY start_time DESC LIMIT 1",
        [],
        |row| {
            Ok(ActivityEntry {
                start_time: row.get(0)?,
                end_time: row.get(1)?,
                title: row.get(2)?,
                process_path: PathBuf::from(row.get::<_, String>(3)?),
                app_name: row.get(4)?,
                is_idle: row.get(5)?,
                is_audio_playing: row.get(6)?,
            })
        },
    );

    match latest_entry {
        Ok(entry) => Ok(Some(entry)),
        Err(duckdb::Error::QueryReturnedNoRows) => {
            debug!("Table is empty");
            Ok(None)
        }
        Err(e) => {
            warn!("No rows found or error: {}", e);
            Err(e)
        }
    }
}

pub fn update_latest_entry(entry: &ActivityEntry) -> () {
    let conn = DB
        .get()
        .expect("Database not initialized")
        .lock()
        .unwrap();
    let result = conn.execute(
        "UPDATE activityRecords SET 
        end_time = ?, is_idle = ?, is_audio_playing = ?
        WHERE start_time = (SELECT MAX(start_time) FROM activityRecords)",
        params![entry.end_time, entry.is_idle, entry.is_audio_playing,],
    );

    match result {
        Ok(rows_updated) => {
            if rows_updated == 0 {
                debug!("No rows updated, no entry found to update");
            } else {
                debug!("Updated latest entry successfully");
            }
        }
        Err(e) => {
            warn!("Error updating latest entry: {}", e);
        }
    }
}

pub fn get_grouped_entry(
    group_by: String,
    start_time: i64,
    end_time: i64,
) -> Result<Vec<GroupedEntry>, Box<dyn std::error::Error>> {
    let conn = DB
        .get()
        .expect("Database not initialized")
        .lock()
        .unwrap();
    let sql = format!(
        "SELECT {col} as name, SUM(epoch_ms(end_time) - epoch_ms(start_time)) as total_ms \
        FROM activityRecords \
        WHERE epoch_ms(start_time) >= ? AND epoch_ms(start_time) < ? AND (is_idle = 0 OR is_audio_playing = 1) \
        GROUP BY {col} \
        ORDER BY total_ms DESC",
        col = group_by
    );
    let mut stmt = conn.prepare(&sql)?;

    let mut entries = Vec::new();
    let mut rows = stmt.query(params![start_time, end_time])?;

    while let Some(row) = rows.next()? {
        entries.push(GroupedEntry {
            name: row.get(0)?,
            total_ms: row.get(1)?,
        });
    }

    Ok(entries)
}
