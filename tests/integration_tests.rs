//! End-to-End Integration Tests for **Mochigotchi v1.0.0**:
//! Verifies 40×22 3/4 side-profile pixel art sprites, 15 rich emotions,
//! starvation/sickness/death (`PassedOut`) & Phoenix Tear revival (`[H]`),
//! direct first-person pet feedback when spamming actions, animated evolution
//! morph showcase, interactive pet dialogue (`[Y]`), 3-in-1 retro arcade games,
//! and 8-bit chiptune WAV synthesis.

use std::fs;
use std::path::PathBuf;

use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::{
    build_grid_with_gaze, compute_posture_and_gaze, draw_ui, format_hearts, get_sprite_frames,
    load_pet, load_sanctuary, render_animated_sprite, save_exists, save_pet, save_sanctuary,
    sense_environment, speech_bubble, synthesize_wav, thought_bubble, App, AppScreen,
    AttentionCall, CareRank, EvolutionStage, FoodChoice, GameMode, ItemKind, LcdIcon, MiniGame,
    OnboardingStep, Pet, PetAction, PetGaze, PetMood, PetSpecies, Sanctuary, ShellTheme,
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
fn all_ten_species_have_distinct_40x22_pixel_sprites_and_mythical_creatures() {
    let all = PetSpecies::all();
    assert_eq!(all.len(), 10);

    let mythical_count = all.iter().filter(|s| s.is_mythical()).count();
    assert_eq!(mythical_count, 6);

    for sp in all {
        let frames = get_sprite_frames(*sp, EvolutionStage::CyberElder, PetMood::Idle);
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
fn realistic_starvation_sickness_death_and_phoenix_revival_work() {
    let mut pet = Pet::new_custom("Astra", PetSpecies::CrystalUni, ShellTheme::LavenderDream);
    pet.stage = EvolutionStage::Adult;
    pet.hunger = 0;
    pet.cleanliness = 10;
    pet.poop_count = 4;

    // Tick until health reaches 0 from starvation & untreated illness
    for _ in 0..25 {
        pet.tick_decay();
        if !pet.is_alive {
            break;
        }
    }
    assert!(!pet.is_alive);
    assert_eq!(pet.health, 0);
    assert_eq!(pet.mood, PetMood::PassedOut);
    assert!(pet.cause_of_death.is_some());

    // Normal actions are blocked while PassedOut
    let blocked_feed = pet.feed();
    assert!(blocked_feed.contains("passed out"));
    assert!(speech_bubble(&pet, 0).contains("Phoenix"));

    // 1st Phoenix Tear Revival works (1× per lifetime) and adds +3 Care Mistakes!
    let revive_msg = pet.heal();
    assert!(revive_msg.contains("PHOENIX REVIVAL"));
    assert!(pet.is_alive);
    assert_eq!(pet.revival_count, 1);
    assert_eq!(pet.health, 80);
    assert_eq!(pet.mood, PetMood::Loved);

    // If the pet dies a SECOND time, Phoenix Tear cannot revive them again — must hold 3-step Vigil!
    pet.health = 0;
    pet.is_alive = false;
    pet.mood = PetMood::PassedOut;
    let second_revive = pet.heal();
    assert!(second_revive.contains("one-time Phoenix Revival"));
    assert!(!pet.is_alive);

    // Complete 3-step Burial Vigil Rite (`[B]`)
    assert!(pet.hold_vigil_step().contains("Step 1/3"));
    assert!(!pet.buried);
    assert!(pet.hold_vigil_step().contains("Step 2/3"));
    assert!(!pet.buried);
    assert!(pet.hold_vigil_step().contains("Vigil Complete (3/3)"));
    assert!(pet.buried);
}

#[test]
fn direct_first_person_pet_feedback_on_spam_feeding_scolding_and_patting() {
    let mut pet = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);
    pet.set_caretaker_title("Aqsan");
    pet.stage = EvolutionStage::Adult;
    pet.hunger = 100;

    // 1st feed when already full (4/4 hearts)
    let _ = pet.feed();
    assert_eq!(pet.mood, PetMood::Overstuffed);
    assert!(speech_bubble(&pet, 0).contains("completely full"));
    assert!(speech_bubble(&pet, 0).contains("Aqsan"));

    // 2nd spam feed when full -> pet complains about stomach cramping and loses health/bond!
    let health_before = pet.health;
    let _ = pet.feed();
    assert_eq!(pet.mood, PetMood::Overstuffed);
    assert!(speech_bubble(&pet, 0).contains("UGH!"));
    assert!(pet.health < health_before);

    // Unfair scold spam -> pet gets sad then Angry and turns away!
    let _ = pet.discipline_scold();
    assert_eq!(pet.mood, PetMood::Lonely);
    let _ = pet.discipline_scold();
    assert_eq!(pet.mood, PetMood::Angry);
    assert!(speech_bubble(&pet, 0).contains("HMPH!"));
    assert_eq!(compute_posture_and_gaze(&pet, 0, 4).1, PetGaze::Right);

    // Patting spam -> pet giggles that it's ticklish!
    pet.happiness = 70;
    let _ = pet.pet_love();
    assert_eq!(pet.mood, PetMood::Loved);
    let _ = pet.pet_love();
    assert_eq!(pet.mood, PetMood::Playful);
    assert!(speech_bubble(&pet, 0).contains("tickles"));

    // Verify 3-way gaze (Left, Center, Right) produces distinct grids
    let g_left = build_grid_with_gaze(
        PetSpecies::MochiCat,
        EvolutionStage::Adult,
        PetMood::Happy,
        0,
        0,
        PetGaze::Left,
    );
    let g_center = build_grid_with_gaze(
        PetSpecies::MochiCat,
        EvolutionStage::Adult,
        PetMood::Happy,
        0,
        0,
        PetGaze::Center,
    );
    let g_right = build_grid_with_gaze(
        PetSpecies::MochiCat,
        EvolutionStage::Adult,
        PetMood::Happy,
        0,
        0,
        PetGaze::Right,
    );
    assert_ne!(g_left, g_center);
    assert_ne!(g_center, g_right);

    // Verify sensory snapshot includes custom caretaker title
    let snap = sense_environment(&pet, 0, LcdIcon::Food, 4, 1);
    assert!(snap.sight.contains("Aqsan"));
}

#[test]
fn onboarding_wizard_auto_morphs_evolution_and_adopts_into_sanctuary() {
    let paths = unique_test_paths("onboarding_morph");
    let mut app = App::new_onboarding(paths.clone());

    assert_eq!(app.screen, AppScreen::Onboarding);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectSpecies);
    assert!(app.preview_auto_morph);

    let mut seen_stages = vec![app.preview_stage_idx];
    for _ in 0..72 {
        app.on_tick();
        if !seen_stages.contains(&app.preview_stage_idx) {
            seen_stages.push(app.preview_stage_idx);
        }
    }
    assert!(seen_stages.len() >= 4);

    for _ in 0..4 {
        app.handle_key(KeyCode::Down);
    }
    assert_eq!(
        PetSpecies::curated()[app.selected_species_idx],
        PetSpecies::BobaPup
    );

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);

    app.handle_key(KeyCode::Down);
    app.handle_key(KeyCode::Down);
    assert_eq!(
        ShellTheme::all()[app.selected_theme_idx],
        ShellTheme::SkyBlue
    );

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
    assert_eq!(app.onboarding_name_input, "Boba");

    for _ in 0..10 {
        app.handle_key(KeyCode::Backspace);
    }
    for ch in "Draco".chars() {
        app.handle_key(KeyCode::Char(ch));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.screen, AppScreen::Habitat);
    assert_eq!(app.pet.name, "Draco");
    assert_eq!(app.pet.species, PetSpecies::BobaPup);
    assert_eq!(app.pet.theme, ShellTheme::SkyBlue);
    assert!(save_exists(&paths));

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn interactive_pet_dialogue_and_inner_thoughts_boost_bond_and_coins() {
    let paths = unique_test_paths("dialogue_talk");
    let mut pet = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);
    pet.stage = EvolutionStage::Adult;

    assert!(thought_bubble(&pet, 0).contains("👁") || thought_bubble(&pet, 0).contains("💭"));
    assert!(speech_bubble(&pet, 0).contains("💬"));

    let prompt = pet.dialogue_prompt(0);
    assert!(prompt.inner_thought.contains("💭"));
    assert!(prompt.pet_line.contains("💬"));
    assert!(prompt.option_1.starts_with("1."));
    assert!(prompt.option_2.starts_with("2."));

    let mut app = App::new(pet, paths.clone());
    let bond_before = app.pet.bond;
    let coins_before = app.pet.coins;

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
    pet.weight_grams = 600;
    let mut app = App::new(pet, paths.clone());

    app.handle_key(KeyCode::Char('g'));
    assert_eq!(app.screen, AppScreen::MiniGame);
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::SnackCatcher);

    app.handle_key(KeyCode::Char('2'));
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::MindReader);

    app.handle_key(KeyCode::Char('3'));
    assert_eq!(app.minigame.as_ref().unwrap().mode, GameMode::DashRunner);
    app.handle_key(KeyCode::Char(' '));
    assert!(app.minigame.as_ref().unwrap().dash_jump_ticks > 0);

    app.handle_key(KeyCode::Char('1'));
    {
        let game = app.minigame.as_mut().unwrap();
        let px = game.player_x;
        game.spawn_item(px, ItemKind::FeverGem);
        for _ in 0..game.height {
            game.items
                .retain(|i| i.x == px && i.kind == ItemKind::FeverGem);
            game.step();
        }
        assert!(game.score >= 45);
        assert!(game.coins_earned >= 8);
    }
    app.handle_key(KeyCode::Esc);
    assert_eq!(app.screen, AppScreen::Habitat);
    assert!(app.pet.coins >= 8);
    assert!(app.pet.weight_grams < 600);

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

    for cycle in 0..220 {
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
    assert!(pet.xp >= 2_200);
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

    pet.toggle_sleep();
    assert!(pet.is_sleeping && pet.light_on);
    assert_eq!(pet.attention_call, AttentionCall::NeedsCare);
    pet.toggle_light();
    assert!(!pet.light_on);
    assert_eq!(pet.attention_call, AttentionCall::None);
    pet.toggle_sleep();

    pet.attention_call = AttentionCall::Misbehaving;
    let d0 = pet.discipline;
    pet.discipline_scold();
    assert_eq!(pet.discipline, d0 + 25);

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
    assert!(rendered_text.contains("Talk"));
    assert!(rendered_text.contains("[?] Keys"));

    let _ = MiniGame::new(20, 8, 10);
    let _ = render_animated_sprite(&app.pet, 0);
    let _ = PetAction::WarmEgg.label();

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}

#[test]
fn talk_dialogue_reward_cooldown_and_sleep_light_exploit_guard_work() {
    let mut pet = Pet::new_custom("Mochi", PetSpecies::MochiCat, ShellTheme::StrawberryPink);
    pet.stage = EvolutionStage::Adult;
    pet.set_caretaker_title("Aqsan");

    let xp0 = pet.xp;
    let coins0 = pet.coins;
    let msg1 = pet.answer_dialogue(2);
    assert!(msg1.contains("+5 coins"));
    assert_eq!(pet.xp, xp0 + 8);
    assert_eq!(pet.coins, coins0 + 5);
    assert!(pet.last_talk_reward_at.is_some());
    assert!(pet.talk_reward_cooldown_remaining(Utc::now()) > 0);

    // Immediate 2nd talk while on 180s cooldown: chats playfully without granting XP or coins!
    let xp_after_first = pet.xp;
    let coins_after_first = pet.coins;
    let msg2 = pet.answer_dialogue(1);
    assert!(msg2.contains("throat's getting dry"));
    assert_eq!(pet.xp, xp_after_first);
    assert_eq!(pet.coins, coins_after_first);

    // After 181s elapsed, talk reward grants again
    pet.last_talk_reward_at = Some(Utc::now() - Duration::seconds(181));
    assert_eq!(pet.talk_reward_cooldown_remaining(Utc::now()), 0);
    let _ = pet.answer_dialogue(1);
    assert_eq!(pet.xp, xp_after_first + 12);

    // Sleep light toggle spam guard: only grants XP once per cooldown window
    pet.is_sleeping = true;
    pet.light_on = true;
    let xp_before_light = pet.xp;
    let _ = pet.toggle_light(); // OFF -> grants +5 XP
    assert_eq!(pet.xp, xp_before_light + 5);
    let _ = pet.toggle_light(); // ON
    let _ = pet.toggle_light(); // OFF again on cooldown -> 0 XP
    assert_eq!(pet.xp, xp_before_light + 5);
}

#[test]
fn welcome_home_hosting_greets_keeper_with_toward_keeper_gaze_and_choices() {
    let mut pet = Pet::new_custom("Boba", PetSpecies::BobaPup, ShellTheme::MatchaGreen);
    pet.stage = EvolutionStage::Adult;
    pet.set_caretaker_title("Boss");

    // Medium absence (2 hours): companion greets TowardKeeper and finds lucky coins
    let future = Utc::now() + Duration::hours(2);
    let (elapsed, report) = pet.apply_offline_decay_with_report(future);
    assert!(elapsed >= 7_200);
    assert!(report.is_some());

    let host = pet
        .pending_hosting
        .as_ref()
        .expect("should prepare welcome hosting event");
    assert_eq!(host.gaze, PetGaze::TowardKeeper);
    assert!(host.greeting_speech.contains("Boss"));
    assert_eq!(
        compute_posture_and_gaze(&pet, 0, 0).1,
        PetGaze::TowardKeeper
    );

    let coins_before = pet.coins;
    let bond_before = pet.bond;
    let reply = pet.respond_welcome_hosting(3);
    assert!(reply.contains("while away"));
    assert!(pet.pending_hosting.is_none());
    assert!(pet.coins >= coins_before + 5);
    assert!(pet.bond > bond_before);
}

#[test]
fn wardrobe_customization_and_photo_booth_share_card_work_end_to_end() {
    let paths = unique_test_paths("wardrobe_photo");
    let mut pet = Pet::new_custom("Cloudy", PetSpecies::CloudBun, ShellTheme::SkyBlue);
    pet.stage = EvolutionStage::Adult;
    pet.coins = 150;
    pet.level = 5;

    // Equip/buy CoatPreset and Accessory
    let coat_msg = pet.equip_or_buy_coat(mochigotchi::CoatPreset::SakuraMochi);
    assert!(coat_msg.contains("Sakura Mochi"));
    assert_eq!(pet.coat, mochigotchi::CoatPreset::SakuraMochi);

    let acc_msg = pet.equip_or_buy_accessory(mochigotchi::Accessory::TinyCrown);
    assert!(acc_msg.contains("Tiny Crown"));
    assert_eq!(pet.accessory, mochigotchi::Accessory::TinyCrown);

    // Persist and reload via Sanctuary
    let sanctuary = Sanctuary::new_with_pet(pet);
    save_sanctuary(&sanctuary, &paths).unwrap();
    let loaded = load_sanctuary(&paths).unwrap();
    assert_eq!(
        loaded.active_pet().coat,
        mochigotchi::CoatPreset::SakuraMochi
    );
    assert_eq!(
        loaded.active_pet().accessory,
        mochigotchi::Accessory::TinyCrown
    );

    // Open Wardrobe [V] and Photo Booth [X] in TUI and export PNG card
    let mut app = App::new_with_sanctuary(loaded, paths.clone(), false);
    assert!(app.show_changelog);
    app.handle_key(KeyCode::Enter);
    assert!(!app.show_changelog);
    app.handle_key(KeyCode::Char('v'));
    assert!(app.wardrobe_open);
    app.handle_key(KeyCode::Esc);
    assert!(!app.wardrobe_open);

    app.handle_key(KeyCode::Char('x'));
    assert!(app.photo_booth_open);
    app.handle_key(KeyCode::Right);
    assert_eq!(app.photo_ratio, mochigotchi::ShareRatio::Story9x16);

    let card_bytes =
        mochigotchi::render_share_card_png(&app.pet, mochigotchi::ShareRatio::Landscape16x9);
    assert_eq!(&card_bytes[..8], b"\x89PNG\r\n\x1a\n");

    let _ = fs::remove_dir_all(paths.json_path.parent().unwrap());
}
