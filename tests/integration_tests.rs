//! End-to-end integration tests for **Mochigotchi**: lifecycle rules,
//! realistic multi-hour/day cooldowns, offline welcome-back reports, food
//! pantry, TrueColor sprites, the multi-pet Sanctuary, persistence, and TUI.

use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::minigame::{ItemKind, MiniGame};
use mochigotchi::pet::{
    EvolutionStage, FoodChoice, Pet, PetAction, PetMood, PetSpecies, Sanctuary, ShellTheme,
    PET_COOLDOWN_SECS, TRAIN_COOLDOWN_SECS, WARM_COOLDOWN_SECS,
};
use mochigotchi::sprites::{
    get_mood_particles, get_sprite_frames, render_animated_sprite, render_colored_preview,
    render_colored_sprite, render_species_preview, speech_bubble, theme_color,
};
use mochigotchi::storage::{
    has_existing_save, load_dual, load_sanctuary, save_dual, save_sanctuary, StoragePaths,
};
use mochigotchi::tui::{draw_ui, App, AppScreen, LayoutTier, OnboardingStep};
use ratatui::{backend::TestBackend, Terminal};
use tempfile::TempDir;

fn temp_paths() -> (TempDir, StoragePaths) {
    let dir = tempfile::tempdir().expect("tempdir");
    let paths = StoragePaths::new(dir.path().join("save.json"), dir.path().join("save.db"));
    (dir, paths)
}

// ── 1. Species catalogue & TrueColor sprites ──────────────────────────────

#[test]
fn all_ten_species_are_described_and_animated() {
    let all = PetSpecies::all();
    assert_eq!(all.len(), 10);

    let cozy = all.iter().filter(|s| s.family() == "Cozy Classics").count();
    let mythical = all.iter().filter(|s| s.is_mythical()).count();
    assert_eq!(cozy, 4);
    assert_eq!(mythical, 6);

    for species in all {
        assert!(!species.display_name().is_empty());
        assert!(!species.rarity().contains("  "));
        assert!(!species.element_badge().is_empty());
        assert!(!species.description().is_empty());
        assert!(!species.default_name().is_empty());
        assert!(!species.favorite_snack().is_empty());

        let preview = render_species_preview(*species);
        assert!(!preview.is_empty(), "{species:?} preview");

        let colored_preview = render_colored_preview(*species, EvolutionStage::Adult, 0);
        assert!(!colored_preview.is_empty());

        for stage in EvolutionStage::all() {
            for mood in PetMood::all() {
                let frames = get_sprite_frames(*species, *stage, *mood);
                assert_eq!(frames.len(), 2, "{species:?} {stage:?} {mood:?}");
                assert_ne!(frames[0], frames[1], "{species:?} {stage:?} {mood:?}");
            }
        }
    }

    assert!(all.iter().any(|s| s.display_name().contains("Ignis Drake")));
    assert!(all.iter().any(|s| s.display_name().contains("Azure Wyrm")));
}

// ── 2. Egg stage: guarding, realistic incubation, hatching ─────────────────

#[test]
fn unhatched_egg_refuses_every_care_action() {
    let mut pet = Pet::new("Mochi");
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.incubation, 0);
    assert_eq!(pet.coins, 0);

    for action in [
        PetAction::Feed,
        PetAction::Play,
        PetAction::PetLove,
        PetAction::Train,
        PetAction::ToggleSleep,
        PetAction::Clean,
        PetAction::Heal,
    ] {
        let mut clone = pet.clone();
        let message = clone.apply_action(action);
        assert!(message.contains("unhatched"), "{action:?} -> {message}");
        assert_eq!(clone.xp, 0, "{action:?} must not grant XP to an egg");
        assert_eq!(clone.coins, 0);
        assert_eq!(clone.stage, EvolutionStage::Egg);
        assert_eq!(clone.mood, PetMood::Idle);
    }

    let first = pet.warm_egg();
    assert!(first.contains("Incubation at 20%"), "{first}");
    assert_eq!(pet.incubation, 20);

    let blocked = pet.warm_egg();
    assert!(blocked.contains("absorbing your warmth"), "{blocked}");
    assert_eq!(pet.incubation, 20, "cooldown must block incubation gain");
    let remaining = pet.warm_cooldown_remaining(Utc::now());
    assert!(remaining > 0);
    assert!(remaining <= WARM_COOLDOWN_SECS as u64);
}

#[test]
fn five_manual_warmings_hatch_the_egg_into_a_baby() {
    let mut pet = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);

    for expected in [20u8, 40, 60, 80, 100] {
        pet.clear_cooldowns();
        pet.warm_egg();
        assert_eq!(pet.incubation, expected);
    }

    assert_eq!(pet.stage, EvolutionStage::Baby);
    assert!(pet.xp >= EvolutionStage::Baby.min_xp());
    assert_eq!(pet.level, 2);
    assert!(pet.coins >= 15);
    assert!(pet.action_log.iter().any(|l| l.contains("hatched")));
}

#[test]
fn passive_incubation_never_hatches_the_egg_by_itself() {
    let mut pet = Pet::new("Mochi");
    for _ in 0..200 {
        pet.tick_decay();
    }
    assert_eq!(pet.stage, EvolutionStage::Egg, "time alone must not hatch");
    assert_eq!(pet.incubation, 99, "passive warmth tops out at 99%");
    assert_eq!(pet.hunger, 65, "an egg never starves");

    let mut pet = Pet::new("Mochi");
    let now = Utc::now() + Duration::hours(3);
    let (elapsed, report) = pet.apply_offline_decay_with_report(now);
    assert!(elapsed >= 10_000);
    assert!(report.is_some());
    assert_eq!(pet.incubation, 99);
    assert_eq!(pet.hunger, 65);

    pet.clear_cooldowns();
    let msg = pet.warm_egg();
    assert!(msg.contains("CRACK"), "{msg}");
    assert_eq!(pet.stage, EvolutionStage::Baby);
}

// ── 3. Anti-spam care guards & Pantry choices ─────────────────────────────

#[test]
fn care_actions_cannot_be_spammed_for_free_xp() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;

    pet.hunger = 98;
    let before = pet.xp;
    let msg = pet.feed();
    assert!(msg.contains("completely full"), "{msg}");
    assert_eq!(pet.xp, before);

    pet.happiness = 100;
    pet.clear_cooldowns();
    let msg = pet.pet_love();
    assert!(msg.contains("maxed out"), "{msg}");
    assert_eq!(pet.xp, before);

    pet.happiness = 50;
    pet.clear_cooldowns();
    let first = pet.pet_love();
    assert!(!first.contains("still basking"), "{first}");
    let second = pet.pet_love();
    assert!(second.contains("still basking"), "{second}");
    let pet_rest = pet.pet_cooldown_remaining(Utc::now());
    assert!(pet_rest > 0);
    assert!(pet_rest <= PET_COOLDOWN_SECS as u64);

    pet.energy = 100;
    pet.hunger = 100;
    pet.clear_cooldowns();
    let trained = pet.train();
    assert!(!trained.contains("catching their breath"), "{trained}");
    let too_soon = pet.train();
    assert!(too_soon.contains("catching their breath"), "{too_soon}");
    let train_rest = pet.train_cooldown_remaining(Utc::now());
    assert!(train_rest > 0);
    assert!(train_rest <= TRAIN_COOLDOWN_SECS as u64);

    pet.cleanliness = 100;
    pet.poop_count = 0;
    let xp_before = pet.xp;
    let msg = pet.clean();
    assert!(msg.contains("sparkling clean"), "{msg}");
    assert_eq!(pet.xp, xp_before);

    pet.is_sick = false;
    pet.health = 100;
    let msg = pet.heal();
    assert!(msg.contains("tip-top health"), "{msg}");
    assert_eq!(pet.xp, xp_before);

    pet.clear_cooldowns();
    pet.energy = 5;
    assert!(pet.play().contains("too tired"));
    assert!(pet.train().contains("lacks energy"));

    pet.energy = 100;
    pet.hunger = 10;
    assert!(pet.play().contains("too hungry"));
    assert!(pet.train().contains("too hungry to focus"));

    pet.hunger = 80;
    pet.energy = 90;
    pet.is_sick = true;
    pet.health = 30;
    assert!(pet.play().contains("sick"));
    assert!(pet.train().contains("unwell"));
    let healed = pet.heal();
    assert!(healed.contains("recovered"), "{healed}");
    assert!(!pet.is_sick);

    let mut sleepy = Pet::new("Mochi");
    sleepy.stage = EvolutionStage::Baby;
    let msg = sleepy.toggle_sleep();
    assert!(msg.contains("Goodnight"), "{msg}");
    assert!(sleepy.feed().contains("fast asleep"));
    assert!(sleepy.play().contains("sleeping"));
    assert!(sleepy.train().contains("asleep"));
}

#[test]
fn pantry_meals_and_poop_cleaning_cycle() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.hunger = 40;

    // Expensive meal without coins is refused.
    pet.coins = 0;
    let refused = pet.feed_choice(FoodChoice::StarlightDelight);
    assert!(refused.contains("Not enough coins"), "{refused}");

    // Free favorite snack works.
    pet.clear_cooldowns();
    pet.feed();
    assert_eq!(pet.poop_count, 0, "one meal is not enough for a mess");

    // Second meal with coins buys Bento and triggers poop.
    pet.coins = 20;
    pet.hunger = 40;
    pet.clear_cooldowns();
    let bento = pet.feed_choice(FoodChoice::HeartyBento);
    assert!(bento.contains("Hearty Bento"), "{bento}");
    assert_eq!(pet.coins, 12);
    assert_eq!(pet.poop_count, 1);
    assert!(pet.cleanliness < 100);

    let cleaned = pet.clean();
    assert!(cleaned.contains("bubble bath"), "{cleaned}");
    assert_eq!(pet.poop_count, 0);
    assert_eq!(pet.cleanliness, 100);
}

// ── 4. Evolution needs real multi-hour age, bond and XP ───────────────────

#[test]
fn evolution_requires_xp_bond_and_time_together() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.xp = EvolutionStage::Teen.min_xp();
    pet.bond = EvolutionStage::Teen.min_bond();
    pet.age_seconds = 60; // 1 minute is too young for Teen (needs 6 hours)

    assert!(!pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Baby);

    pet.age_seconds = EvolutionStage::Teen.min_age_seconds();
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Teen);

    pet.xp = EvolutionStage::Adult.min_xp();
    pet.age_seconds = EvolutionStage::Adult.min_age_seconds();
    pet.bond = 0;
    assert!(!pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Teen);

    pet.bond = 100;
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Adult);
    assert!(pet.xp_progress_ratio() >= 0.0 && pet.xp_progress_ratio() <= 1.0);

    pet.xp = EvolutionStage::CyberElder.min_xp();
    pet.age_seconds = EvolutionStage::CyberElder.min_age_seconds();
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::CyberElder);
    assert!(!pet.check_evolution());
    assert!(pet.next_evolution_requirements().contains("Max evolution"));

    let maxed = EvolutionStage::CyberElder.max_stat_bonus();
    assert!(maxed > EvolutionStage::Egg.max_stat_bonus());
    assert_eq!(EvolutionStage::from_xp(0), EvolutionStage::Egg);
    assert_eq!(EvolutionStage::from_xp(5_000), EvolutionStage::CyberElder);
}

#[test]
fn offline_decay_generates_welcome_back_report_and_exploration_gifts() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.hunger = 80;
    let before = (pet.hunger, pet.happiness, pet.energy, pet.coins);
    let now = Utc::now() + Duration::minutes(45);
    let (elapsed, report) = pet.apply_offline_decay_with_report(now);

    assert_eq!(elapsed, 2_700);
    assert!(pet.hunger < before.0);
    assert!(pet.happiness < before.1);
    assert!(pet.energy < before.2);
    assert!(pet.coins > before.3, "pet should find coins while exploring");
    let rep = report.expect("should generate welcome back report");
    assert_eq!(rep.pet_name, "Mochi");
    assert!(!rep.events.is_empty());

    // A sleeping pet recovers energy and wakes up after a long rest.
    let mut sleeper = Pet::new("Mochi");
    sleeper.stage = EvolutionStage::Baby;
    sleeper.energy = 20;
    sleeper.toggle_sleep();
    sleeper.apply_offline_decay(Utc::now() + Duration::minutes(60));
    assert!(sleeper.energy > 20);
}

// ── 5. Multi-pet Sanctuary ────────────────────────────────────────────────

#[test]
fn sanctuary_holds_many_companions_and_cycles_between_them() {
    let mut sanctuary = Sanctuary::new_with_pet(Pet::new_custom(
        "Ignis",
        PetSpecies::IgnisDragon,
        ShellTheme::SunsetGold,
    ));
    assert_eq!(sanctuary.pets.len(), 1);
    assert_eq!(sanctuary.active_index, 0);

    let idx_a = sanctuary.add_pet(Pet::new_custom(
        "Ao",
        PetSpecies::AzureWyrm,
        ShellTheme::SkyBlue,
    ));
    let idx_b = sanctuary.add_pet(Pet::new("Puni"));
    assert_eq!(idx_a, 1);
    assert_eq!(idx_b, 2);
    assert_eq!(sanctuary.active_pet().name, "Puni");
    assert_eq!(sanctuary.hatched_count(), 0);

    sanctuary.next_pet();
    assert_eq!(sanctuary.active_pet().name, "Ignis");
    sanctuary.prev_pet();
    assert_eq!(sanctuary.active_pet().name, "Puni");

    assert!(sanctuary.switch_pet(1));
    assert_eq!(sanctuary.active_pet().species, PetSpecies::AzureWyrm);
    assert!(!sanctuary.switch_pet(99));

    sanctuary.active_pet_mut().rename("Ao-chan");
    let snapshot = sanctuary.active_pet().clone();
    sanctuary.sync_active(&snapshot);
    assert_eq!(sanctuary.pets[1].name, "Ao-chan");

    let before: Vec<u64> = sanctuary.pets.iter().map(|p| p.age_seconds).collect();
    sanctuary.tick_decay_all();
    for (pet, old_age) in sanctuary.pets.iter().zip(before) {
        assert!(pet.age_seconds > old_age);
    }

    sanctuary.apply_offline_decay_all(Utc::now() + Duration::minutes(5));
    assert!(sanctuary.sanctuary_level >= 1);
}

#[test]
fn sanctuary_survives_a_save_and_load_cycle() {
    let (_dir, paths) = temp_paths();
    assert!(!has_existing_save(&paths));

    let mut sanctuary = Sanctuary::new_with_pet(Pet::new_custom(
        "Ignis",
        PetSpecies::IgnisDragon,
        ShellTheme::SunsetGold,
    ));
    sanctuary.add_pet(Pet::new_custom(
        "Ao",
        PetSpecies::AzureWyrm,
        ShellTheme::SkyBlue,
    ));
    sanctuary.add_pet(Pet::new_custom(
        "Kitsune",
        PetSpecies::EmberFox,
        ShellTheme::LavenderDream,
    ));
    sanctuary.pets[0].stage = EvolutionStage::Adult;
    sanctuary.pets[0].xp = 1_500;
    sanctuary.active_index = 1;

    save_sanctuary(&sanctuary, &paths).expect("save sanctuary");
    assert!(has_existing_save(&paths));

    let loaded = load_sanctuary(&paths).expect("load sanctuary");
    assert_eq!(loaded.pets.len(), 3);
    assert_eq!(loaded.active_index, 1);
    assert_eq!(loaded.pets[0].stage, EvolutionStage::Adult);
    assert_eq!(loaded.pets[0].xp, 1_500);
    assert_eq!(loaded.pets[2].species, PetSpecies::EmberFox);

    let mirrored = load_dual(&paths).expect("legacy mirror");
    assert_eq!(mirrored.name, loaded.active_pet().name);

    let (_dir2, legacy_paths) = temp_paths();
    save_dual(&Pet::new("Pebble"), &legacy_paths).expect("legacy save");
    let upgraded = load_sanctuary(&legacy_paths).expect("upgrade legacy save");
    assert_eq!(upgraded.pets.len(), 1);
    assert_eq!(upgraded.active_pet().name, "Pebble");
}

// ── 6. Mini-game ──────────────────────────────────────────────────────────

#[test]
fn snack_catcher_arcade_awards_rewards() {
    let mut game = MiniGame::new(30, 10, 40);
    assert!(!game.is_finished());

    game.spawn_item(5, ItemKind::Snack);
    game.spawn_item(6, ItemKind::Star);
    game.spawn_item(7, ItemKind::Hazard);
    assert_eq!(game.items.len(), 3);

    game.move_right();
    game.move_left();
    for _ in 0..50 {
        game.step();
        if game.is_finished() {
            break;
        }
    }
    assert!(game.is_finished());
    assert_eq!(game.ticks_remaining, 0);
}

// ── 7. TUI end-to-end ─────────────────────────────────────────────────────

#[test]
fn adoption_wizard_lets_you_build_a_collection() {
    let (_dir, paths) = temp_paths();
    let mut app = App::new_onboarding(paths);
    assert_eq!(app.screen, AppScreen::Onboarding);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectSpecies);

    for _ in 0..4 {
        app.handle_key(KeyCode::Down);
    }
    app.handle_key(KeyCode::Char('v'));
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);
    app.handle_key(KeyCode::Tab);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
    for _ in 0..8 {
        app.handle_key(KeyCode::Backspace);
    }
    for c in "Sparky".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.screen, AppScreen::Habitat);
    assert_eq!(app.pet.species, PetSpecies::IgnisDragon);
    assert_eq!(app.pet.name, "Sparky");
    assert_eq!(app.sanctuary.pets.len(), 1);

    app.handle_key(KeyCode::Char('n'));
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.sanctuary.pets.len(), 2);

    let before = app.pet.species;
    app.handle_key(KeyCode::Char('['));
    assert_ne!(app.pet.species, before);
    app.handle_key(KeyCode::Char(']'));
    assert_eq!(app.pet.species, before);

    app.handle_key(KeyCode::Char('l'));
    assert_eq!(app.screen, AppScreen::SanctuaryRoster);
    app.handle_key(KeyCode::Up);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.screen, AppScreen::Habitat);

    let incubation = app.pet.incubation;
    app.handle_key(KeyCode::Char('w'));
    assert!(app.pet.incubation > incubation || app.pet.stage != EvolutionStage::Egg);

    app.handle_key(KeyCode::Char('r'));
    app.handle_key(KeyCode::Backspace);
    app.handle_key(KeyCode::Char('z'));
    app.handle_key(KeyCode::Enter);
    assert!(!app.pet.name.is_empty());

    let theme = app.pet.theme;
    app.handle_key(KeyCode::Tab);
    assert_ne!(app.pet.theme, theme);
    app.handle_key(KeyCode::Char('q'));
    assert!(app.should_quit);
    let _ = theme_color(app.pet.theme);
}

#[test]
fn tui_renders_across_every_screen_and_terminal_size() {
    let (_dir, paths) = temp_paths();
    let mut sanctuary = Sanctuary::new_with_pet(Pet::new_custom(
        "Ignis",
        PetSpecies::IgnisDragon,
        ShellTheme::SunsetGold,
    ));
    sanctuary.add_pet(Pet::new_custom(
        "Ao",
        PetSpecies::AzureWyrm,
        ShellTheme::SkyBlue,
    ));
    let mut app = App::new_with_sanctuary(sanctuary, paths, false);
    app.pet.stage = EvolutionStage::Adult;
    app.pet.poop_count = 2;
    app.pet.log_action("test entry");

    assert_eq!(LayoutTier::from_width(60), LayoutTier::Compact);
    assert_eq!(LayoutTier::from_width(90), LayoutTier::Standard);
    assert_eq!(LayoutTier::from_width(200), LayoutTier::Wide);

    for (width, height) in [(50u16, 18u16), (78, 24), (100, 32), (132, 40)] {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("terminal");
        for screen in [
            AppScreen::Habitat,
            AppScreen::SanctuaryRoster,
            AppScreen::MiniGame,
            AppScreen::Onboarding,
        ] {
            app.screen = screen;
            terminal.draw(|f| draw_ui(f, &app)).expect("draw");
            let buffer = terminal.backend().buffer().clone();
            assert_eq!(buffer.area.width, width);
            assert_eq!(buffer.area.height, height);
        }
    }

    for mood in PetMood::all() {
        app.pet.mood = *mood;
        assert!(!get_mood_particles(*mood, 0).is_empty());
        assert!(!speech_bubble(&app.pet, 0).is_empty());
        assert!(!render_animated_sprite(&app.pet, 0).is_empty());
        assert!(!render_colored_sprite(&app.pet, 0).is_empty());
    }
}

#[test]
fn habitat_keys_pantry_and_help_drive_care_loop() {
    let (_dir, paths) = temp_paths();
    let mut app = App::new(Pet::new("Mochi"), paths);
    app.pet.stage = EvolutionStage::Adult;
    app.pet.hunger = 50;
    app.pet.energy = 90;

    // Open and use Pantry menu with 'm'
    app.handle_key(KeyCode::Char('m'));
    assert!(app.pantry_open);
    app.handle_key(KeyCode::Enter);
    assert!(!app.pantry_open);
    assert!(app.pet.hunger > 50);

    // Open and close Help modal with '?'
    app.handle_key(KeyCode::Char('?'));
    assert!(app.show_help);
    app.handle_key(KeyCode::Esc);
    assert!(!app.show_help);

    for key in [
        KeyCode::Char('p'),
        KeyCode::Char('t'),
        KeyCode::Char('e'),
        KeyCode::Char('s'),
        KeyCode::Char('s'),
        KeyCode::Char('c'),
        KeyCode::Char('h'),
        KeyCode::Char('g'),
    ] {
        app.handle_key(key);
    }
    assert_eq!(app.screen, AppScreen::MiniGame);

    while app.screen == AppScreen::MiniGame {
        app.on_tick();
    }
    assert_eq!(app.screen, AppScreen::Habitat);
    assert!(app.pet.xp > 0);
}
