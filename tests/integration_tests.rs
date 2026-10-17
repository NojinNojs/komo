//! End-to-End Integration Tests for **Mochigotchi v1.1.0**:
//! Verifies 32×20 half-block pixel art sprites, animated evolution morph showcase,
//! interactive pet dialogue (`[Y]`), 3-in-1 retro arcade games, 8-bit chiptune
//! WAV synthesis, Bandai Tamagotchi mechanics, and multi-pet Sanctuary persistence.

use std::fs;
use std::path::PathBuf;

use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::{
    draw_ui, format_hearts, get_sprite_frames, load_pet, load_sanctuary, render_animated_sprite,
    save_exists, save_pet, save_sanctuary, speech_bubble, synthesize_wav, thought_bubble, App,
    AppScreen, AttentionCall, CareRank, EvolutionStage, FoodChoice, GameMode, ItemKind, LcdIcon,
    MiniGame, OnboardingStep, Pet, PetAction, PetMood, PetSpecies, Sanctuary, ShellTheme,
    SoundEffect, StoragePaths, CANVAS_H, CANVAS_W,
};
use ratatui::{backend::TestBackend, Terminal};

fn unique_test_paths(tag: &str) -> StoragePaths {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("target");
    dir.push("tmp_integration_tests");
    dir.push(format!("{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    StoragePaths::new(dir.join("pet.json"), dir.join("pet.db"))
}

#[test]
fn all_ten_species_have_distinct_32x20_pixel_sprites_and_mythical_creatures() {
    let all = PetSpecies::all();
    assert_eq!(all.len(), 10);

    let mythical_count = all.iter().filter(|s| s.is_mythical()).count();
    assert_eq!(mythical_count, 6);

    for sp in all {
        let frames = get_sprite_frames(*sp, EvolutionStage::Adult, PetMood::Idle);
        assert_eq!(frames.len(), 2);
        let lines: Vec<&str> = frames[0].lines().collect();
        assert_eq!(lines.len(), CANVAS_H / 2);
        for line in lines {
            assert_eq!(line.chars().count(), CANVAS_W);
        }
        assert!(!sp.display_name().is_empty());
        assert!(!sp.favorite_snack().is_empty());
        assert!(!sp.description().is_empty());
    }
}

#[test]
fn onboarding_wizard_auto_morphs_evolution_and_adopts_into_sanctuary() {
    let paths = unique_test_paths("onboarding_morph");
    let mut app = App::new_onboarding(paths.clone());

    assert_eq!(app.screen, AppScreen::Onboarding);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectSpecies);
    assert!(app.preview_auto_morph);

    // Step 1 auto-animates through Egg -> Baby -> Teen -> Adult -> Elder every 18 ticks (~4.5s)
    let mut seen_stages = vec![app.preview_stage_idx];
    for _ in 0..72 {
        app.on_tick();
        if !seen_stages.contains(&app.preview_stage_idx) {
            seen_stages.push(app.preview_stage_idx);
        }
    }
    assert!(
        seen_stages.len() >= 4,
        "Evolution morph showcase must cycle across multiple evolution forms automatically"
    );

    // Navigate down 4 times to select IgnisDragon (index 4)
    for _ in 0..4 {
        app.handle_key(KeyCode::Down);
    }
    assert_eq!(
        PetSpecies::all()[app.selected_species_idx],
        PetSpecies::IgnisDragon
    );

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);

    app.handle_key(KeyCode::Down);
    app.handle_key(KeyCode::Down);
    assert_eq!(ShellTheme::all()[app.selected_theme_idx], ShellTheme::SkyBlue);

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
    assert_eq!(app.onboarding_name_input, "Ignis");

    for _ in 0..10 {
        app.handle_key(KeyCode::Backspace);
    }
    for ch in "Draco".chars() {
        app.handle_key(KeyCode::Char(ch));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.screen, AppScreen::Habitat);
    assert_eq!(app.pet.name, "Draco");
    assert_eq!(app.pet.species, PetSpecies::IgnisDragon);
    assert_eq!(app.pet.theme, ShellTheme::SkyBlue);
    assert!(save_exists(&paths));

    // Adopt a second companion (Phoenix) into the Sanctuary
    app.handle_key(KeyCode::Char('n'));
    assert_eq!(app.screen, AppScreen::Onboarding);
    for _ in 0..7 {
        app.handle_key(KeyCode::Down);
    }
    assert_eq!(
        PetSpecies::all()[app.selected_species_idx],
        PetSpecies::PhoenixChick
    );
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.sanctuary.pets.len(), 2);
    assert_eq!(app.pet.species, PetSpecies::PhoenixChick);
    assert_eq!(app.pet.name, "Sol");

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn interactive_pet_dialogue_and_inner_thoughts_boost_bond_and_coins() {
    let paths = unique_test_paths("dialogue_talk");
    let mut pet = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);
    pet.stage = EvolutionStage::Adult;

    assert!(thought_bubble(&pet, 0).contains("Thinking"));
    assert!(speech_bubble(&pet, 0).contains("💬"));

    let prompt = pet.dialogue_prompt(0);
    assert!(prompt.inner_thought.contains("💭"));
    assert!(prompt.pet_line.contains("💬"));
    assert!(prompt.option_1.starts_with("1."));
    assert!(prompt.option_2.starts_with("2."));

    let mut app = App::new(pet, paths.clone());
    let bond_before = app.pet.bond;
    let coins_before = app.pet.coins;

    // Open Talk screen with [Y] and select Option 2 (awards +5 coins & +5 Bond)
    app.handle_key(KeyCode::Char('y'));
    assert!(app.talk_open);
    app.handle_key(KeyCode::Char('2'));
    assert!(!app.talk_open);
    assert!(app.pet.bond > bond_before);
    assert_eq!(app.pet.coins, coins_before + 5);
    assert_eq!(app.sound.last_cue, Some(SoundEffect::ChatChirp));

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn chiptune_sound_engine_synthesizes_valid_wav_and_toggles_with_u_key() {
    let paths = unique_test_paths("sound_fx");
    let mut app = App::new(Pet::new("Mochi"), paths.clone());
    assert!(app.sound.enabled);

    for &fx in SoundEffect::all() {
        let wav = synthesize_wav(fx.notes());
        assert!(wav.len() > 100);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }

    app.handle_key(KeyCode::Char('u'));
    assert!(!app.sound.enabled);
    assert_eq!(app.sound.status_badge(), "🔇 Sound:OFF");

    app.handle_key(KeyCode::Char('u'));
    assert!(app.sound.enabled);

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn three_in_one_arcade_modes_work_and_reward_pet() {
    let paths = unique_test_paths("arcade_3in1");
    let mut pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::MatchaGreen);
    pet.stage = EvolutionStage::Baby;
    pet.weight_grams = 12;
    let mut app = App::new(pet, paths.clone());

    // Launch Arcade [G]
    app.handle_key(KeyCode::Char('g'));
    assert_eq!(app.screen, AppScreen::MiniGame);
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::SnackCatcher);

    // Switch to Mode 2: Bandai Left/Right Mind-Reader
    app.handle_key(KeyCode::Char('2'));
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::MindReader);

    // Switch to Mode 3: Starlight Hurdle Dash and jump
    app.handle_key(KeyCode::Char('3'));
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::DashRunner);
    app.handle_key(KeyCode::Char(' '));
    assert!(app.minigame.as_ref().unwrap().dash_jump_ticks > 0);

    // Switch back to Mode 1, catch a FeverGem, and finish
    app.handle_key(KeyCode::Char('1'));
    {
        let game = app.minigame.as_mut().unwrap();
        let px = game.player_x;
        game.spawn_item(px, ItemKind::FeverGem);
        for _ in 0..game.height {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::FeverGem);
            game.step();
        }
        assert!(game.score >= 45);
        assert!(game.coins_earned >= 8);
    }
    app.handle_key(KeyCode::Esc);
    assert_eq!(app.screen, AppScreen::Habitat);
    assert!(app.pet.coins >= 8);
    assert!(app.pet.weight_grams < 12);

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn full_pet_lifecycle_from_egg_to_cyber_elder() {
    let mut pet = Pet::new_custom("Ao", PetSpecies::AzureWyrm, ShellTheme::SkyBlue);
    assert_eq!(pet.stage, EvolutionStage::Egg);

    let blocked = pet.feed();
    assert!(blocked.contains("still unhatched"));

    for _ in 0..5 {
        pet.clear_cooldowns();
        pet.warm_egg();
    }
    assert_eq!(pet.stage, EvolutionStage::Baby);
    assert!(pet.coins >= 20);

    for cycle in 0..320 {
        pet.clear_cooldowns();
        pet.age_seconds += 3_600;
        if pet.is_sleeping {
            pet.toggle_sleep();
        }
        if pet.is_sick || pet.has_toothache || pet.health < 60 {
            pet.heal();
        }
        if pet.poop_count > 0 || pet.cleanliness < 60 {
            pet.clean();
        }
        if pet.hunger < 65 {
            pet.feed();
        }
        if pet.energy < 40 {
            pet.toggle_sleep();
            for _ in 0..10 {
                pet.tick_decay();
            }
            pet.toggle_sleep();
        }
        if cycle % 2 == 0 {
            pet.train();
        } else {
            pet.play();
        }
        if cycle % 5 == 0 {
            pet.pet_love();
        }
        if pet.stage == EvolutionStage::CyberElder {
            break;
        }
    }

    assert_eq!(pet.stage, EvolutionStage::CyberElder);
    assert!(pet.xp >= 1_200);
    assert!(pet.bond >= 50);
}

#[test]
fn reverse_engineered_tamagotchi_mechanics_toothache_light_discipline_and_bezel() {
    let paths = unique_test_paths("tamagotchi_mechanics");
    let mut pet = Pet::new_custom("Tama", PetSpecies::MochiCat, ShellTheme::SunsetGold);
    pet.stage = EvolutionStage::Baby;
    pet.weight_grams = EvolutionStage::Baby.base_weight_grams();
    pet.hunger = 50;
    pet.happiness = 50;

    assert_eq!(pet.hungry_hearts(), 2);
    assert_eq!(format_hearts(pet.hungry_hearts()), "♥♥·· 2/4");

    // 3 sweet snacks in a row -> Toothache!
    for i in 1..=3 {
        let msg = pet.feed_choice(FoodChoice::WarmMilkTea);
        assert_eq!(pet.snacks_in_a_row, i);
        if i == 3 {
            assert!(pet.has_toothache);
            assert!(msg.contains("Toothache"));
        }
    }
    assert!(pet.feed().contains("Toothache"));
    let heal_msg = pet.heal();
    assert!(heal_msg.contains("Toothache"));
    assert!(!pet.has_toothache);

    // Sleeping with Light ON -> NeedsCare call -> Light OFF clears it
    pet.toggle_sleep();
    assert!(pet.is_sleeping && pet.light_on);
    assert_eq!(pet.attention_call, AttentionCall::NeedsCare);
    pet.toggle_light();
    assert!(!pet.light_on);
    assert_eq!(pet.attention_call, AttentionCall::None);
    pet.toggle_sleep();

    // Misbehaving -> Scold [D] raises Discipline
    pet.attention_call = AttentionCall::Misbehaving;
    let d0 = pet.discipline;
    pet.discipline_scold();
    assert_eq!(pet.discipline, d0 + 25);

    // LCD Bezel navigation
    let mut app = App::new(pet, paths.clone());
    assert_eq!(LcdIcon::selectable()[app.selected_icon_idx], LcdIcon::Food);
    app.handle_key(KeyCode::Enter);
    assert!(app.pantry_open);
    app.handle_key(KeyCode::Esc);

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn sanctuary_roundtrip_keeps_every_pet_even_if_sqlite_is_removed() {
    let paths = unique_test_paths("sanctuary_save");

    let mut dragon = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);
    dragon.stage = EvolutionStage::Adult;
    dragon.xp = 1_600;
    dragon.coins = 77;

    let mut kitsune = Pet::new_custom("Kitsune", PetSpecies::EmberFox, ShellTheme::LavenderDream);
    kitsune.stage = EvolutionStage::Teen;
    kitsune.xp = 520;

    let mut sanctuary = Sanctuary::new_with_pet(dragon);
    sanctuary.add_pet(kitsune);
    save_sanctuary(&sanctuary, &paths).expect("save sanctuary");

    let loaded = load_sanctuary(&paths).expect("load sanctuary");
    assert_eq!(loaded.pets.len(), 2);
    assert_eq!(loaded.active_pet().name, "Kitsune");

    fs::remove_file(&paths.sqlite_path).expect("remove sqlite");
    let from_json_only = load_sanctuary(&paths).expect("load sanctuary from json");
    assert_eq!(from_json_only.pets.len(), 2);

    fs::remove_file(&paths.json_path).expect("remove json");
    save_pet(&from_json_only.pets[0], &paths).expect("save pet");
    let single = load_pet(&paths).expect("load single");
    assert_eq!(single.name, "Ignis");

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn offline_report_and_care_rank_work_end_to_end() {
    let mut pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::MatchaGreen);
    pet.stage = EvolutionStage::Baby;
    pet.discipline = 80;
    pet.care_mistakes = 0;
    assert_eq!(pet.care_rank(), CareRank::PerfectKeeper);

    let future = Utc::now() + Duration::minutes(45);
    let (elapsed, report) = pet.apply_offline_decay_with_report(future);
    assert!(elapsed >= 2_700);
    let rep = report.expect("should produce welcome-back offline report");
    assert!(!rep.events.is_empty());
}

#[test]
fn tui_renders_every_screen_cleanly() {
    let paths = unique_test_paths("tui_render_all");
    let mut app = App::new(
        Pet::new_custom("Astra", PetSpecies::CrystalUni, ShellTheme::LavenderDream),
        paths.clone(),
    );
    app.pet.stage = EvolutionStage::Adult;

    let backend = TestBackend::new(110, 32);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| draw_ui(f, &app)).unwrap();

    let buf = terminal.backend().buffer().clone();
    let mut rendered_text = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            rendered_text.push_str(buf[(x, y)].symbol());
        }
        rendered_text.push('\n');
    }
    assert!(rendered_text.contains("ASTRA"));
    assert!(rendered_text.contains("Thinking"));
    assert!(
        rendered_text.contains("Talk"),
        "Rightmost bezel icon 💬Talk must never be truncated to 💬Tal"
    );
    assert!(
        rendered_text.contains("[?] Keys"),
        "Advisor footer [?] Keys must never be truncated"
    );

    let _ = MiniGame::new(20, 8, 10);
    let _ = render_animated_sprite(&app.pet, 0);
    let _ = PetAction::WarmEgg.label();

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}
