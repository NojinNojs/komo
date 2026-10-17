use chrono::{Duration, Utc};
use rustagotchi::{
    generate_post_commit_hook_script, get_mood_particles, get_sprite_frames,
    install_post_commit_hook, load_dual, load_json, load_or_create, load_sqlite, mood_color,
    render_animated_sprite, save_dual, save_json, save_sqlite, speech_bubble, stage_color,
    EvolutionStage, ItemKind, MiniGame, Pet, PetMood, StoragePaths,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("rustagotchi_{}_{}_{}", prefix, std::process::id(), nanos));
    fs::create_dir_all(&dir).expect("should create temp dir");
    dir
}

#[test]
fn test_evolution_stage_progression() {
    assert_eq!(EvolutionStage::all().len(), 5);
    assert_eq!(EvolutionStage::from_xp(0), EvolutionStage::Egg);
    assert_eq!(EvolutionStage::from_xp(49), EvolutionStage::Egg);
    assert_eq!(EvolutionStage::from_xp(50), EvolutionStage::Baby);
    assert_eq!(EvolutionStage::from_xp(149), EvolutionStage::Baby);
    assert_eq!(EvolutionStage::from_xp(150), EvolutionStage::Teen);
    assert_eq!(EvolutionStage::from_xp(349), EvolutionStage::Teen);
    assert_eq!(EvolutionStage::from_xp(350), EvolutionStage::Adult);
    assert_eq!(EvolutionStage::from_xp(699), EvolutionStage::Adult);
    assert_eq!(EvolutionStage::from_xp(700), EvolutionStage::CyberElder);
    assert_eq!(EvolutionStage::from_xp(5000), EvolutionStage::CyberElder);

    assert_eq!(EvolutionStage::Egg.next_stage(), Some(EvolutionStage::Baby));
    assert_eq!(EvolutionStage::Baby.next_stage(), Some(EvolutionStage::Teen));
    assert_eq!(EvolutionStage::Teen.next_stage(), Some(EvolutionStage::Adult));
    assert_eq!(
        EvolutionStage::Adult.next_stage(),
        Some(EvolutionStage::CyberElder)
    );
    assert_eq!(EvolutionStage::CyberElder.next_stage(), None);

    let mut pet = Pet::new("Ferris");
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert_eq!(pet.level, 1);
    assert!((pet.xp_progress_ratio() - 0.0).abs() < f64::EPSILON);

    // 25 XP -> still Egg
    assert!(!pet.add_xp(25));
    assert_eq!(pet.stage, EvolutionStage::Egg);
    assert!(pet.xp_progress_ratio() > 0.0 && pet.xp_progress_ratio() < 1.0);

    // +25 XP (total 50) -> evolves to Baby
    assert!(pet.add_xp(25));
    assert_eq!(pet.stage, EvolutionStage::Baby);
    assert_eq!(pet.level, 2);

    // +100 XP (total 150) -> evolves to Teen
    assert!(pet.add_xp(100));
    assert_eq!(pet.stage, EvolutionStage::Teen);

    // +200 XP (total 350) -> evolves to Adult
    assert!(pet.add_xp(200));
    assert_eq!(pet.stage, EvolutionStage::Adult);

    // +350 XP (total 700) -> evolves to CyberElder
    assert!(pet.add_xp(350));
    assert_eq!(pet.stage, EvolutionStage::CyberElder);
    assert!((pet.xp_progress_ratio() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_offline_decay_normal_sleeping_and_sickness() {
    // 1. Normal decay over 1 hour while awake
    let mut pet = Pet::new("Ferris");
    let start = pet.last_updated;
    let initial_hunger = pet.hunger;
    let initial_happiness = pet.happiness;
    let initial_cleanliness = pet.cleanliness;
    let initial_energy = pet.energy;

    let elapsed = pet.apply_offline_decay(start + Duration::hours(1));
    assert_eq!(elapsed, 3600);
    assert!(pet.age_seconds >= 3600);
    assert!(pet.hunger < initial_hunger);
    assert!(pet.happiness < initial_happiness);
    assert!(pet.cleanliness < initial_cleanliness);
    assert!(pet.energy < initial_energy);

    // 2. Sleeping energy recovery during offline decay
    let mut sleeping_pet = Pet::new("Snoozer");
    sleeping_pet.energy = 30;
    sleeping_pet.is_sleeping = true;
    sleeping_pet.mood = PetMood::Sleeping;
    let sleep_start = sleeping_pet.last_updated;

    let sleep_elapsed = sleeping_pet.apply_offline_decay(sleep_start + Duration::hours(1));
    assert_eq!(sleep_elapsed, 3600);
    assert!(sleeping_pet.energy > 30);

    // 3. Critical sickness trigger when hunger or cleanliness drops <= 15
    let mut neglected_pet = Pet::new("Neglected");
    neglected_pet.hunger = 18;
    neglected_pet.cleanliness = 18;
    let neglect_start = neglected_pet.last_updated;

    neglected_pet.apply_offline_decay(neglect_start + Duration::hours(2));
    assert!(neglected_pet.hunger <= 15 || neglected_pet.cleanliness <= 15);
    assert!(neglected_pet.is_sick);
    assert!(neglected_pet.health < 100);
}

#[test]
fn test_pet_actions_and_git_commit_deduplication() {
    let mut pet = Pet::new("BorrowChecker");
    pet.hunger = 60;
    pet.happiness = 60;
    pet.cleanliness = 60;
    pet.energy = 70;

    // Feed
    let feed_msg = pet.feed();
    assert!(!feed_msg.is_empty());
    assert!(pet.hunger > 60);

    // Play
    let xp_before_play = pet.xp;
    let coins_before_play = pet.coins;
    let play_msg = pet.play();
    assert!(!play_msg.is_empty());
    assert!(pet.xp > xp_before_play);
    assert!(pet.coins > coins_before_play);

    // Toggle sleep on and off
    let sleep_on_msg = pet.toggle_sleep();
    assert!(!sleep_on_msg.is_empty());
    assert!(pet.is_sleeping);
    assert_eq!(pet.mood, PetMood::Sleeping);

    let sleep_off_msg = pet.toggle_sleep();
    assert!(!sleep_off_msg.is_empty());
    assert!(!pet.is_sleeping);

    // Clean
    let clean_msg = pet.clean();
    assert!(!clean_msg.is_empty());
    assert_eq!(pet.cleanliness, 100);

    // Code session
    let xp_before_code = pet.xp;
    let code_msg = pet.code_session();
    assert!(!code_msg.is_empty());
    assert!(pet.xp >= xp_before_code + 25);
    assert_eq!(pet.mood, PetMood::Coding);

    // Heal
    pet.is_sick = true;
    pet.health = 50;
    let heal_msg = pet.heal();
    assert!(!heal_msg.is_empty());
    assert!(!pet.is_sick);
    assert_eq!(pet.health, 100);

    // Git commit reward & deduplication
    let xp_before_commit = pet.xp;
    let coins_before_commit = pet.coins;
    assert!(!pet.reward_git_commit("   "));
    assert!(pet.reward_git_commit("a1b2c3d feat: implement borrow checker pet"));
    assert_eq!(pet.commits_fed, 1);
    assert!(pet.xp >= xp_before_commit + 40);
    assert!(pet.coins >= coins_before_commit + 20);

    // Feeding the exact same commit again must be rejected
    assert!(!pet.reward_git_commit("a1b2c3d feat: implement borrow checker pet"));
    assert_eq!(pet.commits_fed, 1);

    // Status summary and rename
    pet.rename("CrabMaster");
    assert_eq!(pet.name, "CrabMaster");
    let summary = pet.status_summary();
    assert!(summary.contains("CrabMaster"));
}

#[test]
fn test_sprites_and_animation_cycling() {
    let pet = Pet::new("SpriteTester");

    for &stage in EvolutionStage::all() {
        let _sc = stage_color(stage);
        assert!(!stage.name().is_empty());
        let _bonus = stage.max_stat_bonus();

        for &mood in PetMood::all() {
            let _mc = mood_color(mood);
            assert!(!mood.label().is_empty());

            let frames = get_sprite_frames(stage, mood);
            assert!(
                frames.len() >= 2,
                "Expected at least 2 sprite frames for {:?} / {:?}",
                stage,
                mood
            );

            for tick in 0..4 {
                let rendered = render_animated_sprite(stage, mood, tick);
                assert!(!rendered.is_empty());
                let particles = get_mood_particles(mood, tick);
                assert!(!particles.is_empty());
            }
        }
    }

    for tick in 0..6 {
        let bubble = speech_bubble(&pet, tick);
        assert!(!bubble.is_empty());
    }
}

#[test]
fn test_minigame_state_transitions_and_rewards() {
    let mut game = MiniGame::new(24, 10);
    assert!(game.active);
    assert!(!game.is_finished());

    let start_x = game.player_x;
    game.move_left();
    assert!(game.player_x <= start_x);
    game.move_right();
    assert!(game.player_x >= start_x);

    // Clamp left boundary
    for _ in 0..50 {
        game.move_left();
    }
    assert_eq!(game.player_x, 0);

    // Clamp right boundary
    for _ in 0..100 {
        game.move_right();
    }
    assert!(game.player_x < game.width);

    // Spawn beneficial item right above player and step
    let target_x = game.player_x;
    game.spawn_item(target_x, ItemKind::Crate);
    assert!(!game.items.is_empty());

    let lines = game.render_lines();
    assert!(!lines.is_empty());

    for _ in 0..15 {
        game.step();
    }

    let mut pet = Pet::new("GamerFerris");
    let initial_coins = pet.coins;
    let initial_xp = pet.xp;
    let reward_summary = pet.apply_minigame_reward(120, 30, 45);
    assert!(!reward_summary.is_empty());
    assert_eq!(pet.coins, initial_coins + 30);
    assert_eq!(pet.xp, initial_xp + 45);
}

#[test]
fn test_dual_persistence_and_git_hook_installation() {
    let temp_dir = unique_temp_dir("persistence");
    let json_path = temp_dir.join("pet_save.json");
    let sqlite_path = temp_dir.join("pet_save.db");
    let _paths = StoragePaths::new(&json_path, &sqlite_path);

    let mut original = Pet::new("PersistCrab");
    original.add_xp(160);
    original.reward_git_commit("commit-001 initial");
    original.coins = 77;
    original.last_updated = Utc::now();

    // 1. JSON save/load roundtrip
    save_json(&original, &json_path).expect("save_json should succeed");
    let from_json = load_json(&json_path).expect("load_json should succeed");
    assert_eq!(from_json.name, "PersistCrab");
    assert_eq!(from_json.stage, EvolutionStage::Teen);
    assert_eq!(from_json.coins, 77);
    assert_eq!(from_json.commits_fed, 1);

    // 2. SQLite save/load roundtrip
    save_sqlite(&original, &sqlite_path).expect("save_sqlite should succeed");
    let from_sqlite = load_sqlite(&sqlite_path).expect("load_sqlite should succeed");
    assert_eq!(from_sqlite.name, "PersistCrab");
    assert_eq!(from_sqlite.stage, EvolutionStage::Teen);
    assert_eq!(from_sqlite.coins, 77);
    assert_eq!(from_sqlite.commits_fed, 1);

    // 3. Dual save/load & load_or_create
    let dual_json = temp_dir.join("dual.json");
    let dual_db = temp_dir.join("dual.db");
    let dual_paths = StoragePaths::new(&dual_json, &dual_db);

    let created = load_or_create(&dual_paths, "NewbornCrab");
    assert_eq!(created.name, "NewbornCrab");

    save_dual(&original, &dual_paths).expect("save_dual should succeed");
    let loaded_dual = load_dual(&dual_paths).expect("load_dual should succeed");
    assert_eq!(loaded_dual.name, "PersistCrab");
    assert_eq!(loaded_dual.xp, original.xp);

    // 4. Git post-commit hook generation & installation
    let script = generate_post_commit_hook_script(Some("/usr/local/bin/rustagotchi"));
    assert!(script.contains("--git-commit"));

    let fake_repo = temp_dir.join("repo");
    fs::create_dir_all(fake_repo.join(".git").join("hooks")).expect("create .git/hooks");
    let hook_path = install_post_commit_hook(&fake_repo, Some("rustagotchi"))
        .expect("install_post_commit_hook should succeed");
    assert!(hook_path.exists());
    let hook_contents = fs::read_to_string(&hook_path).expect("read installed hook");
    assert!(hook_contents.contains("--git-commit"));

    let _ = fs::remove_dir_all(&temp_dir);
}
