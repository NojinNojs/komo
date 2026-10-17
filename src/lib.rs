//! # Mochigotchi v0.5.0
//!
//! A cozy, collectible pocket-pet simulator for your terminal with
//! reverse-engineered authentic Bandai Tamagotchi mechanics:
//! - 8-Icon LCD Bezel (`🍙 Food`, `💡 Light`, `🎮 Game`, `💉 Meds`, `🚽 Bath`,
//!   `📏 Meter`, `📢 Scold`, `🔔 Call`) with 3-button hardware navigation
//! - 4 Hungry Hearts (`♥♥♥♥`) & 4 Happy Hearts (`♥♥♥♥`)
//! - Meal vs. Sweet Snack weight (`g`) & 3-snack Toothache (`🦷`) system
//! - Room Light switch (`💡`), Attention Calls (`🔔`), Discipline (`📢`), and Care Mistakes
//! - 10 collectible companions in a multi-pet [`Sanctuary`] with TrueColor ASCII art

pub mod minigame;
pub mod pet;
pub mod sprites;
pub mod storage;
pub mod tui;

pub use minigame::{FallingItem, ItemKind, MiniGame};
pub use pet::{
    format_duration_human, format_hearts, AttentionCall, EvolutionStage, FoodChoice, LcdIcon,
    OfflineReport, Pet, PetAction, PetMood, PetSpecies, PetState, Sanctuary, ShellTheme,
    FEED_COOLDOWN_SECS, PET_COOLDOWN_SECS, PLAY_COOLDOWN_SECS, TRAIN_COOLDOWN_SECS,
    WARM_COOLDOWN_SECS,
};
pub use sprites::{
    get_mood_particles, get_sprite_frames, mood_color, mood_dialogue, render_animated_sprite,
    render_colored_preview, render_colored_sprite, render_species_preview, sky_scene,
    speech_bubble, species_palette, stage_color, theme_color,
};
pub use storage::{
    has_existing_save, load_dual, load_json, load_or_create, load_sanctuary, load_sqlite,
    save_dual, save_json, save_sanctuary, save_sqlite, StoragePaths, DEFAULT_JSON_PATH,
    DEFAULT_SQLITE_PATH,
};
pub use tui::{draw_ui, run_tui, run_tui_with_report, App, AppScreen, LayoutTier, OnboardingStep};
