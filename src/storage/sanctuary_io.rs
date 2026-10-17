use std::error::Error;
use std::fs;

use chrono::Utc;

use rusqlite::{params, Connection};

use super::{ensure_parent, load_dual, save_sqlite, write_atomic, StoragePaths};
use crate::pet::Sanctuary;

// ── Sanctuary (multi-pet) API ──────────────────────────────────────────────

fn ensure_sanctuary_table(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sanctuary_saves (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            sanctuary_json TEXT NOT NULL,
            pet_count INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        );",
    )?;
    Ok(())
}

/// Persist the whole roster redundantly into both `mochigotchi_save.json`
/// (atomic write) and SQLite (`sanctuary_saves` + active pet mirror `pet_saves`).
pub fn save_sanctuary(sanctuary: &Sanctuary, paths: &StoragePaths) -> Result<(), Box<dyn Error>> {
    let active = sanctuary.active_pet();
    save_sqlite(active, &paths.sqlite_path)?;

    let json = serde_json::to_string_pretty(sanctuary)?;
    write_atomic(&paths.json_path, &json)?;

    ensure_parent(&paths.sqlite_path)?;
    let conn = Connection::open(&paths.sqlite_path)?;
    ensure_sanctuary_table(&conn)?;
    conn.execute(
        "INSERT INTO sanctuary_saves (id, sanctuary_json, pet_count, updated_at)
         VALUES (1, ?1, ?2, ?3)
         ON CONFLICT(id) DO UPDATE SET
            sanctuary_json = excluded.sanctuary_json,
            pet_count = excluded.pet_count,
            updated_at = excluded.updated_at",
        params![json, sanctuary.pets.len() as i64, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

/// Load the roster from SQLite or JSON, transparently upgrading a legacy
/// single-pet save.
pub fn load_sanctuary(paths: &StoragePaths) -> Result<Sanctuary, Box<dyn Error>> {
    if paths.sqlite_path.exists() {
        if let Ok(conn) = Connection::open(&paths.sqlite_path) {
            if ensure_sanctuary_table(&conn).is_ok() {
                let row: rusqlite::Result<String> = conn.query_row(
                    "SELECT sanctuary_json FROM sanctuary_saves WHERE id = 1",
                    [],
                    |row| row.get(0),
                );
                if let Ok(json) = row {
                    if let Ok(mut s) = serde_json::from_str::<Sanctuary>(&json) {
                        if !s.pets.is_empty() {
                            for pet in &mut s.pets {
                                pet.refresh_mood();
                            }
                            return Ok(s);
                        }
                    }
                }
            }
        }
    }

    if paths.json_path.exists() {
        if let Ok(raw) = fs::read_to_string(&paths.json_path) {
            if raw.contains("\"pets\"") {
                if let Ok(mut s) = serde_json::from_str::<Sanctuary>(&raw) {
                    if !s.pets.is_empty() {
                        for pet in &mut s.pets {
                            pet.refresh_mood();
                        }
                        return Ok(s);
                    }
                }
            }
        }
    }

    let pet = load_dual(paths)?;
    Ok(Sanctuary::new_with_pet(pet))
}
