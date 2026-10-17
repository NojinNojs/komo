use chrono::Utc;
use rustagotchi::{
    install_post_commit_hook, load_or_create, run_tui, save_dual, sync_latest_commits, StoragePaths,
};
use std::env;
use std::process;

fn print_help() {
    println!(
        "rustagotchi - Terminal Virtual Pet for Developers\n\
         \n\
         USAGE:\n\
           rustagotchi [OPTIONS]\n\
         \n\
         OPTIONS:\n\
           --status                 Show current pet status and exit\n\
           --feed                   Feed your pet and save\n\
           --play                   Play with your pet and save\n\
           --sleep                  Toggle sleep state and save\n\
           --clean                  Clean your pet and save\n\
           --code                   Run a coding session and save\n\
           --heal                   Heal your pet and save\n\
           --git-commit <MSG>       Reward pet for a git commit (deduplicated)\n\
           --sync-git               Scan recent git log commits and reward unseen ones\n\
           --install-hook           Install .git/hooks/post-commit in current repository\n\
           --name <NAME>            Rename or set initial pet name\n\
           --json-path <PATH>       Custom JSON save file path (default: save.json)\n\
           --db-path <PATH>         Custom SQLite DB file path (default: rustagotchi.db)\n\
           -h, --help               Show this help message"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut json_path = rustagotchi::DEFAULT_JSON_PATH.to_string();
    let mut sqlite_path = rustagotchi::DEFAULT_SQLITE_PATH.to_string();
    let mut pet_name = "Ferris".to_string();

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
    let mut pet = load_or_create(&paths, &pet_name);
    let _elapsed = pet.apply_offline_decay(Utc::now());

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
            "--status" => {
                headless_action = true;
                println!("{}", pet.status_summary());
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
            "--code" => {
                headless_action = true;
                println!("{}", pet.code_session());
                idx += 1;
            }
            "--heal" => {
                headless_action = true;
                println!("{}", pet.heal());
                idx += 1;
            }
            "--git-commit" => {
                headless_action = true;
                if idx + 1 < args.len() {
                    let commit_msg = &args[idx + 1];
                    if pet.reward_git_commit(commit_msg) {
                        println!("Rewarded Rustagotchi for commit: {}", commit_msg);
                    } else {
                        println!("Commit already rewarded or empty: {}", commit_msg);
                    }
                    idx += 2;
                } else {
                    eprintln!("Error: --git-commit requires an argument");
                    process::exit(1);
                }
            }
            "--sync-git" => {
                headless_action = true;
                match sync_latest_commits(&mut pet, ".", 10) {
                    Ok(count) => println!("Synced {} new git commit(s)!", count),
                    Err(e) => eprintln!("Git sync warning: {}", e),
                }
                idx += 1;
            }
            "--install-hook" => {
                headless_action = true;
                match install_post_commit_hook(".", None) {
                    Ok(hook_path) => println!("Installed post-commit hook at {}", hook_path.display()),
                    Err(e) => {
                        eprintln!("Failed to install git hook: {}", e);
                        process::exit(1);
                    }
                }
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
