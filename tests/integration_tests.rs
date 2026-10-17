use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::minigame::{ItemKind, MiniGame};
use mochigotchi::pet::{
    EvolutionStage, Pet, PetAction, PetMood, PetSpecies, PetState, ShellTheme,
};
use mochigotchi::sprites::{
    get_mood_particles, get_sprite_frames, mood_color, render_animated_sprite, speech_bubble,
    stage_color, theme_color,
};
use mochigotchi::storage::{
    has_existing_save, load_dual, load_json, load_or_create, load_sqlite, save_dual, save_json,
    save_sqlite, StoragePaths,
};
use mochigotchi::tui::{draw_ui, App, AppScreen, OnboardingStep};
use ratatui::{backend::TestBackend, Terminal};
use tempfile::tempdir;

#[test]
fn test_egg_initialization_and_guards() {
    let mut pet = Pet::new("Mochi");
    assert_eq!(pet.name, "Mochi");
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.incubation, 0);
    assert_eq!(pet.hunger, 65);
    assert_eq!(pet.xp, 0);
    assert_eq!(pet.level, 1);
    assert_eq!(pet.coins, 0);

    // Unhatched Egg guards: care actions should not alter stats, XP, or coins
    let msg_feed = pet.feed();
    assert!(!msg_feed.is_empty());
    assert_eq!(pet.hunger, 65);
    assert_eq!(pet.xp, 0);
    assert_eq!(pet.coins, 0);

    let msg_play = pet.play();
    assert!(!msg_play.is_empty());
    assert_eq!(pet.xp, 0);
    assert_eq!(pet.coins, 0);

    let msg_pet = pet.pet_love();
    assert!(!msg_pet.is_empty());
    assert_eq!(pet.xp, 0);

    let msg_train = pet.train();
    assert!(!msg_train.is_empty());
    assert_eq!(pet.xp, 0);

    let msg_sleep = pet.toggle_sleep();
    assert!(!msg_sleep.is_empty());
    assert!(!pet.is_sleeping);

    let msg_clean = pet.clean();
    assert!(!msg_clean.is_empty());
    assert_eq!(pet.xp, 0);

    let msg_heal = pet.heal();
    assert!(!msg_heal.is_empty());
    assert_eq!(pet.xp, 0);
}

#[test]
fn test_egg_warming_and_hatching() {
    let mut pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::MatchaGreen);
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.incubation, 0);

    for step in 1..=3 {
        let msg = pet.warm_egg();
        assert!(!msg.is_empty());
        assert_eq!(pet.incubation, step * 25);
        assert_eq!(pet.stage, EvolutionStage::Egg);
    }

    // 4th warm hatches Egg -> Baby
    let hatch_msg = pet.warm_egg();
    assert!(!hatch_msg.is_empty());
    assert_eq!(pet.incubation, 100);
    assert_eq!(pet.stage, EvolutionStage::Baby);
    assert!(pet.xp >= 50);
    assert!(pet.level >= 2);
    assert_eq!(pet.mood, PetMood::Happy);

    // Calling warm_egg() after hatching delegates to pet_love()
    pet.happiness = 70;
    let xp_before = pet.xp;
    let love_msg = pet.warm_egg();
    assert!(!love_msg.is_empty());
    assert!(pet.happiness > 70);
    assert!(pet.xp > xp_before);
}

#[test]
fn test_anti_spam_care_guards() {
    let mut pet = Pet::new("Cloudy");
    for _ in 0..4 {
        pet.warm_egg();
    }
    assert_eq!(pet.stage, EvolutionStage::Baby);

    // 1. Full stomach guard (hunger >= 95)
    pet.hunger = 95;
    let xp_before = pet.xp;
    let _ = pet.feed();
    assert_eq!(pet.xp, xp_before);

    // 2. Max happiness guard (happiness >= 98)
    pet.happiness = 98;
    let xp_before = pet.xp;
    let _ = pet.pet_love();
    assert_eq!(pet.happiness, 100);
    assert_eq!(pet.xp, xp_before);

    // 3. Clean room guard (cleanliness == 100 && poop_count == 0)
    pet.cleanliness = 100;
    pet.poop_count = 0;
    let xp_before = pet.xp;
    let _ = pet.clean();
    assert_eq!(pet.xp, xp_before);

    // 4. Healthy pet guard (!is_sick && health >= 95)
    pet.is_sick = false;
    pet.health = 95;
    let xp_before = pet.xp;
    let _ = pet.heal();
    assert_eq!(pet.xp, xp_before);

    // 5. Low energy / sleeping / sick guards for play() and train()
    pet.energy = 10;
    pet.hunger = 80;
    let xp_before = pet.xp;
    let _ = pet.play();
    let _ = pet.train();
    assert_eq!(pet.xp, xp_before);

    pet.energy = 80;
    pet.is_sleeping = true;
    let _ = pet.play();
    let _ = pet.train();
    assert_eq!(pet.xp, xp_before);

    pet.is_sleeping = false;
    pet.is_sick = true;
    let _ = pet.play();
    let _ = pet.train();
    assert_eq!(pet.xp, xp_before);
}

#[test]
fn test_feeding_poop_cycle_and_cleaning() {
    let mut pet = Pet::new("Mochi");
    for _ in 0..4 {
        pet.warm_egg();
    }

    pet.hunger = 30;
    pet.poop_count = 0;
    pet.meals_since_poop = 0;
    pet.cleanliness = 100;

    pet.feed();
    assert_eq!(pet.poop_count, 0);

    pet.hunger = 40;
    pet.feed();
    assert!(pet.poop_count >= 1);
    assert!(pet.cleanliness < 100);

    let xp_before = pet.xp;
    pet.clean();
    assert_eq!(pet.poop_count, 0);
    assert_eq!(pet.cleanliness, 100);
    assert!(pet.xp > xp_before);
}

#[test]
fn test_xp_and_evolution_stages() {
    assert_eq!(EvolutionStage::all().len(), 5);
    assert_eq!(EvolutionStage::from_xp(0), EvolutionStage::Egg);
    assert_eq!(EvolutionStage::from_xp(50), EvolutionStage::Baby);
    assert_eq!(EvolutionStage::from_xp(150), EvolutionStage::Teen);
    assert_eq!(EvolutionStage::from_xp(350), EvolutionStage::Adult);
    assert_eq!(EvolutionStage::from_xp(700), EvolutionStage::CyberElder);

    for stage in EvolutionStage::all() {
        assert!(!stage.name().is_empty());
        assert_eq!(stage.name(), stage.display_name());
        let _ = stage.min_xp();
        let _ = stage.max_stat_bonus();
        let _ = stage.next_stage();
    }

    let mut pet = Pet::new("Pebble");
    assert_eq!(pet.stage, EvolutionStage::Egg);

    pet.add_xp(50);
    assert_eq!(pet.stage, EvolutionStage::Baby);
    assert!(pet.xp_progress_ratio() >= 0.0 && pet.xp_progress_ratio() <= 1.0);

    pet.add_xp(100);
    assert_eq!(pet.stage, EvolutionStage::Teen);

    pet.add_xp(200);
    assert_eq!(pet.stage, EvolutionStage::Adult);

    pet.add_xp(400);
    assert_eq!(pet.stage, EvolutionStage::CyberElder);
    assert_eq!(pet.stage.next_stage(), None);
    assert_eq!(pet.xp_progress_ratio(), 1.0);
}

#[test]
fn test_species_themes_moods_and_actions() {
    assert_eq!(PetSpecies::all().len(), 4);
    for species in PetSpecies::all() {
        assert!(!species.display_name().is_empty());
        assert!(!species.description().is_empty());
        assert!(!species.default_name().is_empty());
        assert!(!species.favorite_snack().is_empty());
    }

    assert_eq!(ShellTheme::all().len(), 5);
    for theme in ShellTheme::all() {
        assert!(!theme.display_name().is_empty());
    }

    assert_eq!(PetMood::all().len(), 6);
    for mood in PetMood::all() {
        let state: PetState = *mood;
        assert!(!state.label().is_empty());
    }

    let mut pet = Pet::new("Tester");
    pet.rename("Starry");
    assert_eq!(pet.name, "Starry");

    // Warm via perform_action
    for _ in 0..4 {
        let _ = pet.perform_action(PetAction::WarmEgg);
    }
    assert_eq!(pet.stage, EvolutionStage::Baby);

    pet.hunger = 50;
    let _ = pet.perform_action(PetAction::Feed);
    let _ = pet.perform_action(PetAction::Play);
    pet.happiness = 70;
    let _ = pet.perform_action(PetAction::PetLove);
    let _ = pet.perform_action(PetAction::Train);
    let _ = pet.perform_action(PetAction::ToggleSleep);
    assert!(pet.is_sleeping);
    let _ = pet.perform_action(PetAction::ToggleSleep);
    assert!(!pet.is_sleeping);

    pet.poop_count = 1;
    pet.cleanliness = 70;
    let _ = pet.perform_action(PetAction::Clean);
    assert_eq!(pet.poop_count, 0);

    pet.is_sick = true;
    pet.health = 50;
    let _ = pet.perform_action(PetAction::Heal);
    assert!(!pet.is_sick);

    assert!(!pet.status_summary().is_empty());
}

#[test]
fn test_offline_decay_and_tick_decay() {
    // Unhatched Egg does not decay stats
    let mut egg = Pet::new("Eggy");
    egg.tick_decay();
    assert_eq!(egg.hunger, 65);

    let future = Utc::now() + Duration::hours(2);
    let elapsed = egg.apply_offline_decay(future);
    assert!(elapsed >= 7190);
    assert_eq!(egg.hunger, 65);

    // Hatched pet decays over time
    let mut pet = Pet::new("Mochi");
    for _ in 0..4 {
        pet.warm_egg();
    }
    pet.hunger = 80;
    pet.happiness = 80;
    pet.energy = 80;
    pet.tick_decay();
    assert!(pet.hunger < 80);

    let future2 = Utc::now() + Duration::hours(3);
    let _ = pet.apply_offline_decay(future2);
    assert!(pet.hunger < 75);
}

#[test]
fn test_dual_storage_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let json_path = dir.path().join("pet.json");
    let sqlite_path = dir.path().join("pet.db");
    let paths = StoragePaths::new(json_path.clone(), sqlite_path.clone());

    assert!(!has_existing_save(&paths));

    let fresh = load_or_create(&paths, "DefaultMochi");
    assert_eq!(fresh.name, "DefaultMochi");

    let mut pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::LavenderDream);
    for _ in 0..4 {
        pet.warm_egg();
    }
    pet.coins = 42;

    save_dual(&pet, &paths).expect("save_dual should succeed");
    assert!(has_existing_save(&paths));

    let from_json = load_json(&json_path).expect("load_json");
    assert_eq!(from_json.name, "Boba");
    assert_eq!(from_json.species, PetSpecies::BobaPup);
    assert_eq!(from_json.theme, ShellTheme::LavenderDream);
    assert_eq!(from_json.coins, 42);

    let from_sql = load_sqlite(&sqlite_path).expect("load_sqlite");
    assert_eq!(from_sql.name, "Boba");
    assert_eq!(from_sql.coins, 42);

    save_json(&pet, &json_path).expect("save_json");
    save_sqlite(&pet, &sqlite_path).expect("save_sqlite");

    let loaded = load_dual(&paths).expect("load_dual");
    assert_eq!(loaded.name, "Boba");
    assert_eq!(loaded.stage, EvolutionStage::Baby);
}

#[test]
fn test_sprites_and_speech_bubbles() {
    for species in PetSpecies::all() {
        for stage in EvolutionStage::all() {
            let _ = stage_color(*stage);
            for mood in PetMood::all() {
                let _ = mood_color(*mood);
                let _ = get_mood_particles(*mood, 0);
                let frames = get_sprite_frames(*species, *stage, *mood);
                assert!(!frames.is_empty());
            }
        }
    }

    for theme in ShellTheme::all() {
        let _ = theme_color(*theme);
    }

    let pet = Pet::new("Mochi");
    for tick in 0..8 {
        assert!(!speech_bubble(&pet, tick).is_empty());
        assert!(!render_animated_sprite(&pet, tick).is_empty());
    }
}

#[test]
fn test_minigame_mechanics_and_rewards() {
    let mut game = MiniGame::new(40, 16, 20);
    assert!(!game.is_finished());

    let initial_x = game.player_x;
    game.move_left();
    assert!(game.player_x <= initial_x);
    game.move_right();
    game.move_right();

    for kind in [ItemKind::Snack, ItemKind::Star, ItemKind::Hazard] {
        assert!(!kind.symbol().is_empty());
        assert!(!kind.label().is_empty());
        let _ = kind.is_reward();
    }

    game.spawn_item(game.player_x, ItemKind::Snack);
    assert!(!game.render_lines().is_empty());

    for _ in 0..20 {
        game.step();
    }
    assert!(game.is_finished());
    assert_eq!(game.ticks_remaining, 0);

    let mut pet = Pet::new("Mochi");
    for _ in 0..4 {
        pet.warm_egg();
    }
    let xp_before = pet.xp;
    let coins_before = pet.coins;
    let reward_msg = pet.apply_minigame_reward(120, 18, 35);
    assert!(!reward_msg.is_empty());
    assert_eq!(pet.coins, coins_before + 18);
    assert!(pet.xp >= xp_before + 35);
}

#[test]
fn test_tui_onboarding_and_responsive_rendering() {
    let dir = tempdir().expect("tempdir");
    let paths = StoragePaths::new(dir.path().join("pet.json"), dir.path().join("pet.db"));

    // 1. Onboarding wizard flow
    let mut app = App::new_onboarding(paths.clone());
    assert_eq!(app.screen, AppScreen::Onboarding);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectSpecies);

    let backend = TestBackend::new(90, 28);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|f| draw_ui(f, &app))
        .expect("draw onboarding species");

    app.handle_key(KeyCode::Right);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);
    terminal
        .draw(|f| draw_ui(f, &app))
        .expect("draw onboarding theme");

    app.handle_key(KeyCode::Right);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
    terminal
        .draw(|f| draw_ui(f, &app))
        .expect("draw onboarding name");

    app.handle_key(KeyCode::Backspace);
    app.handle_key(KeyCode::Char('K'));
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.screen, AppScreen::Habitat);

    // 2. Habitat rendering across Compact (60x22), Standard (90x28), and Wide Deluxe (120x34)
    for (w, h) in [(60, 22), (90, 28), (120, 34)] {
        let b = TestBackend::new(w, h);
        let mut term = Terminal::new(b).expect("term");
        term.draw(|f| draw_ui(f, &app)).expect("draw habitat");
    }

    // 3. Hatch pet via keyboard 'w' and enter MiniGame via 'g'
    for _ in 0..4 {
        app.handle_key(KeyCode::Char('w'));
    }
    assert_eq!(app.pet.stage, EvolutionStage::Baby);

    app.on_tick();
    app.handle_key(KeyCode::Char('g'));
    assert_eq!(app.screen, AppScreen::MiniGame);

    terminal
        .draw(|f| draw_ui(f, &app))
        .expect("draw minigame");

    app.handle_key(KeyCode::Left);
    app.handle_key(KeyCode::Right);
    app.handle_key(KeyCode::Esc);
    assert_eq!(app.screen, AppScreen::Habitat);
}
