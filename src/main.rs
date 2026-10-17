use std::env;
use std::fs;
use std::path::PathBuf;

use chrono::Utc;
use mochigotchi::{
    load_sanctuary, render_animated_sprite, run_tui_with_report, save_exists, save_sanctuary,
    speech_bubble, thought_bubble, Pet, PetAction, PetSpecies, Sanctuary, ShellTheme, StoragePaths,
};

fn print_help() {
    println!(
        "\
🌿 KOMO v{} — Pocket Soul & Living Terminal Companion

USAGE:
    komo [OPTIONS]             (aliases: mochi / mochigotchi)

OPTIONS:
    -h, --help                Print this help information
    -V, --version             Print version information
    --name <NAME>             Set companion name when creating or adopting
    --call-me <TITLE>         Set how your companion addresses you (e.g., Aqsan, Boss, Keeper)
    --species <SPECIES>       cat | pup | bun | pengu | dragon | wyrm | fox | phoenix | uni | slime
    --theme <THEME>           pink | matcha | blue | lavender | gold
    --adopt                   Adopt a new companion into your Sanctuary (blocked if active died unburied)
    --switch <INDEX|NAME>     Switch active companion by 1-based index or name
    --json-path <PATH>        Custom path for JSON save file
    --db-path <PATH>          Custom path for SQLite save database
    --status                  Print active companion status, 4 senses, and 40x22 pixel sprite
    --roster                  List all companions and Memorial Hall records in your Sanctuary
    --vigil                   Perform one step (1..3) of the Burial Vigil Rite for a fallen companion
    --warm                    Warm & incubate your egg (+20% warmth)
    --feed                    Feed a meal (+1 Hungry Heart, +1g weight)
    --snack                   Feed a sweet snack (+1 Happy Heart, +2g weight)
    --play                    Play a game (+18 Happy, -2g weight)
    --pet                     Praise / cuddle your companion
    --talk                    Have a heart-to-heart chat with your companion
    --train                   Run a training session (+Discipline, -2g weight)
    --sleep                   Toggle sleep mode (curls onto the woven sleeping mat)
    --light                   Toggle the room light switch ON/OFF
    --scold                   Discipline (scold) when misbehaving
    --clean                   Flush mess, brush teeth & take a warm bath
    --heal                    Give herbal medicine, or use your 1-time Sacred Phoenix Tear
    --reset                   Delete existing save files and start fresh",
        env!("CARGO_PKG_VERSION")
    );
}

fn parse_species(val: &str) -> PetSpecies {
    match val.to_lowercase().as_str() {
        "pup" | "dog" | "boba" => PetSpecies::BobaPup,
        "bun" | "bunny" | "cloud" => PetSpecies::CloudBun,
        "pengu" | "penguin" | "star" => PetSpecies::StarPengu,
        "dragon" | "ignis" | "drake" => PetSpecies::IgnisDragon,
        "wyrm" | "azure" | "seadragon" => PetSpecies::AzureWyrm,
        "fox" | "kitsune" | "ember" => PetSpecies::EmberFox,
        "phoenix" | "sol" | "firebird" => PetSpecies::PhoenixChick,
        "uni" | "unicorn" | "crystal" => PetSpecies::CrystalUni,
        "slime" | "jelly" | "king" => PetSpecies::SlimeKing,
        _ => PetSpecies::MochiCat,
    }
}

fn parse_theme(val: &str) -> ShellTheme {
    match val.to_lowercase().as_str() {
        "matcha" | "green" => ShellTheme::MatchaGreen,
        "blue" | "sky" => ShellTheme::SkyBlue,
        "lavender" | "purple" => ShellTheme::LavenderDream,
        "gold" | "sunset" | "yellow" => ShellTheme::SunsetGold,
        _ => ShellTheme::StrawberryPink,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut custom_name: Option<String> = None;
    let mut custom_caretaker: Option<String> = None;
    let mut custom_species: Option<PetSpecies> = None;
    let mut custom_theme: Option<ShellTheme> = None;
    let mut adopt_flag = false;
    let mut vigil_flag = false;
    let mut switch_query: Option<String> = None;
    let mut json_path: Option<PathBuf> = None;
    let mut db_path: Option<PathBuf> = None;
    let mut cli_actions: Vec<PetAction> = Vec::new();
    let mut status_only = false;
    let mut roster_only = false;
    let mut reset = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-V" | "--version" => {
                println!("komo {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--name" => {
                i += 1;
                if i < args.len() {
                    custom_name = Some(args[i].clone());
                }
            }
            "--call-me" | "--caretaker" => {
                i += 1;
                if i < args.len() {
                    custom_caretaker = Some(args[i].clone());
                }
            }
            "--species" => {
                i += 1;
                if i < args.len() {
                    custom_species = Some(parse_species(&args[i]));
                }
            }
            "--theme" => {
                i += 1;
                if i < args.len() {
                    custom_theme = Some(parse_theme(&args[i]));
                }
            }
            "--adopt" => adopt_flag = true,
            "--vigil" => vigil_flag = true,
            "--switch" => {
                i += 1;
                if i < args.len() {
                    switch_query = Some(args[i].clone());
                }
            }
            "--json-path" => {
                i += 1;
                if i < args.len() {
                    json_path = Some(PathBuf::from(&args[i]));
                }
            }
            "--db-path" => {
                i += 1;
                if i < args.len() {
                    db_path = Some(PathBuf::from(&args[i]));
                }
            }
            "--status" => status_only = true,
            "--roster" => roster_only = true,
            "--warm" => cli_actions.push(PetAction::WarmEgg),
            "--feed" => cli_actions.push(PetAction::Feed),
            "--snack" => cli_actions.push(PetAction::FeedSnack),
            "--play" => cli_actions.push(PetAction::Play),
            "--pet" => cli_actions.push(PetAction::PetLove),
            "--talk" => cli_actions.push(PetAction::Talk),
            "--train" => cli_actions.push(PetAction::Train),
            "--sleep" => cli_actions.push(PetAction::ToggleSleep),
            "--light" => cli_actions.push(PetAction::ToggleLight),
            "--scold" => cli_actions.push(PetAction::DisciplineScold),
            "--clean" => cli_actions.push(PetAction::Clean),
            "--heal" => cli_actions.push(PetAction::Heal),
            "--reset" => reset = true,
            _ => {}
        }
        i += 1;
    }

    let defaults = StoragePaths::default_paths();
    let paths = StoragePaths::new(
        json_path.unwrap_or(defaults.json_path),
        db_path.unwrap_or(defaults.sqlite_path),
    );

    if reset {
        let _ = fs::remove_file(&paths.json_path);
        let _ = fs::remove_file(&paths.sqlite_path);
    }

    let first_time = !save_exists(&paths);
    let mut sanctuary = if first_time {
        let species = custom_species.unwrap_or(PetSpecies::MochiCat);
        let theme = custom_theme.unwrap_or(ShellTheme::StrawberryPink);
        let name = custom_name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        Sanctuary::new_with_pet(Pet::new_custom(&name, species, theme))
    } else {
        load_sanctuary(&paths)?
    };

    let (elapsed_offline, offline_report) = sanctuary.apply_offline_decay_all_with_report(Utc::now());
    if elapsed_offline >= 60 && (!cli_actions.is_empty() || status_only || roster_only) {
        sanctuary.active_pet_mut().log_action(format!(
            "⏱ Offline time applied: {} minutes",
            elapsed_offline / 60
        ));
    }

    if let Some(ref title) = custom_caretaker {
        sanctuary.set_caretaker_title_all(title);
    }

    if adopt_flag && !first_time {
        if let Err(err) = sanctuary.can_adopt_new() {
            eprintln!("{err}");
            return Ok(());
        }
        let species = custom_species.unwrap_or(PetSpecies::MochiCat);
        let theme = custom_theme.unwrap_or(ShellTheme::StrawberryPink);
        let name = custom_name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        let mut new_pet = Pet::new_custom(&name, species, theme);
        if let Some(ref title) = custom_caretaker {
            new_pet.set_caretaker_title(title);
        }
        sanctuary.add_pet(new_pet);
        println!(
            "🥚 Adopted {} ({}) into your Sanctuary! (Total companions: {})",
            sanctuary.active_pet().name,
            species.display_name(),
            sanctuary.pets.len()
        );
    } else {
        if let Some(ref name) = custom_name {
            sanctuary.active_pet_mut().rename(name);
        }
        if let Some(sp) = custom_species {
            sanctuary.active_pet_mut().species = sp;
        }
        if let Some(th) = custom_theme {
            sanctuary.active_pet_mut().theme = th;
        }
    }

    if let Some(ref query) = switch_query {
        if let Some(idx) = sanctuary.switch_by_query(query) {
            println!(
                "🔄 Switched active companion to #{}: {} ({})",
                idx + 1,
                sanctuary.active_pet().name,
                sanctuary.active_pet().species.display_name()
            );
        } else {
            eprintln!("⚠️ Could not find a companion matching '{query}'.");
        }
    }

    if !cli_actions.is_empty()
        || status_only
        || roster_only
        || adopt_flag
        || vigil_flag
        || custom_caretaker.is_some()
        || switch_query.is_some()
    {
        if vigil_flag {
            let msg = sanctuary.active_pet_mut().hold_vigil_step();
            let active_clone = sanctuary.active_pet().clone();
            sanctuary.record_memorial_if_buried(&active_clone);
            println!("{msg}");
        }
        for action in cli_actions {
            let feedback = sanctuary.active_pet_mut().perform_action(action);
            println!("{feedback}");
        }
        save_sanctuary(&sanctuary, &paths)?;

        if roster_only {
            println!(
                "🌿 KOMO Sanctuary (Lv.{} • Keeper: '{}' • Karma: {:+}) — {} companions:",
                sanctuary.sanctuary_level,
                sanctuary.caretaker_title,
                sanctuary.caretaker_karma,
                sanctuary.pets.len()
            );
            for (idx, p) in sanctuary.pets.iter().enumerate() {
                let marker = if idx == sanctuary.active_index {
                    "★"
                } else {
                    "•"
                };
                println!("  {} #{}: {}", marker, idx + 1, p.status_summary());
            }
            if !sanctuary.memorials.is_empty() {
                println!("\n🪦 Sanctuary Memorial Hall ({}):", sanctuary.memorials.len());
                for m in &sanctuary.memorials {
                    println!(
                        "  • {} ({} • {}) — Cause: {} | \"{}\"",
                        m.pet_name,
                        m.species.display_name(),
                        m.stage.badge(),
                        m.cause_of_death,
                        m.epitaph
                    );
                }
            }
            return Ok(());
        }

        let active = sanctuary.active_pet();
        println!("{}", active.status_summary());
        println!("{}", thought_bubble(active, 0));
        println!("{}", speech_bubble(active, 0));
        for line in render_animated_sprite(active, 0) {
            println!("  {line}");
        }
        return Ok(());
    }

    let start_onboarding =
        first_time && custom_name.is_none() && custom_species.is_none() && custom_theme.is_none();
    save_sanctuary(&sanctuary, &paths)?;
    run_tui_with_report(sanctuary, paths, start_onboarding, offline_report)?;
    Ok(())
}
