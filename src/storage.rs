//! Dual persistence for **Mochigotchi**.
//!
//! Every save is written twice: a human-readable JSON file *and* a SQLite
//! database. The active companion is always mirrored into the classic
//! single-pet tables so older tooling (and `load_dual`) keeps working, while
//! the full [`Sanctuary`] roster lives in its own table.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::{params, Connection};

use crate::pet::{Pet, Sanctuary};

pub const DEFAULT_JSON_PATH: &str = "mochigotchi_save.json";
pub const DEFAULT_SQLITE_PATH: &str = "mochigotchi_save.db";

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

fn ensure_parent(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

// ── Single-pet (legacy-compatible) API ─────────────────────────────────────

pub fn save_json(pet: &Pet, path: &Path) -> Result<(), Box<dyn Error>> {
    ensure_parent(path)?;
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
    ensure_parent(path)?;
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
            pet.last_updated.to_rfc3339()
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

/// Persist the whole roster: active pet into the classic dual stores, and the
/// complete collection into `sanctuary_saves`.
pub fn save_sanctuary(sanctuary: &Sanctuary, paths: &StoragePaths) -> Result<(), Box<dyn Error>> {
    let active = sanctuary.active_pet();
    save_dual(active, paths)?;

    ensure_parent(&paths.sqlite_path)?;
    let conn = Connection::open(&paths.sqlite_path)?;
    ensure_sanctuary_table(&conn)?;

    let json = serde_json::to_string_pretty(sanctuary)?;
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

/// Load the roster, transparently upgrading a legacy single-pet save.
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

    let pet = load_dual(paths)?;
    Ok(Sanctuary::new_with_pet(pet))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::{PetSpecies, ShellTheme};

    fn temp_paths(tag: &str) -> StoragePaths {
        let dir = std::env::temp_dir().join(format!(
            "mochigotchi_test_{}_{}",
            tag,
            std::process::id()
        ));
        StoragePaths::new(dir.join("save.json"), dir.join("save.db"))
    }

    #[test]
    fn legacy_single_pet_roundtrip() {
        let paths = temp_paths("legacy");
        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());

        let pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::SkyBlue);
        save_dual(&pet, &paths).unwrap();
        assert!(has_existing_save(&paths));

        let json_pet = load_json(&paths.json_path).unwrap();
        assert_eq!(json_pet.name, "Boba");
        let sqlite_pet = load_sqlite(&paths.sqlite_path).unwrap();
        assert_eq!(sqlite_pet.species, PetSpecies::BobaPup);
        assert_eq!(load_dual(&paths).unwrap().theme, ShellTheme::SkyBlue);

        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
    }

    #[test]
    fn sanctuary_roundtrip_keeps_every_pet() {
        let paths = temp_paths("sanctuary");
        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());

        let mut s = Sanctuary::new_with_pet(Pet::new_custom(
            "Ignis",
            PetSpecies::IgnisDragon,
            ShellTheme::SunsetGold,
        ));
        s.add_pet(Pet::new_custom(
            "Ao",
            PetSpecies::AzureWyrm,
            ShellTheme::SkyBlue,
        ));
        s.add_pet(Pet::new("Mochi"));
        save_sanctuary(&s, &paths).unwrap();

        let loaded = load_sanctuary(&paths).unwrap();
        assert_eq!(loaded.pets.len(), 3);
        assert_eq!(loaded.active_pet().name, "Mochi");
        assert_eq!(loaded.pets[0].species, PetSpecies::IgnisDragon);
        assert_eq!(loaded.pets[1].species, PetSpecies::AzureWyrm);

        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
    }

    #[test]
    fn loading_legacy_save_upgrades_into_sanctuary() {
        let paths = temp_paths("upgrade");
        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());

        save_dual(&Pet::new("Pebble"), &paths).unwrap();
        let s = load_sanctuary(&paths).unwrap();
        assert_eq!(s.pets.len(), 1);
        assert_eq!(s.active_pet().name, "Pebble");

        let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
    }
}
