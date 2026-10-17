//! End-to-end integration tests for **Mochigotchi v1.0.0**:
//! - 24×16 TrueColor Half-Block Pixel Art (`▀`, `▄`, `█`) with isolated `E`/`K`
//!   eye-blink channels so white body pixels (`W`) never flicker dark
//! - Minimalist step-by-step Retro Handheld LCD UI (`Step 1/3` -> `Step 2/3` ->
//!   `Step 3/3`, plus genuine 4-page Status Meter `Page 1/4..4/4`)
//! - Authentic Bandai Tamagotchi mechanics (Weight, Toothache without play-reset
//!   exploit, Room Light, Discipline, Care Mistakes, Care Ranks, Sanctuary
//!   Standby Care, and full JSON+SQLite Sanctuary redundancy).

use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::minigame::{ItemKind, MiniGame};
use mochigotchi::pet::{
    AttentionCall, CareRank, EvolutionStage, FoodChoice, LcdIcon, Pet, PetAction, PetMood,
    PetSpecies, Sanctuary, ShellTheme, PET_COOLDOWN_SECS, TRAIN_COOLDOWN_SECS, WARM_COOLDOWN_SECS,
};
use mochigotchi::sprites::{
    get_mood_particles, get_sprite_frames, render_animated_sprite, render_colored_preview,
    render_colored_sprite, render_species_preview, speech_bubble, theme_color,
};
use mochigotchi::storage::{
    has_existing_save, load_dual, load_sanctuary, save_dual, save_sanctuary, StoragePaths,
};
use mochigotchi::tui::{
    draw_ui, App, AppScreen, LayoutTier, OnboardingStep, MIN_TERM_WIDTH,
};
use ratatui::{backend::TestBackend, Terminal};
use tempfile::TempDir;

fn temp_paths() -> (TempDir, StoragePaths) {
    let dir = tempfile::tempdir().expect("tempdir");
    let paths = StoragePaths::new(dir.path().join("save.json"), dir.path().join("save.db"));
    (dir, paths)
}

fn assert_no_zwj_or_fe0f(s: &str, context: &str) {
    assert!(
        !s.contains('\u{200d}'),
        "{context} contains ZWJ U+200D which breaks terminal column alignment: {s:?}"
    );
    assert!(
        !s.contains('\u{fe0f}'),
        "{context} contains U+FE0F variation selector which breaks terminal column alignment: {s:?}"
    );
}

#[test]
fn all_ten_species_render_half_block_pixel_art_and_avoid_zwj_fe0f() {
    let all = PetSpecies::all();
    assert_eq!(all.len(), 10);

    let cozy = all.iter().filter(|s| s.family() == "Cozy Classics").count();
    let mythical = all.iter().filter(|s| s.is_mythical()).count();
    assert_eq!(cozy, 4);
    assert_eq!(mythical, 6);

    for species in all {
        assert_no_zwj_or_fe0f(species.display_name(), "display_name");
        assert_no_zwj_or_fe0f(species.element_badge(), "element_badge");
        assert_no_zwj_or_fe0f(species.favorite_snack(), "favorite_snack");
        assert_eq!(species.star_bar().chars().count(), 5);

        let preview = render_species_preview(*species);
        assert_eq!(preview.len(), 8, "{species:?} 24x16 pixel art is 8 rows");
        assert!(preview
            .iter()
            .any(|l| l.contains('█') || l.contains('▀') || l.contains('▄')));

        let colored_preview = render_colored_preview(*species, EvolutionStage::Adult, 0);
        assert!(!colored_preview.is_empty());

        for stage in EvolutionStage::all() {
            for mood in PetMood::all() {
                let frames = get_sprite_frames(*species, *stage, *mood);
                assert_eq!(frames.len(), 2, "{species:?} {stage:?} {mood:?}");
                assert_ne!(frames[0], frames[1], "{species:?} {stage:?} {mood:?}");
                for f in &frames {
                    assert_no_zwj_or_fe0f(f, "sprite_frame");
                }
            }
        }
    }

    for theme in ShellTheme::all() {
        assert_no_zwj_or_fe0f(theme.display_name(), "theme.display_name");
    }
}

#[test]
fn unhatched_egg_refuses_every_care_action() {
    let mut pet = Pet::new("Mochi");
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.incubation, 0);
    assert_eq!(pet.coins, 0);

    for action in [
        PetAction::Feed,
        PetAction::FeedSnack,
        PetAction::Play,
        PetAction::PetLove,
        PetAction::Train,
        PetAction::ToggleSleep,
        PetAction::ToggleLight,
        PetAction::DisciplineScold,
        PetAction::Clean,
        PetAction::Heal,
    ] {
        let mut clone = pet.clone();
        let message = clone.apply_action(action);
        assert!(message.contains("unhatched"), "{action:?} -> {message}");
        assert_eq!(clone.xp, 0);
        assert_eq!(clone.coins, 0);
        assert_eq!(clone.stage, EvolutionStage::Egg);
        assert_eq!(clone.mood, PetMood::Idle);
    }

    let first = pet.warm_egg();
    assert!(first.contains("Incubation at 20%"), "{first}");
    assert_eq!(pet.incubation, 20);

    let blocked = pet.warm_egg();
    assert!(blocked.contains("absorbing your warmth"), "{blocked}");
    assert_eq!(pet.incubation, 20);
    let remaining = pet.warm_cooldown_remaining(Utc::now());
    assert!(remaining > 0 && remaining <= WARM_COOLDOWN_SECS as u64);
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
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.incubation, 99);
    assert_eq!(pet.hunger, 65);

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

#[test]
fn authentic_tamagotchi_weight_toothache_light_discipline_and_care_rank() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.hunger = 50;
    pet.happiness = 40;
    assert_eq!(pet.weight_grams, 5);

    // Playing does NOT reset snacks_in_a_row (only real Meals, Clean, or Medicine do!).
    pet.feed_choice(FoodChoice::WarmMilkTea);
    assert_eq!(pet.snacks_in_a_row, 1);
    pet.clear_cooldowns();
    pet.play();
    assert_eq!(pet.snacks_in_a_row, 1, "play must not reset sweet snack streak");

    pet.feed_choice(FoodChoice::WarmMilkTea);
    assert_eq!(pet.snacks_in_a_row, 2);
    let msg = pet.feed_choice(FoodChoice::WarmMilkTea);
    assert_eq!(pet.snacks_in_a_row, 3);
    assert!(msg.contains("Toothache"), "{msg}");
    assert!(pet.has_toothache);

    assert!(pet.feed().contains("Toothache"));
    assert!(pet.play().contains("Medicine"));

    let cured = pet.heal();
    assert!(cured.contains("Toothache"), "{cured}");
    assert!(!pet.has_toothache);
    assert_eq!(pet.snacks_in_a_row, 0);

    // Care Rank scales with Discipline & Care Mistakes
    pet.care_mistakes = 0;
    pet.discipline = 80;
    assert_eq!(pet.care_rank(), CareRank::PerfectKeeper);
    pet.care_mistakes = 6;
    assert_eq!(pet.care_rank(), CareRank::MischievousRascal);

    // Unfair scold vs valid scold
    let happy_before = pet.happiness;
    let unfair = pet.discipline_scold();
    assert!(unfair.contains("Unfair scold"), "{unfair}");
    assert!(pet.happiness < happy_before);

    pet.discipline = 50;
    pet.attention_call = AttentionCall::Misbehaving;
    let scolded = pet.discipline_scold();
    assert!(scolded.contains("Discipline rose"), "{scolded}");
    assert_eq!(pet.discipline, 75);
    assert_eq!(pet.attention_call, AttentionCall::None);

    pet.attention_call = AttentionCall::Crying;
    pet.clear_cooldowns();
    let praised = pet.pet_love();
    assert!(praised.contains("Praised & comforted"), "{praised}");
    assert_eq!(pet.attention_call, AttentionCall::None);

    pet.toggle_sleep();
    assert!(pet.is_sleeping);
    assert!(pet.light_on);
    assert_eq!(pet.attention_call, AttentionCall::NeedsCare);
    let light_msg = pet.toggle_light();
    assert!(light_msg.contains("light OFF"), "{light_msg}");
    assert!(!pet.light_on);
}

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
    assert!(pet_rest > 0 && pet_rest <= PET_COOLDOWN_SECS as u64);

    pet.energy = 100;
    pet.hunger = 100;
    pet.clear_cooldowns();
    let trained = pet.train();
    assert!(!trained.contains("catching their breath"), "{trained}");
    let too_soon = pet.train();
    assert!(too_soon.contains("catching their breath"), "{too_soon}");
    let train_rest = pet.train_cooldown_remaining(Utc::now());
    assert!(train_rest > 0 && train_rest <= TRAIN_COOLDOWN_SECS as u64);

    pet.cleanliness = 100;
    pet.poop_count = 0;
    let xp_before = pet.xp;
    let msg = pet.clean();
    assert!(msg.contains("sparkling clean"), "{msg}");
    assert_eq!(pet.xp, xp_before);

    pet.is_sick = false;
    pet.has_toothache = false;
    pet.health = 100;
    let msg = pet.heal();
    assert!(msg.contains("tip-top health"), "{msg}");
    assert_eq!(pet.xp, xp_before);
}

#[test]
fn pantry_meals_and_poop_cleaning_cycle() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.hunger = 40;

    pet.coins = 0;
    let refused = pet.feed_choice(FoodChoice::StarlightDelight);
    assert!(refused.contains("Not enough coins"), "{refused}");

    pet.clear_cooldowns();
    pet.feed();
    assert_eq!(pet.poop_count, 0);

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

#[test]
fn evolution_requires_xp_bond_and_time_together() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.xp = EvolutionStage::Teen.min_xp();
    pet.bond = EvolutionStage::Teen.min_bond();
    pet.age_seconds = 60;

    assert!(!pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Baby);

    pet.age_seconds = EvolutionStage::Teen.min_age_seconds();
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Teen);
    assert!(pet.weight_grams >= EvolutionStage::Teen.base_weight_grams());

    pet.xp = EvolutionStage::Adult.min_xp();
    pet.age_seconds = EvolutionStage::Adult.min_age_seconds();
    pet.bond = 0;
    assert!(!pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Teen);

    pet.bond = 100;
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::Adult);

    pet.xp = EvolutionStage::CyberElder.min_xp();
    pet.age_seconds = EvolutionStage::CyberElder.min_age_seconds();
    assert!(pet.check_evolution());
    assert_eq!(pet.stage, EvolutionStage::CyberElder);
}

#[test]
fn offline_decay_and_sanctuary_standby_care_work() {
    let mut sanctuary = Sanctuary::new_with_pet(Pet::new("Mochi"));
    sanctuary.pets[0].stage = EvolutionStage::Baby;
    sanctuary.pets[0].hunger = 20;

    let mut active = Pet::new_custom("Ignis", PetSpecies::IgnisDragon, ShellTheme::SunsetGold);
    active.stage = EvolutionStage::Baby;
    active.hunger = 80;
    sanctuary.add_pet(active);
    assert_eq!(sanctuary.active_index, 1);

    let now = Utc::now() + Duration::hours(2);
    let (elapsed, report) = sanctuary.apply_offline_decay_all_with_report(now);
    assert!(elapsed >= 7_000);
    assert!(report.is_some());
    assert_eq!(
        sanctuary.pets[0].care_mistakes, 0,
        "inactive companion on standby must not accumulate care mistakes"
    );

    assert_eq!(sanctuary.switch_by_query("Mochi"), Some(0));
    assert_eq!(sanctuary.switch_by_query("2"), Some(1));
    assert_eq!(sanctuary.switch_by_query("nonexistent"), None);
}

#[test]
fn sanctuary_holds_many_companions_and_survives_save_load() {
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
    sanctuary.pets[0].stage = EvolutionStage::Adult;
    sanctuary.pets[0].xp = 1_500;
    sanctuary.active_index = 1;

    save_sanctuary(&sanctuary, &paths).expect("save sanctuary");
    assert!(has_existing_save(&paths));

    let loaded = load_sanctuary(&paths).expect("load sanctuary");
    assert_eq!(loaded.pets.len(), 2);
    assert_eq!(loaded.active_index, 1);
    assert_eq!(loaded.pets[0].stage, EvolutionStage::Adult);

    let mirrored = load_dual(&paths).expect("legacy mirror");
    assert_eq!(mirrored.name, loaded.active_pet().name);

    let (_dir2, legacy_paths) = temp_paths();
    save_dual(&Pet::new("Pebble"), &legacy_paths).expect("legacy save");
    let upgraded = load_sanctuary(&legacy_paths).expect("upgrade legacy save");
    assert_eq!(upgraded.pets.len(), 1);
    assert_eq!(upgraded.active_pet().name, "Pebble");
}

#[test]
fn snack_catcher_arcade_and_lcd_bezel_navigation_work() {
    let mut game = MiniGame::new(30, 10, 40);
    game.spawn_item(5, ItemKind::Snack);
    game.spawn_item(6, ItemKind::Star);
    game.spawn_item(7, ItemKind::Hazard);
    for _ in 0..50 {
        game.step();
        if game.is_finished() {
            break;
        }
    }
    assert!(game.is_finished());

    let (_dir, paths) = temp_paths();
    let mut app = App::new(Pet::new("Mochi"), paths);
    app.pet.stage = EvolutionStage::Adult;
    app.pet.hunger = 50;
    app.pet.energy = 90;

    assert_eq!(LcdIcon::selectable().len(), 7);
    for _ in 0..5 {
        app.handle_key(KeyCode::Right);
    }
    app.handle_key(KeyCode::Enter);
    assert!(app.show_meter);
    app.handle_key(KeyCode::Right);
    assert_eq!(app.meter_page, 1);
    app.handle_key(KeyCode::Esc);
    assert!(!app.show_meter);

    app.handle_key(KeyCode::Char('k'));
    assert!(!app.pet.light_on);
    app.handle_key(KeyCode::Char('k'));
    assert!(app.pet.light_on);
    app.handle_key(KeyCode::Char('d'));
}

#[test]
fn step_by_step_screens_and_four_meter_pages_render_distinctly() {
    let (_dir, paths) = temp_paths();
    let mut app = App::new_onboarding(paths);

    assert_eq!(LayoutTier::from_width(50), LayoutTier::Compact);
    assert_eq!(LayoutTier::from_width(MIN_TERM_WIDTH), LayoutTier::Standard);
    assert_eq!(LayoutTier::from_width(130), LayoutTier::Wide);

    let mut small_term = Terminal::new(TestBackend::new(56, 18)).expect("small term");
    small_term.draw(|f| draw_ui(f, &app)).expect("draw small");
    let small_text: String = small_term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(small_text.contains("Window Size Notice"));

    let mut term = Terminal::new(TestBackend::new(80, 25)).expect("term");
    term.draw(|f| draw_ui(f, &app)).expect("draw step 1");
    let step1_text: String = term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(step1_text.contains("STEP 1 OF 3"));
    assert!(!step1_text.contains("STEP 2 OF 3"));

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);
    term.draw(|f| draw_ui(f, &app)).expect("draw step 2");
    let step2_text: String = term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(step2_text.contains("STEP 2 OF 3"));
    assert!(step2_text.contains("Sunset Gold"));
    assert!(!step2_text.contains("STEP 1 OF 3"));

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
    term.draw(|f| draw_ui(f, &app)).expect("draw step 3");
    let step3_text: String = term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(step3_text.contains("STEP 3 OF 3"));
    assert!(!step3_text.contains("STEP 2 OF 3"));

    app.handle_key(KeyCode::Enter);
    assert_eq!(app.screen, AppScreen::Habitat);

    // Open Meter while Egg
    app.handle_key(KeyCode::Char('i'));
    assert!(app.show_meter);
    term.draw(|f| draw_ui(f, &app)).expect("draw egg meter");
    let egg_meter: String = term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(egg_meter.contains("Egg Incubator Status"));

    // Hatched companion: verify all 4 Meter pages render distinct headings!
    app.pet.stage = EvolutionStage::Adult;
    let expected_headings = [
        "PAGE 1/4",
        "PAGE 2/4",
        "PAGE 3/4",
        "PAGE 4/4",
    ];
    for (p, heading) in expected_headings.iter().enumerate() {
        app.meter_page = p;
        term.draw(|f| draw_ui(f, &app)).expect("draw meter page");
        let page_text: String = term
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(
            page_text.contains(heading),
            "expected {heading} on meter page {p}"
        );
    }

    for mood in PetMood::all() {
        app.pet.mood = *mood;
        assert!(!get_mood_particles(*mood, 0).is_empty());
        assert!(!speech_bubble(&app.pet, 0).is_empty());
        assert!(!render_animated_sprite(&app.pet, 0).is_empty());
        assert!(!render_colored_sprite(&app.pet, 0).is_empty());
    }
    let _ = theme_color(app.pet.theme);
}
