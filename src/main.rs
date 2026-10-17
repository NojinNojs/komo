//! Command-line entry point for **Mochigotchi v1.0.0**.
//!
//! Running `mochigotchi` (or `mochi` / `tamagotchi`) with no arguments from any
//! directory in your terminal opens the interactive Retro Handheld LCD habitat
//! (stored under `~/.local/share/mochigotchi/`).

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use chrono::Utc;
use mochigotchi::pet::{Pet, PetAction, PetSpecies, Sanctuary, ShellTheme};
use mochigotchi::sprites::{render_animated_sprite, speech_bubble};
use mochigotchi::storage::{has_existing_save, load_sanctuary, save_sanctuary, StoragePaths};

fn print_version() {
    println!("mochigotchi {}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    println!(
        "🍡 Mochigotchi v{} — Retro Handheld V-Pet & 24×16 TrueColor Pixel Art Edition",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("USAGE:");
    println!("  mochigotchi [OPTIONS]   (also available as `mochi` or `tamagotchi`)");
    println!();
    println!("OPTIONS:");
    println!("  -h, --help              Show this help message");
    println!("  -V, --version           Print version information");
    println!("  --species <SPECIES>     Select species:");
    println!("                            Cozy:      cat, pup, bun, pengu");
    println!("                            Mythical:  dragon, wyrm, fox, phoenix, unicorn, slime");
    println!("  --theme <THEME>         Shell colorway: pink, matcha, blue, lavender, gold");
    println!("  --name <NAME>           Name your companion");
    println!("  --adopt                 Adopt a new companion into the existing Sanctuary");
    println!("  --switch <INDEX|NAME>   Switch active companion by 1-based number or name");
    println!("  --status                Print the active companion (hearts, weight, rank, pixel art)");
    println!("  --roster                Print every companion in the Sanctuary");
    println!("  --warm                  Warm the egg (+20% incubation, 3m cooldown)");
    println!("  --feed                  Feed a Meal (+1 Hungry Heart, +1g weight)");
    println!("  --snack                 Feed a Sweet Snack (+1 Happy Heart, +2g weight; 3 = Toothache!)");
    println!("  --light                 Toggle the room light ON / OFF");
    println!("  --scold                 Discipline (Scold) during a mischievous Attention Call");
    println!("  --play                  Play Left/Right game (-2g weight, +Happy)");
    println!("  --pet                   Praise / give headpats");
    println!("  --train                 Run a training session (-2g weight, +Discipline, 4m cooldown)");
    println!("  --sleep                 Toggle sleep");
    println!("  --clean                 Flush mess, brush teeth & bubble bath");
    println!("  --heal                  Give Medicine (cures sickness & toothache)");
    println!("  --reset                 Delete existing saves and start over");
    println!();
    println!("IN-GAME CONTROLS:");
    println!("  Button A (←/→) select LCD Bezel Icon • Button B (Enter) confirm • Button C (Esc) cancel");
    println!("  [F] Meal  [M] Pantry  [K] Light  [P] Play  [D] Scold  [T] Praise  [E] Train  [S] Sleep");
    println!("  [C] Bath  [H] Meds  [I] 4-Page Status Meter  [G] Arcade  [L] Sanctuary  [N] Adopt  [?] Keys");
}

fn parse_species(value: &str) -> Option<PetSpecies> {
    match value.trim().to_lowercase().as_str() {
        "cat" | "mochi" | "neko" | "mochicat" => Some(PetSpecies::MochiCat),
        "pup" | "boba" | "dog" | "bobapup" => Some(PetSpecies::BobaPup),
        "bun" | "cloud" | "bunny" | "rabbit" | "cloudbun" => Some(PetSpecies::CloudBun),
        "pengu" | "star" | "penguin" | "starpengu" => Some(PetSpecies::StarPengu),
        "dragon" | "ignis" | "drake" | "ignisdragon" => Some(PetSpecies::IgnisDragon),
        "wyrm" | "azure" | "azurewyrm" => Some(PetSpecies::AzureWyrm),
        "fox" | "kitsune" | "ember" | "emberfox" => Some(PetSpecies::EmberFox),
        "phoenix" | "sol" | "phoenixchick" => Some(PetSpecies::PhoenixChick),
        "unicorn" | "astra" | "uni" | "crystaluni" => Some(PetSpecies::CrystalUni),
        "slime" | "king" | "jelly" | "slimeking" => Some(PetSpecies::SlimeKing),
        _ => None,
    }
}

fn parse_theme(value: &str) -> Option<ShellTheme> {
    match value.trim().to_lowercase().as_str() {
        "pink" | "strawberry" | "strawberrypink" => Some(ShellTheme::StrawberryPink),
        "matcha" | "green" | "matchagreen" => Some(ShellTheme::MatchaGreen),
        "blue" | "sky" | "skyblue" => Some(ShellTheme::SkyBlue),
        "lavender" | "purple" | "lavenderdream" => Some(ShellTheme::LavenderDream),
        "gold" | "sunset" | "sunsetgold" => Some(ShellTheme::SunsetGold),
        _ => None,
    }
}

struct CliArgs {
    reset: bool,
    adopt: bool,
    show_status: bool,
    show_roster: bool,
    switch_target: Option<String>,
    species: Option<PetSpecies>,
    theme: Option<ShellTheme>,
    name: Option<String>,
    actions: Vec<PetAction>,
    paths: StoragePaths,
}

fn parse_args(raw: &[String]) -> Result<CliArgs, String> {
    let mut args = CliArgs {
        reset: false,
        adopt: false,
        show_status: false,
        show_roster: false,
        switch_target: None,
        species: None,
        theme: None,
        name: None,
        actions: Vec::new(),
        paths: StoragePaths::default_paths(),
    };

    let mut idx = 0;
    while idx < raw.len() {
        match raw[idx].as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                print_version();
                std::process::exit(0);
            }
            "--reset" => args.reset = true,
            "--adopt" => args.adopt = true,
            "--status" => args.show_status = true,
            "--roster" => args.show_roster = true,
            "--switch" => {
                let value = raw.get(idx + 1).ok_or("--switch needs an index or name")?;
                args.switch_target = Some(value.clone());
                idx += 1;
            }
            "--species" => {
                let value = raw.get(idx + 1).ok_or("--species needs a value")?;
                args.species =
                    Some(parse_species(value).ok_or_else(|| format!("unknown species: {value}"))?);
                idx += 1;
            }
            "--theme" => {
                let value = raw.get(idx + 1).ok_or("--theme needs a value")?;
                args.theme =
                    Some(parse_theme(value).ok_or_else(|| format!("unknown theme: {value}"))?);
                idx += 1;
            }
            "--name" => {
                let value = raw.get(idx + 1).ok_or("--name needs a value")?;
                if !value.trim().is_empty() {
                    args.name = Some(value.trim().to_string());
                }
                idx += 1;
            }
            "--json-path" => {
                let value = raw.get(idx + 1).ok_or("--json-path needs a value")?;
                args.paths.json_path = value.into();
                idx += 1;
            }
            "--db-path" => {
                let value = raw.get(idx + 1).ok_or("--db-path needs a value")?;
                args.paths.sqlite_path = value.into();
                idx += 1;
            }
            "--warm" => args.actions.push(PetAction::WarmEgg),
            "--feed" => args.actions.push(PetAction::Feed),
            "--snack" => args.actions.push(PetAction::FeedSnack),
            "--light" => args.actions.push(PetAction::ToggleLight),
            "--scold" => args.actions.push(PetAction::DisciplineScold),
            "--play" => args.actions.push(PetAction::Play),
            "--pet" => args.actions.push(PetAction::PetLove),
            "--train" => args.actions.push(PetAction::Train),
            "--sleep" => args.actions.push(PetAction::ToggleSleep),
            "--clean" => args.actions.push(PetAction::Clean),
            "--heal" => args.actions.push(PetAction::Heal),
            other => return Err(format!("unknown option: {other}")),
        }
        idx += 1;
    }

    Ok(args)
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw: Vec<String> = env::args().skip(1).collect();
    let cli = match parse_args(&raw) {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("mochigotchi: {err}");
            print_help();
            std::process::exit(2);
        }
    };

    if cli.reset {
        for path in [&cli.paths.json_path, &cli.paths.sqlite_path] {
            if Path::new(path).exists() {
                fs::remove_file(path)?;
            }
        }
    }

    let had_save = has_existing_save(&cli.paths);
    let customised = cli.species.is_some() || cli.theme.is_some() || cli.name.is_some();

    let mut sanctuary = if had_save {
        load_sanctuary(&cli.paths).unwrap_or_else(|_| Sanctuary::new_with_pet(Pet::default()))
    } else {
        let species = cli.species.unwrap_or_default();
        let theme = cli.theme.unwrap_or_default();
        let name = cli
            .name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        Sanctuary::new_with_pet(Pet::new_custom(&name, species, theme))
    };

    let (elapsed, offline_report) = sanctuary.apply_offline_decay_all_with_report(Utc::now());
    let mut dirty = elapsed > 0;

    if had_save && cli.adopt {
        let species = cli.species.unwrap_or_default();
        let theme = cli.theme.unwrap_or_else(|| sanctuary.active_pet().theme);
        let name = cli
            .name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        let new_pet = Pet::new_custom(&name, species, theme);
        let idx = sanctuary.add_pet(new_pet);
        println!(
            "🥚 Adopted {} ({}) into Sanctuary slot #{}!",
            sanctuary.pets[idx].name,
            sanctuary.pets[idx].species.display_name(),
            idx + 1
        );
        dirty = true;
    } else if had_save {
        if let Some(species) = cli.species {
            sanctuary.active_pet_mut().species = species;
            dirty = true;
        }
        if let Some(theme) = cli.theme {
            sanctuary.active_pet_mut().theme = theme;
            dirty = true;
        }
        if let Some(name) = cli.name.clone() {
            sanctuary.active_pet_mut().rename(&name);
            dirty = true;
        }
    } else if customised || cli.adopt {
        dirty = true;
    }

    if let Some(ref target) = cli.switch_target {
        match sanctuary.switch_by_query(target) {
            Some(idx) => {
                println!(
                    "★ Switched active companion to #{}: {} ({})",
                    idx + 1,
                    sanctuary.pets[idx].name,
                    sanctuary.pets[idx].species.display_name()
                );
                dirty = true;
            }
            None => {
                eprintln!("mochigotchi: companion '{target}' not found in Sanctuary");
                std::process::exit(2);
            }
        }
    }

    if !cli.actions.is_empty() {
        for action in &cli.actions {
            let msg = sanctuary.active_pet_mut().apply_action(*action);
            println!("{msg}");
        }
        dirty = true;
    }

    if dirty {
        save_sanctuary(&sanctuary, &cli.paths)?;
    }

    if cli.show_roster {
        println!(
            "🏰 Sanctuary — {} companions ({} hatched) · Sanctuary Lv.{}",
            sanctuary.pets.len(),
            sanctuary.hatched_count(),
            sanctuary.sanctuary_level
        );
        for (idx, pet) in sanctuary.pets.iter().enumerate() {
            let marker = if idx == sanctuary.active_index {
                "★"
            } else {
                " "
            };
            println!(
                "{marker} {}. {} — {} · {} · Lv.{} · {}g · {} · hunger {}% · incubation {}%",
                idx + 1,
                pet.name,
                pet.species.display_name(),
                pet.stage.badge(),
                pet.level,
                pet.weight_grams,
                pet.care_rank().badge(),
                pet.hunger,
                pet.incubation
            );
        }
        return Ok(());
    }

    if cli.show_status {
        let pet = sanctuary.active_pet().clone();
        println!("{}", pet.status_summary());
        println!("{}", pet.next_evolution_requirements());
        println!("{}", speech_bubble(&pet, 0));
        println!();
        for line in render_animated_sprite(&pet, 0) {
            println!("  {line}");
        }
        return Ok(());
    }

    let start_onboarding = !had_save && !customised && !cli.adopt && cli.actions.is_empty();
    mochigotchi::tui::run_tui_with_report(sanctuary, cli.paths, start_onboarding, offline_report)
}
