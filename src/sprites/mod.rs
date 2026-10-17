//! Professional 16-Bit 3/4 Side-Profile TrueColor Pixel Art Engine (`▀`, `▄`, `█`)
//! for **Mochigotchi v1.0.0**.
//!
//! Every companion is hand-crafted as a **40×22 pixel bitmap** (40 columns × 11
//! terminal rows of `1:1` square half-block pixels) using authentic **3/4
//! side-profile creature anatomy** (distinct snout/muzzle/beak facing left,
//! arched neck, 4 equine/feline/canine/draconic limbs, sweeping wings, and
//! multi-tone shading) that evolves across 5 distinct silhouettes:
//! 1. `🥚 Egg`: Elemental patterned egg on a warm sanctuary cushion.
//! 2. `🍼 Baby`: Cute big-headed 3/4 chibi hatchling.
//! 3. `🌱 Teen`: Sleek, agile juvenile striker with budding wings/horns/tails.
//! 4. `🌸 Adult`: Fierce full-grown guardian beast.
//! 5. `🌟 CyberElder`: Massive Apex Sovereign boss form with golden armor & aura.
//! 6. `👻 PassedOut`: Floating spirit & pixel memorial when a pet faints/dies.

mod adult_classic;
mod adult_mythic;
mod baby_egg;
mod bubbles_colors;
mod elder_classic;
mod elder_mythic;
mod grid;
mod palette_halfblock;
mod pose_gaze;
mod render_api;
mod teen;
#[cfg(test)]
#[cfg(test)]
mod tests;

use adult_classic::*;
use adult_mythic::*;
use baby_egg::*;
pub use bubbles_colors::*;
use elder_classic::*;
use elder_mythic::*;
pub use grid::*;
pub use palette_halfblock::*;
use pose_gaze::*;
pub use render_api::*;
use teen::*;

use crate::pet::{EvolutionStage, LcdIcon, Pet, PetMood, PetSpecies, ShellTheme};
use crate::senses::{compute_posture_and_gaze, render_window_backdrop, sense_environment, PetGaze};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub const CANVAS_W: usize = 40;
pub const CANVAS_H: usize = 22;
