use std::env;
use std::fs;

use chrono::Utc;
use mochigotchi::{
    load_sanctuary, run_tui_with_report, save_exists, save_sanctuary, Pet, PetSpecies, Sanctuary,
    ShellTheme, StoragePaths,
};

mod cli_args;
mod cli_help;
mod cli_runner;

use cli_args::CliArgs;
use cli_runner::run_cli_if_requested;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(cli) = CliArgs::parse(&args) else {
        return Ok(());
    };

    let defaults = StoragePaths::default_paths();
    let paths = StoragePaths::new(
        cli.json_path.clone().unwrap_or(defaults.json_path),
        cli.db_path.clone().unwrap_or(defaults.sqlite_path),
    );

    if cli.reset {
        let _ = fs::remove_file(&paths.json_path);
        let _ = fs::remove_file(&paths.sqlite_path);
    }

    let first_time = !save_exists(&paths);
    let mut sanctuary = if first_time {
        let species = cli.custom_species.unwrap_or(PetSpecies::MochiCat);
        let theme = cli.custom_theme.unwrap_or(ShellTheme::StrawberryPink);
        let name = cli
            .custom_name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        Sanctuary::new_with_pet(Pet::new_custom(&name, species, theme))
    } else {
        load_sanctuary(&paths)?
    };

    let (elapsed_offline, offline_report) =
        sanctuary.apply_offline_decay_all_with_report(Utc::now());
    if elapsed_offline >= 60
        && (!cli.cli_actions.is_empty()
            || cli.status_only
            || cli.roster_only
            || cli.share_ratio.is_some())
    {
        sanctuary.active_pet_mut().log_action(format!(
            "⏱ Offline time applied: {} minutes",
            elapsed_offline / 60
        ));
    }

    if let Some(title) = &cli.custom_caretaker {
        sanctuary.set_caretaker_title_all(title);
    }

    if cli.adopt_flag && !first_time {
        if let Err(err) = sanctuary.can_adopt_new() {
            eprintln!("{err}");
            return Ok(());
        }
        let species = cli.custom_species.unwrap_or(PetSpecies::MochiCat);
        let theme = cli.custom_theme.unwrap_or(ShellTheme::StrawberryPink);
        let name = cli
            .custom_name
            .clone()
            .unwrap_or_else(|| species.default_name().to_string());
        let mut new_pet = Pet::new_custom(&name, species, theme);
        if let Some(title) = &cli.custom_caretaker {
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
        if let Some(name) = &cli.custom_name {
            sanctuary.active_pet_mut().rename(name);
        }
        if let Some(sp) = cli.custom_species {
            sanctuary.active_pet_mut().species = sp;
        }
        if let Some(th) = cli.custom_theme {
            sanctuary.active_pet_mut().theme = th;
        }
    }

    if let Some(query) = &cli.switch_query {
        if let Some(idx) = sanctuary.switch_by_query(query) {
            println!(
                "🔄 Switched active companion to #{}: {} ({})",
                idx + 1,
                sanctuary.active_pet().name,
                sanctuary.active_pet().species.display_name()
            );
        } else {
            eprintln!("⚠ Could not find a companion matching '{query}'.");
        }
    }

    if run_cli_if_requested(&mut sanctuary, &paths, &cli)? {
        return Ok(());
    }

    let start_onboarding = first_time
        && cli.custom_name.is_none()
        && cli.custom_species.is_none()
        && cli.custom_theme.is_none();
    save_sanctuary(&sanctuary, &paths)?;
    run_tui_with_report(sanctuary, paths, start_onboarding, offline_report)?;
    Ok(())
}
