use super::*;
use crate::pet::{PetSpecies, ShellTheme};

fn temp_paths(tag: &str) -> StoragePaths {
    let dir = std::env::temp_dir().join(format!("mochigotchi_test_{}_{}", tag, std::process::id()));
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
