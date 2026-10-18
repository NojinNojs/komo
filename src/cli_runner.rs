//! Non-interactive CLI execution helpers for `komo`.

use mochigotchi::{
    default_share_output_path, render_animated_sprite, save_sanctuary, save_share_card_png,
    speech_bubble, thought_bubble, Pet, Sanctuary, StoragePaths, CURRENT_APP_VERSION,
    WHATS_NEW_FEATURES,
};

use crate::cli_args::CliArgs;

/// Execute non-interactive CLI subcommands (`--status`, `--roster`, `--share`, actions, etc.).
/// Returns `Ok(true)` if CLI mode handled the invocation and the process should exit.
pub(crate) fn run_cli_if_requested(
    sanctuary: &mut Sanctuary,
    paths: &StoragePaths,
    cli: &CliArgs,
) -> Result<bool, Box<dyn std::error::Error>> {
    let is_cli_mode = !cli.cli_actions.is_empty()
        || cli.status_only
        || cli.roster_only
        || cli.changelog_only
        || cli.update_guide
        || cli.adopt_flag
        || cli.vigil_flag
        || cli.custom_caretaker.is_some()
        || cli.switch_query.is_some()
        || cli.share_ratio.is_some();

    if !is_cli_mode {
        return Ok(false);
    }

    if cli.update_guide {
        print_update_guide();
        return Ok(true);
    }

    if cli.changelog_only {
        print_changelog();
        return Ok(true);
    }

    if cli.vigil_flag {
        let msg = sanctuary.active_pet_mut().hold_vigil_step();
        let active_clone = sanctuary.active_pet().clone();
        sanctuary.record_memorial_if_buried(&active_clone);
        println!("{msg}");
    }

    for &action in &cli.cli_actions {
        let feedback = sanctuary.active_pet_mut().perform_action(action);
        println!("{feedback}");
    }

    save_sanctuary(sanctuary, paths)?;

    if let Some(ratio) = cli.share_ratio {
        let out = cli
            .share_out
            .clone()
            .unwrap_or_else(|| default_share_output_path(sanctuary.active_pet(), ratio));
        save_share_card_png(sanctuary.active_pet(), ratio, &out)?;
        println!(
            "📸 Generated {} promo card -> {}",
            ratio.label(),
            out.display()
        );
        return Ok(true);
    }

    if cli.roster_only {
        print_roster(sanctuary);
        return Ok(true);
    }

    print_active_status(sanctuary.active_pet());
    Ok(true)
}

pub(crate) fn print_roster(sanctuary: &Sanctuary) {
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
}

pub(crate) fn print_active_status(active: &Pet) {
    println!("{}", active.status_summary());
    println!("{}", thought_bubble(active, 0));
    println!("{}", speech_bubble(active, 0));
    for line in render_animated_sprite(active, 0) {
        println!("  {line}");
    }
}

pub(crate) fn print_changelog() {
    println!("🎉 What's New in KOMO v{}:", CURRENT_APP_VERSION);
    for item in WHATS_NEW_FEATURES {
        println!("  • {item}");
    }
}

pub(crate) fn print_update_guide() {
    println!(
        "\
🔄 How to Update KOMO (v{} -> Latest) — Your Companion Save Data Stays Safe!

  • Linux / NixOS (installed in ~/.local/bin):
      cd ~/komo && git pull && cargo build --release && cp target/release/mochigotchi ~/.local/bin/komo

  • macOS / Linux (installed via Cargo):
      cargo install --git https://github.com/NojinNojs/komo.git --force

  • Windows (PowerShell):
      cargo install --git https://github.com/NojinNojs/komo.git --force

✨ After updating, launch `komo` to see the What's New screen right inside your habitat!",
        CURRENT_APP_VERSION
    );
}
