//! Command-line argument parsing for the `komo` binary.

use std::path::PathBuf;

use mochigotchi::{PetAction, PetSpecies, ShellTheme};

use crate::cli_help::{parse_species, parse_theme, print_help};

pub struct CliArgs {
    pub custom_name: Option<String>,
    pub custom_caretaker: Option<String>,
    pub custom_species: Option<PetSpecies>,
    pub custom_theme: Option<ShellTheme>,
    pub adopt_flag: bool,
    pub vigil_flag: bool,
    pub switch_query: Option<String>,
    pub json_path: Option<PathBuf>,
    pub db_path: Option<PathBuf>,
    pub cli_actions: Vec<PetAction>,
    pub status_only: bool,
    pub roster_only: bool,
    pub reset: bool,
}

impl CliArgs {
    /// Parse argv. Returns `None` when a help/version flag already printed output.
    pub fn parse(args: &[String]) -> Option<Self> {
        let mut cli = CliArgs {
            custom_name: None,
            custom_caretaker: None,
            custom_species: None,
            custom_theme: None,
            adopt_flag: false,
            vigil_flag: false,
            switch_query: None,
            json_path: None,
            db_path: None,
            cli_actions: Vec::new(),
            status_only: false,
            roster_only: false,
            reset: false,
        };

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    print_help();
                    return None;
                }
                "-V" | "--version" => {
                    println!("komo {}", env!("CARGO_PKG_VERSION"));
                    return None;
                }
                "--adopt" => cli.adopt_flag = true,
                "--vigil" => cli.vigil_flag = true,
                "--status" => cli.status_only = true,
                "--roster" => cli.roster_only = true,
                "--reset" => cli.reset = true,
                "--name" => cli.custom_name = value(args, &mut i),
                "--call-me" | "--caretaker" => cli.custom_caretaker = value(args, &mut i),
                "--species" => cli.custom_species = value(args, &mut i).map(|v| parse_species(&v)),
                "--theme" => cli.custom_theme = value(args, &mut i).map(|v| parse_theme(&v)),
                "--switch" => cli.switch_query = value(args, &mut i),
                "--json-path" => cli.json_path = value(args, &mut i).map(PathBuf::from),
                "--db-path" => cli.db_path = value(args, &mut i).map(PathBuf::from),
                other => {
                    if let Some(action) = action_for(other) {
                        cli.cli_actions.push(action);
                    }
                }
            }
            i += 1;
        }
        Some(cli)
    }
}

/// Consume the value following a flag, advancing `i` past it.
fn value(args: &[String], i: &mut usize) -> Option<String> {
    *i += 1;
    args.get(*i).cloned()
}

fn action_for(flag: &str) -> Option<PetAction> {
    Some(match flag {
        "--warm" => PetAction::WarmEgg,
        "--feed" => PetAction::Feed,
        "--snack" => PetAction::FeedSnack,
        "--play" => PetAction::Play,
        "--pet" => PetAction::PetLove,
        "--talk" => PetAction::Talk,
        "--train" => PetAction::Train,
        "--sleep" => PetAction::ToggleSleep,
        "--light" => PetAction::ToggleLight,
        "--scold" => PetAction::DisciplineScold,
        "--clean" => PetAction::Clean,
        "--heal" => PetAction::Heal,
        _ => return None,
    })
}
