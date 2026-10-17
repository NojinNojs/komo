use super::*;

pub(super) const TICK_MS: u64 = 250;
pub(super) const DECAY_EVERY_TICKS: usize = 24;
pub(super) const AUTO_MORPH_TICKS: usize = 18;

pub const MIN_TERM_WIDTH: u16 = 68;
pub const MIN_TERM_HEIGHT: u16 = 24;

/// Spacious retro handheld frame dimensions that use vertical terminal space gracefully!
pub(super) const HANDHELD_MAX_W: u16 = 82;
pub(super) const HANDHELD_MAX_H: u16 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutTier {
    Compact,
    Standard,
    Wide,
}

impl LayoutTier {
    pub fn from_width(width: u16) -> Self {
        if width < MIN_TERM_WIDTH {
            Self::Compact
        } else if width < 100 {
            Self::Standard
        } else {
            Self::Wide
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Onboarding,
    Habitat,
    SanctuaryRoster,
    MiniGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingStep {
    SelectSpecies,
    SelectTheme,
    EnterName,
}

pub struct App {
    pub pet: Pet,
    pub sanctuary: Sanctuary,
    pub paths: StoragePaths,
    pub sound: SoundEngine,
    pub screen: AppScreen,
    pub onboarding_step: OnboardingStep,
    pub selected_species_idx: usize,
    pub selected_theme_idx: usize,
    pub preview_stage_idx: usize,
    pub preview_auto_morph: bool,
    pub roster_selected_idx: usize,
    pub selected_icon_idx: usize,
    pub pantry_open: bool,
    pub pantry_selected_idx: usize,
    pub talk_open: bool,
    pub talk_selected_idx: u8,
    pub show_meter: bool,
    pub meter_page: usize,
    pub show_help: bool,
    pub offline_report: Option<OfflineReport>,
    pub onboarding_name_input: String,
    pub onboarding_caretaker_input: String,
    pub onboarding_edit_caretaker: bool,
    pub adopting_new: bool,
    pub tick_count: usize,
    pub recent_key_ticks: u8,
    pub key_burst: u8,
    pub should_quit: bool,
    pub minigame: Option<MiniGame>,
    pub rename_buffer: Option<String>,
    pub caretaker_buffer: Option<String>,
    pub last_storage_status: String,
}
