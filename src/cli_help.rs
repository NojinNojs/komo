//! Help text and value parsers for the `komo` CLI.

use mochigotchi::{PetSpecies, ShellTheme};

pub(crate) fn print_help() {
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

pub(crate) fn parse_species(val: &str) -> PetSpecies {
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

pub(crate) fn parse_theme(val: &str) -> ShellTheme {
    match val.to_lowercase().as_str() {
        "matcha" | "green" => ShellTheme::MatchaGreen,
        "blue" | "sky" => ShellTheme::SkyBlue,
        "lavender" | "purple" => ShellTheme::LavenderDream,
        "gold" | "sunset" | "yellow" => ShellTheme::SunsetGold,
        _ => ShellTheme::StrawberryPink,
    }
}
