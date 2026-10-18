use super::*;

pub(super) const TICK_MS: u64 = 250;
pub(super) const DECAY_EVERY_TICKS: usize = 96;
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
    pub wardrobe_open: bool,
    pub wardrobe_tab: u8,
    pub wardrobe_coat_idx: usize,
    pub wardrobe_acc_idx: usize,
    pub photo_booth_open: bool,
    pub photo_ratio: ShareRatio,
    pub photo_status: Option<String>,
    pub hosting_selected_idx: u8,
    pub show_meter: bool,
    pub meter_page: usize,
    pub show_help: bool,
    pub show_changelog: bool,
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

impl App {
    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);
        if self.recent_key_ticks > 0 {
            self.recent_key_ticks -= 1;
            if self.recent_key_ticks == 0 {
                self.key_burst = 0;
            }
        }
        if self.screen == AppScreen::Onboarding
            && self.onboarding_step == OnboardingStep::SelectSpecies
            && self.preview_auto_morph
            && self.tick_count.is_multiple_of(AUTO_MORPH_TICKS)
        {
            self.preview_stage_idx = (self.preview_stage_idx + 1) % EvolutionStage::all().len();
        }
        if self.screen == AppScreen::MiniGame {
            let finished = {
                let Some(game) = self.minigame.as_mut() else {
                    self.screen = AppScreen::Habitat;
                    return;
                };
                game.step();
                game.is_finished()
            };
            if finished {
                self.finish_minigame();
            }
            return;
        }
        if self.screen != AppScreen::Onboarding && self.tick_count.is_multiple_of(DECAY_EVERY_TICKS)
        {
            let call_before = self.pet.attention_call;
            let alive_before = self.pet.is_alive;
            self.sanctuary.sync_active(&self.pet);
            self.sanctuary.tick_decay_all();
            self.pet = self.sanctuary.active_pet().clone();
            if alive_before && !self.pet.is_alive {
                self.sound.play(SoundEffect::ErrorBuzz);
            } else if !call_before.is_active() && self.pet.attention_call.is_active() {
                self.sound.play(SoundEffect::AttentionBeep);
            }
            if self.tick_count.is_multiple_of(DECAY_EVERY_TICKS * 4) {
                self.save_now();
            }
        }
    }
}
