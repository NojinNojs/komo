//! Sensory perception, gaze direction, and living diorama environment for **KOMO**.
//!
//! Gives each companion four grounded senses (`Sight`, `Hearing`, `Feeling`, `Speech`),
//! three gaze directions (`Left`, `Center` facing the player, `Right`), and a living
//! window/floor habitat diorama that reacts to light, weather, and player keystrokes.

use crate::pet::{AttentionCall, EvolutionStage, LcdIcon, Pet, PetMood, PetSpecies};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Which way the companion is currently looking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PetGaze {
    Left,
    #[default]
    Center,
    Right,
}

impl PetGaze {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Left => "Looking Left ◀",
            Self::Center => "Looking at You ●",
            Self::Right => "Looking Right ▶",
        }
    }

    pub fn short(&self) -> &'static str {
        match self {
            Self::Left => "Left ◀",
            Self::Center => "At You ●",
            Self::Right => "Right ▶",
        }
    }
}

/// Real-time snapshot of what the companion perceives right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensorySnapshot {
    pub gaze: PetGaze,
    pub sight: String,
    pub hearing: String,
    pub feeling: String,
}

mod backdrop;
mod environment;
mod posture;
pub use backdrop::*;
pub use environment::*;
pub use posture::*;
