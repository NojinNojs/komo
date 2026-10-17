//! Dual persistence for **Mochigotchi v1.0.0**.
//!
//! Every save is written twice: an atomic human-readable JSON file *and* a
//! SQLite database (`sanctuary_saves` + `pet_saves`). Both JSON and SQLite
//! store the full multi-pet [`Sanctuary`] roster for 100% redundancy, while
//! transparently upgrading legacy single-pet saves on load.

use std::env;
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

    /// Global user-level save directory (`~/.local/share/mochigotchi/`) so the
    /// CLI/TUI works identically from any directory in the terminal.
    pub fn default_paths() -> Self {
        let base_dir = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
            .map(|dir| dir.join("mochigotchi"));

        match base_dir {
            Some(dir) => Self::new(dir.join(DEFAULT_JSON_PATH), dir.join(DEFAULT_SQLITE_PATH)),
            None => Self::new(DEFAULT_JSON_PATH, DEFAULT_SQLITE_PATH),
        }
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

pub fn save_exists(paths: &StoragePaths) -> bool {
    has_existing_save(paths)
}

pub fn save_pet(pet: &Pet, paths: &StoragePaths) -> Result<(), Box<dyn Error>> {
    save_dual(pet, paths)
}

pub fn load_pet(paths: &StoragePaths) -> Result<Pet, Box<dyn Error>> {
    load_dual(paths)
}

fn ensure_parent(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

fn write_atomic(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    ensure_parent(path)?;
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, content)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

// ── Single-pet (legacy-compatible) API ─────────────────────────────────────

pub fn save_json(pet: &Pet, path: &Path) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(pet)?;
    write_atomic(path, &json)
}

pub fn load_json(path: &Path) -> Result<Pet, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    if let Ok(mut pet) = serde_json::from_str::<Pet>(&raw) {
        if !pet.name.is_empty() {
            // Also check if this JSON actually contains a full Sanctuary with pets
            if let Ok(mut s) = serde_json::from_str::<Sanctuary>(&raw) {
                if !s.pets.is_empty() && raw.contains("\"pets\"") {
                    let active = s.active_pet_mut();
                    active.refresh_mood();
                    return Ok(active.clone());
                }
            }
            pet.refresh_mood();
            return Ok(pet);
        }
    }
    let mut s: Sanctuary = serde_json::from_str(&raw)?;
    let active = s.active_pet_mut();
    active.refresh_mood();
    Ok(active.clone())
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
    fn sanctuary_roundtrip_keeps_every_pet_even_if_sqlite_is_removed() {
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

        // Remove SQLite and verify JSON alone still restores all 3 companions!
        let _ = fs::remove_file(&paths.sqlite_path);
        let from_json = load_sanctuary(&paths).unwrap();
        assert_eq!(from_json.pets.len(), 3);
        assert_eq!(from_json.pets[0].species, PetSpecies::IgnisDragon);
        assert_eq!(from_json.pets[1].species, PetSpecies::AzureWyrm);

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
