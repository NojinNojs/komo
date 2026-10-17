//! Core minigame types: modes, item kinds, falling items, and `MiniGame` state.

use ratatui::style::Color;

/// The 3 playable retro minigames in the Arcade Center (`[G]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    SnackCatcher,
    MindReader,
    DashRunner,
}

impl GameMode {
    pub fn all() -> &'static [GameMode] {
        &[Self::SnackCatcher, Self::MindReader, Self::DashRunner]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::SnackCatcher => "1. 🍡 Snack Catcher",
            Self::MindReader => "2. 🔮 Mind-Reader",
            Self::DashRunner => "3. ⚡ Hurdle Dash",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::SnackCatcher => Self::MindReader,
            Self::MindReader => Self::DashRunner,
            Self::DashRunner => Self::SnackCatcher,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Snack,
    Star,
    FeverGem,
    Hazard,
}

impl ItemKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            ItemKind::Snack => "[O]",
            ItemKind::Star => "[★]",
            ItemKind::FeverGem => "<♦>",
            ItemKind::Hazard => "[X]",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ItemKind::Snack => Color::LightGreen,
            ItemKind::Star => Color::LightYellow,
            ItemKind::FeverGem => Color::LightMagenta,
            ItemKind::Hazard => Color::LightRed,
        }
    }

    pub fn is_reward(&self) -> bool {
        !matches!(self, ItemKind::Hazard)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallingItem {
    pub x: u16,
    pub y: u16,
    pub kind: ItemKind,
}

#[derive(Debug, Clone)]
pub struct MiniGame {
    pub mode: GameMode,
    pub width: u16,
    pub height: u16,
    pub player_x: u16,
    pub basket_width: u16,
    pub items: Vec<FallingItem>,
    pub combo: u32,
    pub max_combo: u32,
    pub score: u32,
    pub coins_earned: u32,
    pub xp_earned: u32,
    pub caught_snacks: u32,
    pub caught_stars: u32,
    pub hits_taken: u32,
    pub ticks_remaining: u32,
    // Mode 2: Bandai Left/Right Mind-Reader state
    pub mr_round: u8,
    pub mr_total_rounds: u8,
    pub mr_wins: u8,
    pub mr_history: Vec<bool>,
    pub mr_last_pet_dir: Option<bool>,
    pub mr_last_result: String,
    // Mode 3: Starlight Hurdle Dash state
    pub dash_jump_ticks: u8,
    pub dash_obstacles: Vec<u16>,
    pub dash_coins: Vec<u16>,
    pub dash_scroll: usize,
}
