//! Spacious, Balanced Retro Handheld LCD Interface for **Mochigotchi v1.0.0**:
//! - **Structured 3-Zone Vertical Layouts** inside every LCD screen (Top Header
//!   with breathing spacer, Center 40×22 3/4 Side-Profile Pixel Art Stage, and
//!   Bottom Vitals/Lore & Controls anchored cleanly at the bottom of the LCD!).
//! - **15 Rich Emotions (`PetMood`)** displayed live with facial expressions &
//!   autonomous strolling across the habitat floor.
//! - **Direct First-Person Pet Reactions**: when you feed, cuddle, scold, or spam
//!   a button, your companion talks back to you immediately in the `💬` & `💭`
//!   bubbles!
//! - **Realistic Starvation, Sickness, Spirit Memorial (`👻`) & Phoenix Revival (`[H]`)**.

use std::io;
use std::time::{Duration, Instant};

use chrono::Utc;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::minigame::{GameMode, MiniGame};
use crate::pet::{
    format_duration_human, format_hearts, AttentionCall, EvolutionStage, FoodChoice, LcdIcon,
    OfflineReport, Pet, PetSpecies, Sanctuary, ShellTheme,
};
use crate::sound::{SoundEffect, SoundEngine};
use crate::sprites::{
    mood_color, render_colored_preview, render_colored_sprite, sky_scene, speech_bubble,
    species_palette, stage_color, theme_color, thought_bubble,
};
use crate::storage::{save_sanctuary, StoragePaths};

const TICK_MS: u64 = 250;
const DECAY_EVERY_TICKS: usize = 24;
const AUTO_MORPH_TICKS: usize = 18;

pub const MIN_TERM_WIDTH: u16 = 68;
pub const MIN_TERM_HEIGHT: u16 = 24;

/// Spacious retro handheld frame dimensions that use vertical terminal space gracefully!
const HANDHELD_MAX_W: u16 = 82;
const HANDHELD_MAX_H: u16 = 30;

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
    pub adopting_new: bool,
    pub tick_count: usize,
    pub should_quit: bool,
    pub minigame: Option<MiniGame>,
    pub rename_buffer: Option<String>,
    pub last_storage_status: String,
}

impl App {
    pub fn new(pet: Pet, paths: StoragePaths) -> Self {
        Self::new_with_sanctuary(Sanctuary::new_with_pet(pet), paths, false)
    }

    pub fn new_with_sanctuary(
        sanctuary: Sanctuary,
        paths: StoragePaths,
        start_onboarding: bool,
    ) -> Self {
        let pet = sanctuary.active_pet().clone();
        let selected_species_idx = PetSpecies::all()
            .iter()
            .position(|s| *s == pet.species)
            .unwrap_or(0);
        let selected_theme_idx = ShellTheme::all()
            .iter()
            .position(|t| *t == pet.theme)
            .unwrap_or(0);
        Self {
            pet,
            sanctuary,
            paths,
            sound: SoundEngine::default(),
            screen: if start_onboarding {
                AppScreen::Onboarding
            } else {
                AppScreen::Habitat
            },
            onboarding_step: OnboardingStep::SelectSpecies,
            selected_species_idx,
            selected_theme_idx,
            preview_stage_idx: 4,
            preview_auto_morph: true,
            roster_selected_idx: 0,
            selected_icon_idx: 0,
            pantry_open: false,
            pantry_selected_idx: 0,
            talk_open: false,
            talk_selected_idx: 1,
            show_meter: false,
            meter_page: 0,
            show_help: false,
            offline_report: None,
            onboarding_name_input: String::new(),
            adopting_new: false,
            tick_count: 0,
            should_quit: false,
            minigame: None,
            rename_buffer: None,
            last_storage_status: "💾 saved".to_string(),
        }
    }

    pub fn new_onboarding(paths: StoragePaths) -> Self {
        let mut app =
            Self::new_with_sanctuary(Sanctuary::new_with_pet(Pet::new("Mochi")), paths, true);
        app.preview_stage_idx = 0;
        app.onboarding_name_input = "Mochi".to_string();
        app
    }

    pub fn save_now(&mut self) {
        self.sanctuary.sync_active(&self.pet);
        match save_sanctuary(&self.sanctuary, &self.paths) {
            Ok(()) => self.last_storage_status = "💾 saved".to_string(),
            Err(err) => self.last_storage_status = format!("save err: {err}"),
        }
    }

    fn play_feedback_for_message(&mut self, msg: &str, evolved_or_hatched: bool) {
        if evolved_or_hatched
            || msg.contains("CRACK!")
            || msg.contains("Evolution!")
            || msg.contains("PHOENIX REVIVAL")
        {
            self.sound.play(SoundEffect::HatchFanfare);
        } else if msg.contains("Toothache")
            || msg.contains("⏳")
            || msg.contains("unhatched")
            || msg.contains("asleep")
            || msg.contains("sleeping")
            || msg.contains("completely full")
            || msg.contains("maxed out")
            || msg.contains("Not enough coins")
            || msg.contains("Unfair scold")
            || msg.contains("too tired")
            || msg.contains("too hungry")
            || msg.contains("passed out")
        {
            self.sound.play(SoundEffect::ErrorBuzz);
        } else if msg.contains("Fed ") {
            self.sound.play(SoundEffect::EatChime);
        } else if msg.contains("warm ") {
            self.sound.play(SoundEffect::WarmEgg);
        } else if msg.contains("Played ") || msg.contains("training") {
            self.sound.play(SoundEffect::PlayWin);
        } else {
            self.sound.play(SoundEffect::BeepConfirm);
        }
    }

    fn switch_to_roster_pet(&mut self) {
        self.sanctuary.sync_active(&self.pet);
        if self.sanctuary.switch_pet(self.roster_selected_idx) {
            self.pet = self.sanctuary.active_pet().clone();
            self.sound.play(SoundEffect::BeepConfirm);
            self.save_now();
        }
    }

    fn cycle_pet(&mut self, forward: bool) {
        self.sanctuary.sync_active(&self.pet);
        if forward {
            self.sanctuary.next_pet();
        } else {
            self.sanctuary.prev_pet();
        }
        self.pet = self.sanctuary.active_pet().clone();
        self.roster_selected_idx = self.sanctuary.active_index;
        self.sound.play(SoundEffect::BeepMove);
        self.save_now();
    }

    fn start_adopting(&mut self) {
        self.adopting_new = true;
        self.pantry_open = false;
        self.talk_open = false;
        self.show_meter = false;
        self.show_help = false;
        self.screen = AppScreen::Onboarding;
        self.onboarding_step = OnboardingStep::SelectSpecies;
        self.selected_species_idx = 0;
        self.selected_theme_idx = 0;
        self.preview_stage_idx = 0;
        self.preview_auto_morph = true;
        self.onboarding_name_input = PetSpecies::all()[0].default_name().to_string();
        self.sound.play(SoundEffect::BeepConfirm);
    }

    fn launch_minigame(&mut self, mode: GameMode) {
        let refusal = if !self.pet.is_alive {
            Some(format!(
                "👻 {} has passed out! Press [H] to revive them first!",
                self.pet.name
            ))
        } else if self.pet.stage == EvolutionStage::Egg {
            Some("🥚 The egg can't play yet — warm it first!".to_string())
        } else if self.pet.is_sleeping {
            Some(format!("💤 {} is asleep — wake them first!", self.pet.name))
        } else if self.pet.has_toothache || self.pet.is_sick {
            Some(format!(
                "🤒 {} needs Medicine [H] before playing games!",
                self.pet.name
            ))
        } else if self.pet.energy < 15 {
            Some(format!("⚡ {} is too tired for the arcade.", self.pet.name))
        } else {
            None
        };

        match refusal {
            Some(msg) => {
                self.sound.play(SoundEffect::ErrorBuzz);
                self.pet.log_action(msg);
            }
            None => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.minigame = Some(MiniGame::new_with_mode(mode, 44, 8, 120));
                self.screen = AppScreen::MiniGame;
            }
        }
    }

    fn activate_selected_lcd_icon(&mut self) {
        let icon = LcdIcon::selectable()[self.selected_icon_idx % LcdIcon::selectable().len()];
        match icon {
            LcdIcon::Food => {
                if self.pet.stage == EvolutionStage::Egg {
                    let stage_before = self.pet.stage;
                    let msg = self.pet.warm_egg();
                    let hatched = self.pet.stage != stage_before;
                    self.play_feedback_for_message(&msg, hatched);
                    self.save_now();
                } else {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.pantry_open = true;
                }
            }
            LcdIcon::Lights => {
                let msg = self.pet.toggle_light();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Play => {
                self.launch_minigame(GameMode::MindReader);
            }
            LcdIcon::Medicine => {
                let msg = self.pet.heal();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Bath => {
                let msg = self.pet.clean();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Meter => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_meter = true;
                self.meter_page = 0;
            }
            LcdIcon::Discipline => {
                let msg = if self.pet.attention_call == AttentionCall::Crying {
                    self.pet.pet_love()
                } else {
                    self.pet.discipline_scold()
                };
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Talk => {
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = true;
                self.talk_selected_idx = 1;
            }
            LcdIcon::Attention => {}
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if self.rename_buffer.is_none()
            && !(self.screen == AppScreen::Onboarding
                && self.onboarding_step == OnboardingStep::EnterName)
            && matches!(code, KeyCode::Char('u') | KeyCode::Char('U'))
        {
            self.sound.toggle();
            return;
        }

        if self.offline_report.is_some() {
            if matches!(
                code,
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') | KeyCode::Char('q')
            ) {
                self.sound.play(SoundEffect::BeepConfirm);
                self.offline_report = None;
            }
            return;
        }

        if self.show_help {
            if matches!(
                code,
                KeyCode::Esc
                    | KeyCode::Enter
                    | KeyCode::Char('?')
                    | KeyCode::Char('q')
                    | KeyCode::Char(' ')
            ) {
                self.sound.play(SoundEffect::BeepMove);
                self.show_help = false;
            }
            return;
        }

        if self.show_meter {
            match code {
                KeyCode::Right
                | KeyCode::Down
                | KeyCode::Char(' ')
                | KeyCode::Enter
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.meter_page = (self.meter_page + 1) % 4;
                }
                KeyCode::Left | KeyCode::Up => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.meter_page = (self.meter_page + 3) % 4;
                }
                KeyCode::Esc | KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('q') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.show_meter = false;
                }
                _ => {}
            }
            return;
        }

        if self.rename_buffer.is_some() {
            self.handle_rename_key(code);
            return;
        }

        if self.talk_open {
            self.handle_talk_key(code);
            return;
        }

        if self.pantry_open {
            self.handle_pantry_key(code);
            return;
        }

        match self.screen {
            AppScreen::Onboarding => self.handle_onboarding_key(code),
            AppScreen::SanctuaryRoster => self.handle_roster_key(code),
            AppScreen::MiniGame => self.handle_minigame_key(code),
            AppScreen::Habitat => self.handle_habitat_key(code),
        }
    }

    fn handle_talk_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.talk_selected_idx = if self.talk_selected_idx == 1 { 2 } else { 1 };
            }
            KeyCode::Char('1') => {
                let _ = self.pet.answer_dialogue(1);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Char('2') => {
                let _ = self.pet.answer_dialogue(2);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('y') | KeyCode::Char('Y') => {
                let choice = self.talk_selected_idx;
                let _ = self.pet.answer_dialogue(choice);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.talk_open = false;
            }
            _ => {}
        }
    }

    fn handle_pantry_key(&mut self, code: KeyCode) {
        let choices = FoodChoice::all();
        let count = choices.len();
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_selected_idx = (self.pantry_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_selected_idx = (self.pantry_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('f') | KeyCode::Char('F') => {
                let choice = choices[self.pantry_selected_idx];
                let msg = self.pet.feed_choice(choice);
                self.play_feedback_for_message(&msg, false);
                self.pantry_open = false;
                self.save_now();
            }
            KeyCode::Char(c @ '1'..='5') => {
                let idx = (c as usize) - ('1' as usize);
                if let Some(&choice) = choices.get(idx) {
                    let msg = self.pet.feed_choice(choice);
                    self.play_feedback_for_message(&msg, false);
                    self.pantry_open = false;
                    self.save_now();
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_open = false;
            }
            _ => {}
        }
    }

    fn handle_rename_key(&mut self, code: KeyCode) {
        let mut buffer = self.rename_buffer.take().unwrap_or_default();
        match code {
            KeyCode::Enter => {
                self.pet.rename(&buffer);
                self.sound.play(SoundEffect::BeepConfirm);
                self.save_now();
            }
            KeyCode::Esc => {}
            KeyCode::Backspace => {
                buffer.pop();
                self.rename_buffer = Some(buffer);
            }
            KeyCode::Char(c) if buffer.chars().count() < 18 => {
                buffer.push(c);
                self.rename_buffer = Some(buffer);
            }
            _ => self.rename_buffer = Some(buffer),
        }
    }

    fn handle_onboarding_key(&mut self, code: KeyCode) {
        let species_count = PetSpecies::all().len();
        let theme_count = ShellTheme::all().len();
        let stage_count = EvolutionStage::all().len();

        match self.onboarding_step {
            OnboardingStep::SelectSpecies => match code {
                KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_species_idx =
                        (self.selected_species_idx + species_count - 1) % species_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_species_idx = (self.selected_species_idx + 1) % species_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Char(' ') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.preview_auto_morph = false;
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.preview_auto_morph = !self.preview_auto_morph;
                }
                KeyCode::Enter => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.onboarding_step = OnboardingStep::SelectTheme;
                    self.onboarding_name_input =
                        PetSpecies::all()[self.selected_species_idx].default_name().to_string();
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    if self.sanctuary.hatched_count() > 0 || self.adopting_new {
                        self.adopting_new = false;
                        self.screen = AppScreen::Habitat;
                    } else {
                        self.should_quit = true;
                    }
                }
                _ => {}
            },
            OnboardingStep::SelectTheme => match code {
                KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_theme_idx =
                        (self.selected_theme_idx + theme_count - 1) % theme_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_theme_idx = (self.selected_theme_idx + 1) % theme_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') => {
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::BackTab | KeyCode::Esc => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.onboarding_step = OnboardingStep::SelectSpecies;
                }
                KeyCode::Enter => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.onboarding_step = OnboardingStep::EnterName;
                }
                _ => {}
            },
            OnboardingStep::EnterName => match code {
                KeyCode::Enter => self.finish_adoption(),
                KeyCode::Backspace => {
                    self.onboarding_name_input.pop();
                }
                KeyCode::Esc | KeyCode::BackTab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.onboarding_step = OnboardingStep::SelectTheme;
                }
                KeyCode::Char(c) if self.onboarding_name_input.chars().count() < 18 => {
                    self.onboarding_name_input.push(c);
                }
                _ => {}
            },
        }
    }

    fn finish_adoption(&mut self) {
        let species = PetSpecies::all()[self.selected_species_idx];
        let theme = ShellTheme::all()[self.selected_theme_idx];
        let name = self.onboarding_name_input.trim().to_string();
        let new_pet = Pet::new_custom(&name, species, theme);

        if self.adopting_new {
            self.sanctuary.add_pet(new_pet.clone());
        } else {
            self.sanctuary = Sanctuary::new_with_pet(new_pet.clone());
        }

        self.pet = new_pet;
        self.roster_selected_idx = self.sanctuary.active_index;
        self.adopting_new = false;
        self.screen = AppScreen::Habitat;
        self.sound.play(SoundEffect::HatchFanfare);
        self.save_now();
    }

    fn handle_roster_key(&mut self, code: KeyCode) {
        let count = self.sanctuary.pets.len().max(1);
        match code {
            KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                self.sound.play(SoundEffect::BeepMove);
                self.roster_selected_idx = (self.roster_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Right | KeyCode::Char('j') | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.roster_selected_idx = (self.roster_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.switch_to_roster_pet();
                self.screen = AppScreen::Habitat;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Esc | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.screen = AppScreen::Habitat;
            }
            _ => {}
        }
    }

    fn handle_minigame_key(&mut self, code: KeyCode) {
        let mut sound_to_play: Option<SoundEffect> = None;
        let finished = {
            let Some(game) = self.minigame.as_mut() else {
                self.screen = AppScreen::Habitat;
                return;
            };
            match code {
                KeyCode::Char('1') => {
                    game.switch_mode(GameMode::SnackCatcher);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Char('2') => {
                    game.switch_mode(GameMode::MindReader);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Char('3') => {
                    game.switch_mode(GameMode::DashRunner);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Tab => {
                    let next = game.mode.next();
                    game.switch_mode(next);
                    sound_to_play = Some(SoundEffect::BeepMove);
                }
                KeyCode::Up | KeyCode::Char(' ') | KeyCode::Char('w') | KeyCode::Char('W') => {
                    game.jump();
                    sound_to_play = Some(SoundEffect::BeepMove);
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('h') => {
                    let wins_before = game.mr_wins;
                    game.move_left();
                    if game.mode == GameMode::MindReader {
                        sound_to_play = Some(if game.mr_wins > wins_before {
                            SoundEffect::PlayWin
                        } else {
                            SoundEffect::PlayHit
                        });
                    }
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('l') => {
                    let wins_before = game.mr_wins;
                    game.move_right();
                    if game.mode == GameMode::MindReader {
                        sound_to_play = Some(if game.mr_wins > wins_before {
                            SoundEffect::PlayWin
                        } else {
                            SoundEffect::PlayHit
                        });
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    game.ticks_remaining = 0;
                }
                _ => {}
            }
            game.is_finished()
        };
        if let Some(sfx) = sound_to_play {
            self.sound.play(sfx);
        }
        if finished {
            self.finish_minigame();
        }
    }

    fn finish_minigame(&mut self) {
        if let Some(game) = self.minigame.take() {
            if game.score > 0 || game.coins_earned > 0 || game.xp_earned > 0 {
                let stage_before = self.pet.stage;
                let _ = self
                    .pet
                    .apply_minigame_reward(game.score, game.coins_earned, game.xp_earned);
                if self.pet.stage != stage_before {
                    self.sound.play(SoundEffect::LevelUpFanfare);
                } else {
                    self.sound.play(SoundEffect::PlayWin);
                }
            }
        }
        self.screen = AppScreen::Habitat;
        self.save_now();
    }

    fn handle_habitat_key(&mut self, code: KeyCode) {
        let icon_count = LcdIcon::selectable().len();
        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.save_now();
                self.should_quit = true;
            }
            KeyCode::Left | KeyCode::Char(',') => {
                self.sound.play(SoundEffect::BeepMove);
                self.selected_icon_idx = (self.selected_icon_idx + icon_count - 1) % icon_count;
            }
            KeyCode::Right | KeyCode::Char('.') | KeyCode::Char('a') | KeyCode::Char('A') => {
                self.sound.play(SoundEffect::BeepMove);
                self.selected_icon_idx = (self.selected_icon_idx + icon_count + 1) % icon_count;
            }
            KeyCode::Enter | KeyCode::Char('b') | KeyCode::Char('B') => {
                self.activate_selected_lcd_icon();
            }
            KeyCode::Char('?') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_help = true;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_meter = true;
                self.meter_page = 0;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = true;
                self.talk_selected_idx = 1;
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                let msg = self.pet.toggle_light();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                let msg = self.pet.discipline_scold();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char(' ') => {
                let stage_before = self.pet.stage;
                let msg = if self.pet.stage == EvolutionStage::Egg {
                    self.pet.warm_egg()
                } else {
                    self.pet.pet_love()
                };
                let hatched = self.pet.stage != stage_before;
                self.play_feedback_for_message(&msg, hatched);
                self.save_now();
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('1') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.feed();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.pet.stage == EvolutionStage::Egg {
                    let msg = self.pet.feed();
                    self.play_feedback_for_message(&msg, false);
                } else {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.pantry_open = true;
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char('2') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.play();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('3') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.pet_love();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('4') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.train();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('5') => {
                let msg = self.pet.toggle_sleep();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('6') => {
                let msg = self.pet.clean();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('7') => {
                let msg = self.pet.heal();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.launch_minigame(GameMode::SnackCatcher);
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.sanctuary.sync_active(&self.pet);
                self.roster_selected_idx = self.sanctuary.active_index;
                self.screen = AppScreen::SanctuaryRoster;
            }
            KeyCode::Char('[') => self.cycle_pet(false),
            KeyCode::Char(']') => self.cycle_pet(true),
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.rename_buffer = Some(self.pet.name.clone());
            }
            KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.pet.theme = self.pet.theme.next();
                self.save_now();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

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

        if self.screen != AppScreen::Onboarding
            && self.tick_count.is_multiple_of(DECAY_EVERY_TICKS)
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

// ── Spacious Retro Handheld Layout Helpers ─────────────────────────────────

fn handheld_rect(area: Rect) -> Rect {
    let w = area.width.min(HANDHELD_MAX_W);
    let h = area.height.min(HANDHELD_MAX_H);
    let x = area.x + area.width.saturating_sub(w) / 2;
    let y = area.y + area.height.saturating_sub(h) / 2;
    Rect::new(x, y, w, h)
}

fn outer_shell_block(title: &str, theme: ShellTheme) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme_color(theme)))
        .padding(Padding::new(1, 1, 0, 0))
        .title(Span::styled(
            format!(" {title} "),
            Style::default()
                .fg(theme_color(theme))
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
}

fn lcd_screen_block(title: &str, accent: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(accent))
        .padding(Padding::new(2, 2, 1, 1))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
}

fn custom_meter_line(
    label: &str,
    pct: u8,
    bar_color: Color,
    suffix: &str,
    suffix_color: Color,
) -> Line<'static> {
    let clamped = pct.min(100);
    let filled = ((clamped as usize) * 10 + 50) / 100;
    let filled = filled.min(10);
    let empty = 10usize.saturating_sub(filled);

    Line::from(vec![
        Span::styled(format!("{label:<13}"), Style::default().fg(Color::White)),
        Span::styled("[", Style::default().fg(Color::DarkGray)),
        Span::styled("█".repeat(filled), Style::default().fg(bar_color)),
        Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
        Span::styled("] ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{clamped:>3}% "),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(suffix.to_string(), Style::default().fg(suffix_color)),
    ])
}

fn render_unified_bezel(selected: LcdIcon, pet: &Pet) -> Line<'static> {
    let mut spans = Vec::with_capacity(18);
    for (idx, &icon) in LcdIcon::selectable().iter().enumerate() {
        let is_sel = icon == selected;
        let label = if icon == LcdIcon::Lights {
            if pet.light_on {
                "💡Lit"
            } else {
                "🌙Off"
            }
        } else {
            icon.label()
        };
        let style = if is_sel {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        spans.push(Span::styled(label.to_string(), style));
        if idx + 1 < LcdIcon::selectable().len() {
            spans.push(Span::raw(" "));
        }
    }
    Line::from(spans)
}

fn draw_too_small_notice(f: &mut Frame, area: Rect, theme: ShellTheme) {
    let block = outer_shell_block("MOCHIGOTCHI V-PET", theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🍡 Window Size Notice",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!(
            "Current size: {} × {}  (Minimum: {} × {})",
            area.width, area.height, MIN_TERM_WIDTH, MIN_TERM_HEIGHT
        )),
        Line::from("Please enlarge this terminal window slightly."),
        Line::from(""),
        Line::from(Span::styled(
            "[Q / Esc] Quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(
        Paragraph::new(Text::from(lines))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
        inner,
    );
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    let area = f.area();
    if area.width < MIN_TERM_WIDTH || area.height < MIN_TERM_HEIGHT {
        draw_too_small_notice(f, area, app.pet.theme);
        return;
    }

    let device = handheld_rect(area);
    f.render_widget(Clear, device);

    if let Some(report) = app.offline_report.as_ref() {
        draw_offline_report_screen(f, device, app, report);
    } else if app.show_help {
        draw_help_screen(f, device, app);
    } else if app.show_meter {
        draw_meter_screen(f, device, app);
    } else if app.talk_open {
        draw_talk_screen(f, device, app);
    } else if app.pantry_open {
        draw_pantry_screen(f, device, app);
    } else {
        match app.screen {
            AppScreen::Onboarding => draw_onboarding_step_screen(f, device, app),
            AppScreen::SanctuaryRoster => draw_roster_screen(f, device, app),
            AppScreen::MiniGame => draw_minigame_screen(f, device, app),
            AppScreen::Habitat => draw_habitat_screen(f, device, app),
        }
    }
}

fn stage_power_title(species: PetSpecies, stage: EvolutionStage) -> &'static str {
    match stage {
        EvolutionStage::Egg => "🥚 ELEMENTAL EGG — DORMANT SPIRIT",
        EvolutionStage::Baby => "🍼 BABY FORM — CUTE TINY HATCHLING",
        EvolutionStage::Teen => "🌱 TEEN FORM — AGILE STRIKER",
        EvolutionStage::Adult => "🌸 ADULT FORM — FIERCE GUARDIAN",
        EvolutionStage::CyberElder => match species {
            PetSpecies::IgnisDragon => "⚡ APEX ELDER — INFERNO BAHAMUT SOVEREIGN",
            PetSpecies::AzureWyrm => "⚡ APEX ELDER — TIDAL LEVIATHAN SOVEREIGN",
            PetSpecies::EmberFox => "⚡ APEX ELDER — NINE-TAILED KYUUBI SPIRIT",
            PetSpecies::PhoenixChick => "⚡ APEX ELDER — SOLAR GARUDA DEITY",
            PetSpecies::CrystalUni => "⚡ APEX ELDER — CELESTIAL KIRIN PEGASUS",
            PetSpecies::SlimeKing => "⚡ APEX ELDER — TITAN SLIME EMPEROR",
            PetSpecies::MochiCat => "⚡ APEX ELDER — BYAKKO SABER TIGER SHOGUN",
            PetSpecies::BobaPup => "⚡ APEX ELDER — FENRIR DIRE WOLF GUARDIAN",
            PetSpecies::CloudBun => "⚡ APEX ELDER — RAIJIN STORM VALKYRIE",
            PetSpecies::StarPengu => "⚡ APEX ELDER — EMPOLEON FROST KAISER",
        },
    }
}

fn render_evolution_morph_bar(active_stage: EvolutionStage, auto_morph: bool) -> Line<'static> {
    let mut spans = Vec::with_capacity(12);
    for (idx, &stage) in EvolutionStage::all().iter().enumerate() {
        let is_cur = stage == active_stage;
        let style = if is_cur {
            Style::default()
                .fg(Color::Black)
                .bg(stage_color(stage))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(format!(" {} ", stage.badge()), style));
        if idx + 1 < EvolutionStage::all().len() {
            spans.push(Span::styled("▸", Style::default().fg(Color::DarkGray)));
        }
    }
    spans.push(Span::styled(
        if auto_morph { " 🔄Auto" } else { " ⏸Hold" },
        Style::default().fg(Color::LightCyan),
    ));
    Line::from(spans)
}

/// Step-by-step Onboarding with a balanced 3-Zone vertical layout (Header / Sprite Stage / Footer)
/// so nothing is cramped at the top and there is no empty void at the bottom!
fn draw_onboarding_step_screen(f: &mut Frame, device: Rect, app: &App) {
    let species = PetSpecies::all()[app.selected_species_idx];
    let theme = ShellTheme::all()[app.selected_theme_idx];
    let preview_stage = EvolutionStage::all()[app.preview_stage_idx % EvolutionStage::all().len()];
    let (sp_primary, _, sp_accent) = species_palette(species);

    let shell = outer_shell_block(
        &format!(
            "🍡 MOCHIGOTCHI V-PET  •  NEW COMPANION  •  {}",
            app.sound.status_badge()
        ),
        theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    match app.onboarding_step {
        OnboardingStep::SelectSpecies => {
            let lcd = lcd_screen_block("STEP 1 OF 3  •  CHOOSE COMPANION", sp_primary);
            let lcd_inner = lcd.inner(inner);
            f.render_widget(lcd, inner);

            let zones = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Min(12),
                    Constraint::Length(4),
                ])
                .split(lcd_inner);

            // Zone 1: Top Header with breathing room
            let header_lines = vec![
                Line::from(vec![
                    Span::styled("◀  ", Style::default().fg(Color::LightYellow)),
                    Span::styled(
                        format!(
                            "{:02}/10  {}  ",
                            app.selected_species_idx + 1,
                            species.display_name()
                        ),
                        Style::default().fg(sp_primary).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        species.star_bar().to_string(),
                        Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  ▶", Style::default().fg(Color::LightYellow)),
                ]),
                render_evolution_morph_bar(preview_stage, app.preview_auto_morph),
                Line::from(Span::styled(
                    stage_power_title(species, preview_stage),
                    Style::default()
                        .fg(stage_color(preview_stage))
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];
            f.render_widget(
                Paragraph::new(Text::from(header_lines)).alignment(Alignment::Center),
                zones[0],
            );

            // Zone 2: 40×22 3/4 Side-Profile Pixel Art Stage
            let sprite_lines = render_colored_preview(species, preview_stage, app.tick_count);
            f.render_widget(
                Paragraph::new(Text::from(sprite_lines)).alignment(Alignment::Center),
                zones[1],
            );

            // Zone 3: Bottom Lore & Controls anchored cleanly at the bottom of the LCD
            let footer_lines = vec![
                Line::from(Span::styled(
                    species.description().to_string(),
                    Style::default().fg(Color::White),
                )),
                Line::from(vec![
                    Span::styled(
                        format!("{} • {} • Snack: ", species.family(), species.element_badge()),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(
                        species.favorite_snack().to_string(),
                        Style::default().fg(Color::LightGreen),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "[←/→] Species   [V/Space] Step Form   [M] Auto-Morph   [Enter] Next Step",
                    Style::default()
                        .fg(sp_accent)
                        .add_modifier(Modifier::BOLD),
                )),
            ];
            f.render_widget(
                Paragraph::new(Text::from(footer_lines)).alignment(Alignment::Center),
                zones[2],
            );
        }

        OnboardingStep::SelectTheme => {
            let lcd = lcd_screen_block("STEP 2 OF 3  •  CHOOSE SHELL COLOR", theme_color(theme));
            let lcd_inner = lcd.inner(inner);
            f.render_widget(lcd, inner);

            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Companion: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("{} ({})", species.display_name(), species.star_bar()),
                        Style::default().fg(sp_primary).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Select your retro handheld casing color:",
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
            ];

            for (idx, t) in ShellTheme::all().iter().enumerate() {
                let selected = idx == app.selected_theme_idx;
                let row_text = if selected {
                    format!("▸  {:<22}  ◂", t.display_name())
                } else {
                    format!("   {:<22}   ", t.display_name())
                };
                lines.push(Line::from(Span::styled(
                    row_text,
                    if selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(theme_color(*t))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme_color(*t))
                    },
                )));
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "[↑/↓] Select Shell   [Enter] Next Step   [Esc] Back",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )));

            f.render_widget(
                Paragraph::new(Text::from(lines)).alignment(Alignment::Center),
                lcd_inner,
            );
        }

        OnboardingStep::EnterName => {
            let lcd = lcd_screen_block("STEP 3 OF 3  •  NAME YOUR COMPANION", Color::LightYellow);
            let lcd_inner = lcd.inner(inner);
            f.render_widget(lcd, inner);

            let zones = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Min(12),
                    Constraint::Length(5),
                ])
                .split(lcd_inner);

            f.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    format!(
                        "{}  •  Shell: {}",
                        species.display_name(),
                        theme.display_name()
                    ),
                    Style::default().fg(sp_primary).add_modifier(Modifier::BOLD),
                )))
                .alignment(Alignment::Center),
                zones[0],
            );

            f.render_widget(
                Paragraph::new(Text::from(render_colored_preview(
                    species,
                    EvolutionStage::Egg,
                    app.tick_count,
                )))
                .alignment(Alignment::Center),
                zones[1],
            );

            let bottom_lines = vec![
                Line::from(Span::styled(
                    "Enter a name for your new companion:",
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!("   [  {}▏  ]   ", app.onboarding_name_input),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "[Enter] Adopt & Start   [Backspace] Edit   [Esc] Back",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )),
            ];
            f.render_widget(
                Paragraph::new(Text::from(bottom_lines)).alignment(Alignment::Center),
                zones[2],
            );
        }
    }
}

/// Spacious 3-Zone Habitat Screen with live Emotions, Autonomous Strolling,
/// Direct First-Person Reactions, and balanced vertical distribution!
fn draw_habitat_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let (sp_primary, _, _) = species_palette(pet.species);
    let selected_icon = LcdIcon::selectable()[app.selected_icon_idx % LcdIcon::selectable().len()];

    let shell_title = format!(
        "🍡 MOCHIGOTCHI • {} ({}) • {}",
        pet.name.to_uppercase(),
        pet.species.display_name(),
        app.sound.status_badge()
    );
    let shell = outer_shell_block(&shell_title, pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(18),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(render_unified_bezel(selected_icon, pet)).alignment(Alignment::Center),
        rows[0],
    );

    let (sky, _) = sky_scene(app.tick_count);
    let lcd_title = if !pet.is_alive {
        format!("👻 SPIRIT MEMORIAL  •  {}  •  PRESS [H] TO REVIVE", pet.name)
    } else if pet.stage == EvolutionStage::Egg {
        format!(
            "{}  •  🥚 EGG ({}% WARM)  •  {}",
            sky,
            pet.incubation,
            pet.mood.badge()
        )
    } else {
        format!(
            "{}  •  {} LV.{}  •  {}  •  💰{}  •  {}",
            sky,
            pet.stage.badge(),
            pet.level,
            pet.mood.badge(),
            pet.coins,
            pet.attention_call.badge()
        )
    };
    let lcd = lcd_screen_block(&lcd_title, sp_primary);
    let lcd_inner = lcd.inner(rows[1]);
    f.render_widget(lcd, rows[1]);

    let lcd_zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(13),
            Constraint::Length(2),
        ])
        .split(lcd_inner);

    // Zone 1: Thought/Activity Bubble (`💭`) + First-Person Speech Bubble (`💬`) + Spacer
    let speech_line = if !pet.light_on && pet.is_alive && pet.last_pet_speech.is_none() {
        Line::from(Span::styled(
            "🌙 [ ROOM LIGHT OFF — NIGHT MODE ]",
            Style::default().fg(Color::LightBlue),
        ))
    } else {
        Line::from(Span::styled(
            speech_bubble(pet, app.tick_count),
            Style::default()
                .fg(mood_color(pet.mood))
                .add_modifier(Modifier::BOLD),
        ))
    };

    let bubble_lines = vec![
        Line::from(Span::styled(
            thought_bubble(pet, app.tick_count),
            Style::default().fg(Color::LightCyan),
        )),
        speech_line,
        Line::from(""),
    ];
    f.render_widget(
        Paragraph::new(Text::from(bubble_lines)).alignment(Alignment::Center),
        lcd_zones[0],
    );

    // Zone 2: 40×22 Pixel Art Sprite Stage (with autonomous strolling!)
    let sprite = render_colored_sprite(pet, app.tick_count);
    f.render_widget(
        Paragraph::new(Text::from(sprite)).alignment(Alignment::Center),
        lcd_zones[1],
    );

    // Zone 3: Vitals Bar anchored cleanly at the bottom of the LCD box
    let vitals_line = if pet.stage == EvolutionStage::Egg {
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        Line::from(Span::styled(
            format!("Incubation: [{}] {}%   •   Bond: {}%", boxes.trim(), pet.incubation, pet.bond),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
    } else {
        let poop_suffix = if pet.poop_count > 0 {
            format!("   💩×{}", pet.poop_count)
        } else {
            String::new()
        };
        Line::from(vec![
            Span::styled(
                format!("🍙 {}   ", format_hearts(pet.hungry_hearts())),
                Style::default().fg(if pet.hunger <= 15 {
                    Color::LightRed
                } else {
                    Color::LightGreen
                }),
            ),
            Span::styled(
                format!("💖 {}   ", format_hearts(pet.happy_hearts())),
                Style::default().fg(Color::LightMagenta),
            ),
            Span::styled(
                format!("💚 {}%   ", pet.health),
                Style::default().fg(if pet.health <= 35 {
                    Color::LightRed
                } else {
                    Color::LightCyan
                }),
            ),
            Span::styled(
                format!("⚡ {}%   ⚖ {}g{poop_suffix}", pet.energy, pet.weight_grams),
                Style::default().fg(Color::Yellow),
            ),
        ])
    };
    f.render_widget(
        Paragraph::new(Text::from(vec![Line::from(""), vitals_line])).alignment(Alignment::Center),
        lcd_zones[2],
    );

    // Bottom 2-line Footer
    let footer_lines = if let Some(buf) = app.rename_buffer.as_ref() {
        vec![
            Line::from(Span::styled(
                format!("🔖 Rename: [ {buf}▏ ]  (Enter = Save, Esc = Cancel)"),
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ]
    } else if pet.stage == EvolutionStage::Egg {
        vec![
            Line::from(Span::styled(
                pet.advisor_tip(now),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                "[W] Warm  [Y] Talk  [I] Meter  [L] Pets  [N] Adopt  [U] Sound  [?] Keys",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                pet.advisor_tip(now),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                "[←/→+Enter] Bezel  [F] Meal  [Y] Talk  [G] Arcade  [I] Meter  [U] Sound  [?] Keys",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    };
    f.render_widget(
        Paragraph::new(Text::from(footer_lines)).alignment(Alignment::Center),
        rows[2],
    );
}

fn draw_talk_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let (sp_primary, _, _) = species_palette(pet.species);
    let prompt = pet.dialogue_prompt(app.tick_count / 20);

    let shell = outer_shell_block(
        &format!("💬 HEART-TO-HEART CHAT WITH {}", pet.name.to_uppercase()),
        pet.theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!(
            "{}  •  {}  •  BOND {}%",
            pet.species.display_name(),
            pet.mood.badge(),
            pet.bond
        ),
        sp_primary,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(13),
            Constraint::Length(4),
        ])
        .split(lcd_inner);

    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                prompt.inner_thought,
                Style::default().fg(Color::LightCyan),
            )),
            Line::from(Span::styled(
                prompt.pet_line,
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ]))
        .alignment(Alignment::Center),
        zones[0],
    );

    f.render_widget(
        Paragraph::new(Text::from(render_colored_sprite(pet, app.tick_count)))
            .alignment(Alignment::Center),
        zones[1],
    );

    let opt1_style = if app.talk_selected_idx == 1 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let opt2_style = if app.talk_selected_idx == 2 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(format!(" {} ", prompt.option_1), opt1_style)),
            Line::from(Span::styled(format!(" {} ", prompt.option_2), opt2_style)),
            Line::from(""),
            Line::from(Span::styled(
                "[↑/↓ or 1/2] Choose Response   •   [Enter] Reply   •   [Esc] Back",
                Style::default().fg(Color::Cyan),
            )),
        ]))
        .alignment(Alignment::Center),
        zones[2],
    );
}

fn draw_meter_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let shell = outer_shell_block("📏 TAMAGOTCHI STATUS METER", pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let page_idx = app.meter_page % 4;
    let title = if pet.stage == EvolutionStage::Egg {
        "🥚 Egg Incubator Status".to_string()
    } else {
        format!(
            "📊 Tamagotchi Vitals  •  Page {}/4 (←/→ or Space)",
            page_idx + 1
        )
    };
    let lcd = lcd_screen_block(&title, Color::LightCyan);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut lines = Vec::new();
    if pet.stage == EvolutionStage::Egg {
        let warm_cd = pet.warm_cooldown_remaining(now);
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let step_boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Egg Incubator Status",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        lines.push(custom_meter_line(
            "🥚 Incubation",
            pet.incubation,
            Color::LightYellow,
            "Hatch at 100%",
            Color::LightYellow,
        ));
        lines.push(Line::from(""));
        lines.push(Line::from(format!(
            "🔥 Warmings:   [{}] ({warm_sessions}/5 sessions)",
            step_boxes.trim()
        )));
        lines.push(Line::from(format!(
            "⏳ Warm Timer: {}",
            if warm_cd == 0 {
                "Ready now! Press [W]".to_string()
            } else {
                format!("Wait {}", format_duration_human(warm_cd))
            }
        )));
        lines.push(Line::from(""));
        lines.push(custom_meter_line(
            "🤝 Bond",
            pet.bond,
            Color::LightBlue,
            "Keeper Bond",
            Color::LightBlue,
        ));
    } else {
        match page_idx {
            0 => {
                let xp_pct = (pet.xp_progress_ratio() * 100.0).round() as u8;
                let target_xp = pet.stage.next_stage().map_or(pet.xp, |n| n.min_xp());
                lines.push(Line::from(Span::styled(
                    "PAGE 1/4  •  IDENTITY, EMOTION, WEIGHT & CARE RANK",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(format!(
                    " • Companion: {} ({})",
                    pet.name,
                    pet.species.display_name()
                )));
                lines.push(Line::from(format!(
                    " • Emotion:   {}   •   Stage: {} (Lv.{} • {} XP)",
                    pet.mood.badge(),
                    pet.stage.badge(),
                    pet.level,
                    pet.xp
                )));
                lines.push(Line::from(format!(
                    " • Care Rank: {}",
                    pet.care_rank().badge()
                )));
                lines.push(Line::from(format!(
                    " • Real Age:  {}",
                    format_duration_human(pet.age_seconds)
                )));
                lines.push(Line::from(format!(
                    " • Weight:    {}g (Base {}g{})",
                    pet.weight_grams,
                    pet.stage.base_weight_grams(),
                    if pet.is_overweight() {
                        " — Overweight! Play [P]"
                    } else {
                        " — Healthy!"
                    }
                )));
                lines.push(Line::from(""));
                lines.push(custom_meter_line(
                    "⭐ Stage XP",
                    xp_pct,
                    Color::LightMagenta,
                    &format!("{}/{} XP", pet.xp, target_xp),
                    Color::LightMagenta,
                ));
            }
            1 => {
                lines.push(Line::from(Span::styled(
                    "PAGE 2/4  •  HUNGRY & HAPPY HEARTS",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                lines.push(custom_meter_line(
                    "🍙 Hungry",
                    pet.hunger,
                    Color::LightGreen,
                    &format_hearts(pet.hungry_hearts()),
                    Color::LightGreen,
                ));
                lines.push(Line::from(""));
                lines.push(custom_meter_line(
                    "💖 Happy",
                    pet.happiness,
                    Color::LightMagenta,
                    &format_hearts(pet.happy_hearts()),
                    Color::LightMagenta,
                ));
                lines.push(Line::from(""));
                lines.push(Line::from(format!(
                    " • Sweet Streak:   {}/3 snacks {}",
                    pet.snacks_in_a_row,
                    if pet.has_toothache {
                        "(🦷 TOOTHACHE!)"
                    } else {
                        "(Safe)"
                    }
                )));
                lines.push(Line::from(format!(
                    " • Favorite Meal:  {}",
                    pet.species.favorite_snack()
                )));
                lines.push(Line::from(format!(" • Coins Purse:    {} 💰", pet.coins)));
            }
            2 => {
                let disc_blocks = pet.discipline_blocks() as usize;
                let disc_bar = format!(
                    "{}{}",
                    "■".repeat(disc_blocks),
                    "·".repeat(4usize.saturating_sub(disc_blocks))
                );
                lines.push(Line::from(Span::styled(
                    "PAGE 3/4  •  DISCIPLINE, HYGIENE & HEALTH",
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                lines.push(custom_meter_line(
                    "📢 Discipline",
                    pet.discipline,
                    Color::LightYellow,
                    &format!("{disc_bar} {disc_blocks}/4"),
                    Color::LightYellow,
                ));
                lines.push(custom_meter_line(
                    "⚡ Energy",
                    pet.energy,
                    Color::Yellow,
                    if pet.is_sleeping { "Sleeping" } else { "Awake" },
                    Color::Yellow,
                ));
                lines.push(custom_meter_line(
                    "🚽 Hygiene",
                    pet.cleanliness,
                    Color::LightCyan,
                    &format!("Poop: {}", pet.poop_count),
                    Color::LightCyan,
                ));
                lines.push(custom_meter_line(
                    "💚 Health",
                    pet.health,
                    Color::LightRed,
                    if !pet.is_alive {
                        "Passed Out!"
                    } else if pet.has_toothache {
                        "Toothache!"
                    } else if pet.is_sick {
                        "Sick!"
                    } else {
                        "Healthy"
                    },
                    Color::LightGreen,
                ));
                lines.push(custom_meter_line(
                    "🤝 Bond",
                    pet.bond,
                    Color::LightBlue,
                    &format!("Mistakes: {}", pet.care_mistakes),
                    Color::Gray,
                ));
            }
            _ => {
                lines.push(Line::from(Span::styled(
                    "PAGE 4/4  •  EVOLUTION GOAL & CARE DIARY",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    pet.next_evolution_requirements(),
                    Style::default().fg(Color::LightYellow),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Recent Care Diary:",
                    Style::default().fg(Color::White),
                )));
                for entry in pet.action_log.iter().rev().take(4) {
                    lines.push(Line::from(Span::styled(
                        format!(" • {entry}"),
                        Style::default().fg(Color::Gray),
                    )));
                }
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[←/→ or Space] Flip Page   •   [Esc / I] Return to Habitat",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));

    f.render_widget(Paragraph::new(Text::from(lines)), lcd_inner);
}

fn draw_pantry_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("🍱 TAMAGOTCHI PANTRY  •  MEALS & SNACKS", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!(
            "COINS: {} 💰  •  SWEET STREAK: {}/3",
            app.pet.coins, app.pet.snacks_in_a_row
        ),
        Color::LightYellow,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut items: Vec<ListItem> = Vec::new();
    for (idx, choice) in FoodChoice::all().iter().enumerate() {
        let selected = idx == app.pantry_selected_idx;
        let marker = if selected { "▸" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        items.push(ListItem::new(vec![
            Line::from(Span::styled(
                format!("{marker} [{}] {}", idx + 1, choice.title(app.pet.species)),
                style,
            )),
            Line::from(Span::styled(
                format!("    {}", choice.summary()),
                Style::default().fg(Color::Gray),
            )),
        ]));
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(2)])
        .split(lcd_inner);

    f.render_widget(List::new(items), rows[0]);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[↑/↓ or 1-5] Select   [Enter] Feed   [Esc] Return to Habitat",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );
}

fn draw_offline_report_screen(
    f: &mut Frame,
    device: Rect,
    app: &App,
    report: &OfflineReport,
) {
    let shell = outer_shell_block("🌙 WELCOME BACK TO YOUR SANCTUARY", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!("AWAY FOR {}", report.formatted_duration().to_uppercase()),
        Color::LightCyan,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut lines = vec![
        Line::from(Span::styled(
            format!("While you were away, {} lived on:", report.pet_name),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    for ev in &report.events {
        lines.push(Line::from(Span::styled(
            format!(" • {ev}"),
            Style::default().fg(Color::LightYellow),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("💡 {}", app.pet.advisor_tip(Utc::now())),
        Style::default().fg(Color::LightGreen),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter / Space] Continue to Habitat",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        lcd_inner,
    );
}

fn draw_help_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("📖 TAMAGOTCHI HARDWARE & KEYS GUIDE", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block("CONTROLS & RULES", Color::LightMagenta);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let lines = vec![
        Line::from(Span::styled(
            "📟 Bezel Navigation (3-Button Hardware Style)",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • Button A (←/→): Select icon on top bezel"),
        Line::from(" • Button B (Enter): Open highlighted icon"),
        Line::from(" • Button C (Esc): Return to main LCD Habitat"),
        Line::from(Span::styled(
            "🍙 Meals, Sweet Snacks, Chat & 3-in-1 Arcade",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [F] Meal (+1 Hungry ♥, +1g) • [M] Pantry Menu"),
        Line::from(" • [Y] Heart-to-Heart Chat (see thoughts & reply!)"),
        Line::from(" • [G] 3-in-1 Arcade (1:Catcher 2:MindReader 3:Dash)"),
        Line::from(Span::styled(
            "💡 Room Light, Discipline, Revival & Sanctuary",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [S] Sleep + [K] Light OFF • [C] Bath • [D] Scold"),
        Line::from(" • [H] Medicine / Phoenix Tear Revival • [U] Sound"),
        Line::from(" • [L] Sanctuary Pets • [N] Adopt • [R] Rename"),
        Line::from(""),
        Line::from(Span::styled(
            "[Esc / Enter / ?] Return to Habitat",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    ];

    f.render_widget(Paragraph::new(Text::from(lines)), lcd_inner);
}

fn draw_roster_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block(
        &format!(
            "🏰 SANCTUARY ROSTER  •  {} COMPANIONS (LV.{})",
            app.sanctuary.pets.len(),
            app.sanctuary.sanctuary_level
        ),
        app.pet.theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let idx = app
        .roster_selected_idx
        .min(app.sanctuary.pets.len().saturating_sub(1));
    let highlighted = &app.sanctuary.pets[idx];
    let (h_primary, _, _) = species_palette(highlighted.species);
    let is_active = idx == app.sanctuary.active_index;

    let lcd = lcd_screen_block(
        &format!(
            "◀  COMPANION {:02}/{:02} {}  ▶",
            idx + 1,
            app.sanctuary.pets.len(),
            if is_active { "(★ ACTIVE)" } else { "" }
        ),
        h_primary,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(13),
            Constraint::Length(3),
        ])
        .split(lcd_inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!(
                "{}  •  {}  •  {} Lv.{}  •  {}",
                highlighted.name,
                highlighted.species.display_name(),
                highlighted.stage.badge(),
                highlighted.level,
                highlighted.mood.badge()
            ),
            Style::default().fg(h_primary).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        zones[0],
    );

    f.render_widget(
        Paragraph::new(Text::from(render_colored_sprite(
            highlighted,
            app.tick_count,
        )))
        .alignment(Alignment::Center),
        zones[1],
    );

    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                format!(
                    "Hungry: {}   Happy: {}   Health: {}%",
                    format_hearts(highlighted.hungry_hearts()),
                    format_hearts(highlighted.happy_hearts()),
                    highlighted.health
                ),
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "[←/→ or ↑/↓] Browse   [Enter] Switch   [N] Adopt   [Esc/L] Back",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
        ]))
        .alignment(Alignment::Center),
        zones[2],
    );
}

fn render_arcade_mode_tabs(active_mode: GameMode) -> Line<'static> {
    let mut spans = Vec::with_capacity(8);
    for (idx, &mode) in GameMode::all().iter().enumerate() {
        let is_cur = mode == active_mode;
        let style = if is_cur {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        spans.push(Span::styled(format!(" {} ", mode.title()), style));
        if idx + 1 < GameMode::all().len() {
            spans.push(Span::raw("   "));
        }
    }
    Line::from(spans)
}

fn draw_minigame_screen(f: &mut Frame, device: Rect, app: &App) {
    let (sp_primary, _, _) = species_palette(app.pet.species);
    let shell = outer_shell_block(
        &format!(
            "🎮 RETRO PIXEL ARCADE CABINET  •  {}",
            app.sound.status_badge()
        ),
        app.pet.theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let (active_mode, arena, status) = match app.minigame.as_ref() {
        Some(game) => (
            game.mode,
            Text::from(game.render_colored_lines(sp_primary)),
            format!(
                "SCORE {}  •  STREAK x{}  •  +{} 💰  •  +{} XP  •  ⏱ {}T",
                game.score,
                game.combo.max(1),
                game.coins_earned,
                game.xp_earned,
                game.ticks_remaining
            ),
        ),
        None => (
            GameMode::SnackCatcher,
            Text::from(""),
            "no game running".to_string(),
        ),
    };

    let lcd = lcd_screen_block(&status, sp_primary);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(12),
            Constraint::Length(2),
        ])
        .split(lcd_inner);

    f.render_widget(
        Paragraph::new(render_arcade_mode_tabs(active_mode)).alignment(Alignment::Center),
        rows[0],
    );
    f.render_widget(
        Paragraph::new(arena).alignment(Alignment::Center),
        rows[1],
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[←/→] Move/Guess   [Space/↑] Jump   [1/2/3/Tab] Switch Game   [Esc/Q] Claim & Exit",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[2],
    );
}

pub fn run_tui(
    sanctuary: Sanctuary,
    paths: StoragePaths,
    start_onboarding: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_with_report(sanctuary, paths, start_onboarding, None)
}

pub fn run_tui_with_report(
    sanctuary: Sanctuary,
    paths: StoragePaths,
    start_onboarding: bool,
    offline_report: Option<OfflineReport>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new_with_sanctuary(sanctuary, paths, start_onboarding);
    app.offline_report = offline_report;
    let tick_rate = Duration::from_millis(TICK_MS);
    let mut last_tick = Instant::now();

    let loop_result: io::Result<()> = (|| {
        loop {
            terminal.draw(|f| draw_ui(f, &app))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        app.handle_key(key.code);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }

            if app.should_quit {
                break;
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    loop_result?;
    app.save_now();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn tmp_paths(tag: &str) -> StoragePaths {
        let dir = std::env::temp_dir().join(format!("mochi_tui_{tag}_{}", std::process::id()));
        StoragePaths::new(dir.join("save.json"), dir.join("save.db"))
    }

    #[test]
    fn onboarding_auto_morphs_evolution_and_adopts_companions() {
        let paths = tmp_paths("adopt");
        let _ = std::fs::remove_dir_all(paths.json_path.parent().unwrap());

        let mut app = App::new_onboarding(paths.clone());
        assert_eq!(app.screen, AppScreen::Onboarding);
        assert!(app.preview_auto_morph);
        let initial_stage = app.preview_stage_idx;
        for _ in 0..AUTO_MORPH_TICKS {
            app.on_tick();
        }
        assert_ne!(
            app.preview_stage_idx, initial_stage,
            "Step 1 must auto-animate evolution stages every AUTO_MORPH_TICKS"
        );

        for _ in 0..4 {
            app.handle_key(KeyCode::Down);
        }
        app.handle_key(KeyCode::Char('v'));
        assert!(!app.preview_auto_morph);
        app.handle_key(KeyCode::Enter);
        assert_eq!(app.onboarding_step, OnboardingStep::SelectTheme);
        app.handle_key(KeyCode::Enter);
        assert_eq!(app.onboarding_step, OnboardingStep::EnterName);
        for _ in 0..6 {
            app.handle_key(KeyCode::Backspace);
        }
        app.handle_key(KeyCode::Char('X'));
        app.handle_key(KeyCode::Enter);

        assert_eq!(app.screen, AppScreen::Habitat);
        assert_eq!(app.pet.species, PetSpecies::IgnisDragon);
        assert_eq!(app.pet.name, "X");

        app.handle_key(KeyCode::Char('y'));
        assert!(app.talk_open);
        app.handle_key(KeyCode::Char('1'));
        assert!(!app.talk_open);
        assert!(app.pet.bond > 10);

        let _ = std::fs::remove_dir_all(paths.json_path.parent().unwrap());
    }

    #[test]
    fn renders_resize_guard_on_small_terminal_and_full_ui_on_standard_and_wide() {
        for (w, h) in [(54u16, 18u16), (68, 24), (96, 30), (140, 44)] {
            let backend = TestBackend::new(w, h);
            let mut terminal = Terminal::new(backend).unwrap();
            let paths = tmp_paths(&format!("render{w}"));
            let mut app = App::new(Pet::new("Mochi"), paths);
            app.pet.stage = EvolutionStage::CyberElder;
            app.pantry_open = true;
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
            app.pantry_open = false;
            app.talk_open = true;
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
            app.talk_open = false;
            app.show_meter = true;
            for page in 0..4 {
                app.meter_page = page;
                terminal.draw(|f| draw_ui(f, &app)).unwrap();
            }
            app.show_meter = false;
            app.show_help = true;
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
        }
    }

    #[test]
    fn every_screen_and_arcade_mode_renders() {
        let paths = tmp_paths("screens");
        let mut app = App::new(Pet::new("Mochi"), paths.clone());
        app.pet.stage = EvolutionStage::CyberElder;
        let backend = TestBackend::new(100, 32);
        let mut terminal = Terminal::new(backend).unwrap();

        app.handle_key(KeyCode::Char('g'));
        assert_eq!(app.screen, AppScreen::MiniGame);
        for key in [KeyCode::Char('1'), KeyCode::Char('2'), KeyCode::Char('3')] {
            app.handle_key(key);
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
        }
        app.handle_key(KeyCode::Esc);
        assert_eq!(app.screen, AppScreen::Habitat);

        let mut wizard = App::new_onboarding(paths);
        for step in [
            OnboardingStep::SelectSpecies,
            OnboardingStep::SelectTheme,
            OnboardingStep::EnterName,
        ] {
            wizard.onboarding_step = step;
            terminal.draw(|f| draw_ui(f, &wizard)).unwrap();
        }
    }

    #[test]
    fn tick_loop_decays_stats_and_arcade_finishes() {
        let paths = tmp_paths("tick");
        let mut app = App::new(Pet::new("Mochi"), paths);
        app.pet.stage = EvolutionStage::Baby;
        let hunger_before = app.pet.hunger;
        for _ in 0..(DECAY_EVERY_TICKS * 2) {
            app.on_tick();
        }
        assert!(app.pet.hunger < hunger_before);

        app.pet.energy = 90;
        app.handle_key(KeyCode::Char('g'));
        assert_eq!(app.screen, AppScreen::MiniGame);
        while app.screen == AppScreen::MiniGame {
            app.on_tick();
        }
        assert!(app.pet.xp > 0);
    }
}
