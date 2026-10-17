//! # Mochigotchi
//!
//! A cozy, collectible pocket-pet simulator for your terminal.
//! Ten species, a multi-pet Sanctuary, realistic incubation and care
//! cooldowns, and dual JSON + SQLite saves.

pub mod minigame;
pub mod pet;
pub mod sprites;
pub mod storage;
pub mod tui;

pub use minigame::{FallingItem, ItemKind, MiniGame};
pub use pet::{
    EvolutionStage, Pet, PetAction, PetMood, PetSpecies, PetState, Sanctuary, ShellTheme,
    PET_COOLDOWN_SECS, TRAIN_COOLDOWN_SECS, WARM_COOLDOWN_SECS,
};
pub use sprites::{
    get_mood_particles, get_sprite_frames, mood_color, mood_dialogue, render_animated_sprite,
    render_species_preview, sky_scene, speech_bubble, stage_color, theme_color,
};
pub use storage::{
    has_existing_save, load_dual, load_json, load_or_create, load_sanctuary, load_sqlite,
    save_dual, save_json, save_sanctuary, save_sqlite, StoragePaths, DEFAULT_JSON_PATH,
    DEFAULT_SQLITE_PATH,
};
pub use tui::{draw_ui, run_tui, App, AppScreen, LayoutTier, OnboardingStep};
