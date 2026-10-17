use chrono::Utc;
use mochigotchi::{
    has_existing_save, load_or_create, run_tui, save_dual, PetSpecies, ShellTheme, StoragePaths,
};
use std::env;
use std::process;

fn print_help() {
    println!(
        "mochigotchi — Your Cozy Pocket Pet in the Terminal\n\
         \n\
         USAGE:\n\
           mochigotchi [OPTIONS]\n\
         \n\
         OPTIONS:\n\
           --status                 Show current pet status and exit\n\
           --warm                   Warm / incubate your egg and save\n\
           --feed                   Feed your pet a delicious snack and save\n\
           --play                   Play with your pet and save\n\
           --pet                    Give your pet gentle headpats and save\n\
           --train                  Do a fun study/trick training session and save\n\
           --sleep                  Toggle sleep state and save\n\
           --clean                  Clean your pet's room and save\n\
           --heal                   Give medicine to your pet and save\n\
           --name <NAME>            Rename or set initial pet name\n\
           --species <SPECIES>      Set species (cat, pup, bun, pengu)\n\
           --theme <THEME>          Set shell theme (pink, matcha, blue, lavender, gold)\n\
           --json-path <PATH>       Custom JSON save file path (default: save.json)\n\
           --db-path <PATH>         Custom SQLite DB file path (default: mochigotchi.db)\n\
           -h, --help               Show this help message"
    );
}

fn parse_species(val: &str) -> Option<PetSpecies> {
    match val.trim().to_ascii_lowercase().as_str() {
        "cat" | "mochicat" | "neko" => Some(PetSpecies::MochiCat),
        "pup" | "dog" | "bobapup" => Some(PetSpecies::BobaPup),
        "bun" | "bunny" | "rabbit" | "cloudbun" => Some(PetSpecies::CloudBun),
        "pengu" | "penguin" | "starpengu" => Some(PetSpecies::StarPengu),
        _ => None,
    }
}

fn parse_theme(val: &str) -> Option<ShellTheme> {
    match val.trim().to_ascii_lowercase().as_str() {
        "pink" | "strawberry" | "strawberrypink" => Some(ShellTheme::StrawberryPink),
        "matcha" | "green" | "matchagreen" => Some(ShellTheme::MatchaGreen),
        "blue" | "sky" | "skyblue" => Some(ShellTheme::SkyBlue),
        "lavender" | "purple" | "lavenderdream" => Some(ShellTheme::LavenderDream),
        "gold" | "sunset" | "sunsetgold" => Some(ShellTheme::SunsetGold),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut json_path = mochigotchi::DEFAULT_JSON_PATH.to_string();
    let mut sqlite_path = mochigotchi::DEFAULT_SQLITE_PATH.to_string();
    let mut pet_name = "Mochi".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json-path" if i + 1 < args.len() => {
                json_path = args[i + 1].clone();
                i += 2;
            }
            "--db-path" if i + 1 < args.len() => {
                sqlite_path = args[i + 1].clone();
                i += 2;
            }
            "--name" if i + 1 < args.len() => {
                pet_name = args[i + 1].clone();
                i += 2;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                i += 1;
            }
        }
    }

    let paths = StoragePaths::new(json_path, sqlite_path);
    let save_existed = has_existing_save(&paths);
    let mut pet = if save_existed || !args.is_empty() {
        let mut p = load_or_create(&paths, &pet_name);
        let _elapsed = p.apply_offline_decay(Utc::now());
        p
    } else {
        mochigotchi::Pet::new(&pet_name)
    };

    let mut headless_action = false;
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--json-path" | "--db-path" => {
                idx += 2;
            }
            "--name" => {
                if idx + 1 < args.len() {
                    pet.rename(&args[idx + 1]);
                }
                idx += 2;
            }
            "--species" => {
                if idx + 1 < args.len() {
                    if let Some(sp) = parse_species(&args[idx + 1]) {
                        pet.species = sp;
                    }
                }
                idx += 2;
            }
            "--theme" => {
                if idx + 1 < args.len() {
                    if let Some(th) = parse_theme(&args[idx + 1]) {
                        pet.theme = th;
                    }
                }
                idx += 2;
            }
            "--status" => {
                headless_action = true;
                println!("{}", pet.status_summary());
                idx += 1;
            }
            "--warm" => {
                headless_action = true;
                println!("{}", pet.warm_egg());
                idx += 1;
            }
            "--feed" => {
                headless_action = true;
                println!("{}", pet.feed());
                idx += 1;
            }
            "--play" => {
                headless_action = true;
                println!("{}", pet.play());
                idx += 1;
            }
            "--pet" => {
                headless_action = true;
                println!("{}", pet.pet_love());
                idx += 1;
            }
            "--train" => {
                headless_action = true;
                println!("{}", pet.train());
                idx += 1;
            }
            "--sleep" => {
                headless_action = true;
                println!("{}", pet.toggle_sleep());
                idx += 1;
            }
            "--clean" => {
                headless_action = true;
                println!("{}", pet.clean());
                idx += 1;
            }
            "--heal" => {
                headless_action = true;
                println!("{}", pet.heal());
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }

    if headless_action {
        if let Err(e) = save_dual(&pet, &paths) {
            eprintln!("Failed to save pet state: {}", e);
            process::exit(1);
        }
        return;
    }

    match run_tui(pet, paths.clone()) {
        Ok(final_pet) => {
            if let Err(e) = save_dual(&final_pet, &paths) {
                eprintln!("Failed to save pet state on exit: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("TUI error: {}", e);
            process::exit(1);
        }
    }
}
