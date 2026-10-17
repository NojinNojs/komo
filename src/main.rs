//! Command-line entry point for **Mochigotchi**.
//!
//! Running it with no arguments drops you straight into the LCD habitat (with
//! a Welcome Back report if you were away for 2+ minutes). Flags let you script
//! care actions, inspect a companion, or adopt a specific species without the
//! wizard.

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use chrono::Utc;
use mochigotchi::pet::{Pet, PetAction, PetSpecies, Sanctuary, ShellTheme};
use mochigotchi::sprites::{render_animated_sprite, speech_bubble};
use mochigotchi::storage::{has_existing_save, load_sanctuary, save_sanctuary, StoragePaths};

fn print_help() {
    println!("🍡 Mochigotchi v0.4.0 — Sanctuary, TrueColor Sprites & Real-Time Life Edition");
    println!();
    println!("USAGE:");
    println!("  mochigotchi [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  -h, --help            Show this help message");
    println!("  --species <SPECIES>   Adopt a specific species:");
    println!("                          Cozy:      cat, pup, bun, pengu");
    println!("                          Mythical:  dragon, wyrm, fox, phoenix, unicorn, slime");
    println!("  --theme <THEME>       Shell colorway: pink, matcha, blue, lavender, gold");
    println!("  --name <NAME>         Name your companion");
    println!("  --status              Print the active companion, then exit");
    println!("  --roster              Print every companion in the Sanctuary, then exit");
    println!("  --warm                Warm the egg (+20% incubation, 3m cooldown)");
    println!("  --feed                Feed the active companion");
    println!("  --play                Play together");
    println!("  --pet                 Give headpats");
    println!("  --train               Run a training session (4m cooldown)");
    println!("  --sleep               Toggle sleep");
    println!("  --clean               Tidy the habitat");
    println!("  --heal                Give medicine");
    println!("  --reset               Delete existing saves and start over");
    println!();
    println!("KEYBOARD (in-game):");
    println!("  [W] warm/cuddle  [F] quick feed  [M] pantry menu  [P] play  [E] train  [S] sleep");
    println!("  [C] clean  [H] heal  [G] arcade  [L] Sanctuary roster  [N] adopt  [?] help");
    println!("  [[ / ]] switch companion  [Tab] shell color  [R] rename  [Q] quit");
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
    show_status: bool,
    show_roster: bool,
    species: Option<PetSpecies>,
    theme: Option<ShellTheme>,
    name: Option<String>,
    actions: Vec<PetAction>,
    paths: StoragePaths,
}

fn parse_args(raw: &[String]) -> Result<CliArgs, String> {
    let mut args = CliArgs {
        reset: false,
        show_status: false,
        show_roster: false,
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
            "--reset" => args.reset = true,
            "--status" => args.show_status = true,
            "--roster" => args.show_roster = true,
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

    if had_save {
        if let Some(species) = cli.species {
            sanctuary.active_pet_mut().species = species;
        }
        if let Some(theme) = cli.theme {
            sanctuary.active_pet_mut().theme = theme;
        }
        if let Some(name) = cli.name.clone() {
            sanctuary.active_pet_mut().rename(&name);
        }
    }

    let (_elapsed, offline_report) = sanctuary.apply_offline_decay_all_with_report(Utc::now());

    if !cli.actions.is_empty() {
        for action in &cli.actions {
            let msg = sanctuary.active_pet_mut().apply_action(*action);
            println!("{msg}");
        }
        save_sanctuary(&sanctuary, &cli.paths)?;
    } else if !had_save && customised {
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
                "{marker} {}. {} — {} · {} · Lv.{} · hunger {}% · incubation {}%",
                idx + 1,
                pet.name,
                pet.species.display_name(),
                pet.stage.badge(),
                pet.level,
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

    let start_onboarding = !had_save && !customised && cli.actions.is_empty();
    mochigotchi::tui::run_tui_with_report(sanctuary, cli.paths, start_onboarding, offline_report)
}
