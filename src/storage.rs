use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::{params, Connection};

use crate::pet::Pet;

pub const DEFAULT_JSON_PATH: &str = "save.json";
pub const DEFAULT_SQLITE_PATH: &str = "mochigotchi.db";

#[derive(Debug, Clone)]
pub struct StoragePaths {
    pub json_path: PathBuf,
    pub sqlite_path: PathBuf,
}

impl StoragePaths {
    pub fn new<P1: Into<PathBuf>, P2: Into<PathBuf>>(json_path: P1, sqlite_path: P2) -> Self {
        Self {
            json_path: json_path.into(),
            sqlite_path: sqlite_path.into(),
        }
    }

    pub fn default_paths() -> Self {
        Self::new(DEFAULT_JSON_PATH, DEFAULT_SQLITE_PATH)
    }
}

impl Default for StoragePaths {
    fn default() -> Self {
        Self::default_paths()
    }
}

pub fn has_existing_save(paths: &StoragePaths) -> bool {
    paths.json_path.exists() || paths.sqlite_path.exists()
}

pub fn save_json(pet: &Pet, path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let json = serde_json::to_string_pretty(pet)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_json(path: &Path) -> Result<Pet, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let mut pet: Pet = serde_json::from_str(&raw)?;
    pet.refresh_mood();
    Ok(pet)
}

pub fn save_sqlite(pet: &Pet, path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS pet_saves (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            name TEXT NOT NULL,
            stage TEXT NOT NULL,
            xp INTEGER NOT NULL,
            payload_json TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
    )?;

    let payload_json = serde_json::to_string(pet)?;
    let updated_at = pet.last_updated.to_rfc3339();

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
            payload_json,
            updated_at
        ],
    )?;

    Ok(())
}

pub fn load_sqlite(path: &Path) -> Result<Pet, Box<dyn Error>> {
    if !path.exists() {
        return Err(format!("SQLite save file not found: {}", path.display()).into());
    }
    let conn = Connection::open(path)?;
    let payload_json: String = conn.query_row(
        "SELECT payload_json FROM pet_saves WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    let mut pet: Pet = serde_json::from_str(&payload_json)?;
    pet.refresh_mood();
    Ok(pet)
}

pub fn save_dual(pet: &Pet, paths: &StoragePaths) -> Result<(), Box<dyn Error>> {
    save_json(pet, &paths.json_path)?;
    save_sqlite(pet, &paths.sqlite_path)?;
    Ok(())
}

pub fn load_dual(paths: &StoragePaths) -> Result<Pet, Box<dyn Error>> {
    if paths.json_path.exists() {
        if let Ok(pet) = load_json(&paths.json_path) {
            return Ok(pet);
        }
    }
    load_sqlite(&paths.sqlite_path)
}

pub fn load_or_create(paths: &StoragePaths, default_name: &str) -> Pet {
    if let Ok(mut pet) = load_dual(paths) {
        let _ = pet.apply_offline_decay(Utc::now());
        let _ = save_dual(&pet, paths);
        pet
    } else {
        let pet = Pet::new(default_name);
        let _ = save_dual(&pet, paths);
        pet
    }
}
