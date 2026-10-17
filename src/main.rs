use std::env;
use std::fs;

use chrono::Utc;
use mochigotchi::{
    load_sanctuary, render_animated_sprite, run_tui_with_report, save_exists, save_sanctuary,
    speech_bubble, thought_bubble, Pet, PetSpecies, Sanctuary, ShellTheme, StoragePaths,
};

mod cli_args;
mod cli_help;

use cli_args::CliArgs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(cli) = CliArgs::parse(&args) else {
        return Ok(());
    };
    let CliArgs {
        custom_name,
        custom_caretaker,
        custom_species,
        custom_theme,
        adopt_flag,
        vigil_flag,
        switch_query,
        json_path,
        db_path,
        cli_actions,
        status_only,
        roster_only,
        reset,
    } = cli;

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

    let (elapsed_offline, offline_report) =
        sanctuary.apply_offline_decay_all_with_report(Utc::now());
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
                println!(
                    "\n🪦 Sanctuary Memorial Hall ({}):",
                    sanctuary.memorials.len()
                );
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
