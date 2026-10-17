pub mod git_hook;
pub mod minigame;
pub mod pet;
pub mod sprites;
pub mod storage;
pub mod tui;

pub use git_hook::{
    generate_post_commit_hook_script, install_post_commit_hook, sync_latest_commits,
};
pub use minigame::{FallingItem, ItemKind, MiniGame};
pub use pet::{EvolutionStage, Pet, PetAction, PetMood, PetState};
pub use sprites::{
    get_mood_particles, get_sprite_frames, mood_color, render_animated_sprite, speech_bubble,
    stage_color,
};
pub use storage::{
    load_dual, load_json, load_or_create, load_sqlite, save_dual, save_json, save_sqlite,
    StoragePaths, DEFAULT_JSON_PATH, DEFAULT_SQLITE_PATH,
};
pub use tui::{draw_ui, run_tui, App};
