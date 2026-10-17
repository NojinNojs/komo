use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::{params, Connection};

use crate::pet::Pet;

pub const DEFAULT_JSON_PATH: &str = "save.json";
pub const DEFAULT_SQLITE_PATH: &str = "rustagotchi.db";

#[derive(Debug, Clone)]
pub struct StoragePaths {
    pub json_path: PathBuf,
    pub sqlite_path: PathBuf,
}

impl StoragePaths {
    pub fn new(json_path: impl Into<PathBuf>, sqlite_path: impl Into<PathBuf>) -> Self {
        Self {
            json_path: json_path.into(),
            sqlite_path: sqlite_path.into(),
        }
    }

    pub fn default_paths() -> Self {
        Self::new(DEFAULT_JSON_PATH, DEFAULT_SQLITE_PATH)
    }
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn save_json(pet: &Pet, path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    ensure_parent_dir(path)?;
    let content = serde_json::to_string_pretty(pet).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn load_json(path: impl AsRef<Path>) -> Result<Pet, String> {
    let content = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn save_sqlite(pet: &Pet, path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    ensure_parent_dir(path)?;
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pet_saves (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            name TEXT NOT NULL,
            stage TEXT NOT NULL,
            xp INTEGER NOT NULL,
            payload_json TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    let json_payload = serde_json::to_string(pet).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO pet_saves (id, name, stage, xp, payload_json, updated_at)
         VALUES (1, ?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            stage = excluded.stage,
            xp = excluded.xp,
            payload_json = excluded.payload_json,
            updated_at = excluded.updated_at",
        params![
            pet.name,
            pet.stage.name(),
            pet.xp,
            json_payload,
            pet.last_updated.to_rfc3339()
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_sqlite(path: impl AsRef<Path>) -> Result<Pet, String> {
    let path = path.as_ref();
    if !path.exists() {
        return Err("SQLite database file does not exist".to_string());
    }
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    let payload_json: String = conn
        .query_row(
            "SELECT payload_json FROM pet_saves WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&payload_json).map_err(|e| e.to_string())
}

pub fn save_dual(pet: &Pet, paths: &StoragePaths) -> Result<(), String> {
    save_json(pet, &paths.json_path)?;
    save_sqlite(pet, &paths.sqlite_path)?;
    Ok(())
}

pub fn load_dual(paths: &StoragePaths) -> Result<Pet, String> {
    if let Ok(pet) = load_json(&paths.json_path) {
        return Ok(pet);
    }
    let pet = load_sqlite(&paths.sqlite_path)?;
    let _ = save_json(&pet, &paths.json_path);
    Ok(pet)
}

pub fn load_or_create(paths: &StoragePaths, default_name: &str) -> Pet {
    match load_dual(paths) {
        Ok(mut pet) => {
            pet.apply_offline_decay(Utc::now());
            let _ = save_dual(&pet, paths);
            pet
        }
        Err(_) => {
            let pet = Pet::new(default_name);
            let _ = save_dual(&pet, paths);
            pet
        }
    }
}
