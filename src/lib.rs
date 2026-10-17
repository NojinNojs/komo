pub mod minigame;
pub mod pet;
pub mod senses;
pub mod sound;
pub mod sprites;
pub mod storage;
pub mod tui;

pub use minigame::{FallingItem, GameMode, ItemKind, MiniGame};
pub use pet::{
    format_duration_human, format_hearts, AttentionCall, CareRank, DialoguePrompt, EvolutionStage,
    FoodChoice, LcdIcon, MemorialRecord, OfflineReport, Pet, PetAction, PetMood, PetSpecies,
    PetState, Sanctuary, ShellTheme,
};
pub use senses::{
    compute_posture_and_gaze, render_habitat_floor, render_window_backdrop, sense_environment,
    PetGaze, SensorySnapshot,
};
pub use sound::{synthesize_wav, SoundEffect, SoundEngine};
pub use sprites::{
    build_grid_with_gaze, get_mood_particles, get_sprite_frames, mood_color, mood_dialogue,
    render_animated_sprite, render_colored_preview, render_colored_sprite, render_species_preview,
    sky_scene, species_palette, speech_bubble, stage_color, theme_color, thought_bubble, CANVAS_H,
    CANVAS_W,
};
pub use storage::{
    load_pet, load_sanctuary, save_exists, save_json, save_pet, save_sanctuary, save_sqlite,
    StoragePaths,
};
pub use tui::{
    draw_ui, run_tui, run_tui_with_report, App, AppScreen, LayoutTier, OnboardingStep,
    MIN_TERM_HEIGHT, MIN_TERM_WIDTH,
};
