//! End-to-end integration tests for **Mochigotchi v0.5.1**:
//! - Zero ZWJ (`U+200D`) or Variation Selector (`U+FE0F`) glyph desyncs
//! - Minimum terminal size guard (`90 × 26`) + unclipped tabular Adoption Wizard
//! - Authentic Bandai Tamagotchi reverse-engineered mechanics (8 LCD bezel
//!   icons, 4 Hungry/Happy Hearts, Meal vs. Sweet Snack Weight & Toothache,
//!   Room Light switch, Discipline Scold/Praise, Care Mistakes)
//! - Lifecycle rules, realistic multi-hour/day cooldowns, offline welcome-back
//!   reports, TrueColor sprites, the multi-pet Sanctuary, persistence, and TUI.

use chrono::{Duration, Utc};
use crossterm::event::KeyCode;
use mochigotchi::minigame::{ItemKind, MiniGame};
use mochigotchi::pet::{
    AttentionCall, EvolutionStage, FoodChoice, LcdIcon, Pet, PetAction, PetMood, PetSpecies,
    Sanctuary, ShellTheme, PET_COOLDOWN_SECS, TRAIN_COOLDOWN_SECS, WARM_COOLDOWN_SECS,
};
use mochigotchi::sprites::{
    get_mood_particles, get_sprite_frames, render_animated_sprite, render_colored_preview,
    render_colored_sprite, render_species_preview, speech_bubble, theme_color,
};
use mochigotchi::storage::{
    has_existing_save, load_dual, load_sanctuary, save_dual, save_sanctuary, StoragePaths,
};
use mochigotchi::tui::{
    draw_ui, App, AppScreen, LayoutTier, OnboardingStep, MIN_TERM_HEIGHT, MIN_TERM_WIDTH,
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

// ── 1. Species catalogue, zero ZWJ/FE0F & TrueColor sprites ───────────────

#[test]
fn all_ten_species_are_described_and_free_of_zwj_or_fe0f_glyphs() {
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
        assert!(!species.short_family().is_empty());
        assert_eq!(species.star_bar().chars().count(), 5);

        let preview = render_species_preview(*species);
        assert!(!preview.is_empty(), "{species:?} preview");

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

// ── 2. Egg stage: guarding, realistic incubation, hatching ─────────────────

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

// ── 3. Reverse-engineered Tamagotchi mechanics (Weight, Toothache, Light, Discipline) ─

#[test]
fn authentic_tamagotchi_weight_toothache_light_and_discipline() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Baby;
    pet.hunger = 50;
    pet.happiness = 40;
    assert_eq!(pet.weight_grams, 5);

    for i in 1..=3 {
        let msg = pet.feed_choice(FoodChoice::WarmMilkTea);
        assert_eq!(pet.snacks_in_a_row, i);
        assert_eq!(pet.weight_grams, 5 + (i as u16) * 2);
        if i == 3 {
            assert!(msg.contains("Toothache"), "{msg}");
            assert!(pet.has_toothache);
        }
    }

    assert!(pet.feed().contains("Toothache"));
    assert!(pet.play().contains("Medicine"));

    let cured = pet.heal();
    assert!(cured.contains("Toothache"), "{cured}");
    assert!(!pet.has_toothache);
    assert_eq!(pet.snacks_in_a_row, 0);

    let w_before = pet.weight_grams;
    pet.clear_cooldowns();
    pet.play();
    assert_eq!(pet.weight_grams, w_before - 2);

    let happy_before = pet.happiness;
    let unfair = pet.discipline_scold();
    assert!(unfair.contains("Unfair scold"), "{unfair}");
    assert!(pet.happiness < happy_before);

    pet.attention_call = AttentionCall::Misbehaving;
    let disc_before = pet.discipline;
    let scolded = pet.discipline_scold();
    assert!(scolded.contains("Discipline rose"), "{scolded}");
    assert_eq!(pet.discipline, disc_before + 25);
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

// ── 4. Anti-spam care guards & Pantry choices ─────────────────────────────

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

// ── 5. Evolution needs real multi-hour age, bond and XP ───────────────────

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
    assert!(pet.coins > before.3);
    let rep = report.expect("should generate welcome back report");
    assert_eq!(rep.pet_name, "Mochi");
    assert!(!rep.events.is_empty());
}

// ── 6. Multi-pet Sanctuary & Persistence ──────────────────────────────────

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

// ── 7. Mini-game, Minimum Terminal Size Guard & TUI Bezel Navigation ──────

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
fn minimum_terminal_size_guard_and_full_ui_render_cleanly() {
    let (_dir, paths) = temp_paths();
    let mut app = App::new_onboarding(paths);

    assert_eq!(LayoutTier::from_width(70), LayoutTier::Compact);
    assert_eq!(LayoutTier::from_width(MIN_TERM_WIDTH), LayoutTier::Standard);
    assert_eq!(LayoutTier::from_width(130), LayoutTier::Wide);

    // 1. Below minimum size (< 90x26): renders the Window Size Notice card cleanly.
    let mut small_term = Terminal::new(TestBackend::new(72, 22)).expect("small term");
    small_term.draw(|f| draw_ui(f, &app)).expect("draw small");
    let small_text: String = small_term
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(
        small_text.contains("Window Size Notice"),
        "expected resize notice on 72x22"
    );

    // 2. At and above minimum size (>= 90x26): renders the full UI with all 5 shell themes visible!
    app.onboarding_step = OnboardingStep::SelectTheme;
    for (width, height) in [(MIN_TERM_WIDTH, MIN_TERM_HEIGHT), (104, 32), (132, 40)] {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("terminal");
        terminal.draw(|f| draw_ui(f, &app)).expect("draw");
        let full_text: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(
            full_text.contains("Sunset Gold"),
            "all 5 shell themes must fit without clipping at {width}x{height}"
        );
        assert!(
            full_text.contains("★★★★★"),
            "5-star rarity column must fit without clipping at {width}x{height}"
        );

        for screen in [
            AppScreen::Habitat,
            AppScreen::SanctuaryRoster,
            AppScreen::MiniGame,
            AppScreen::Onboarding,
        ] {
            app.screen = screen;
            terminal.draw(|f| draw_ui(f, &app)).expect("draw screen");
        }
        app.screen = AppScreen::Onboarding;
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
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.screen, AppScreen::Habitat);
    assert_eq!(app.pet.species, PetSpecies::IgnisDragon);
}
