//! Minimalist, Retro, Step-by-Step Handheld LCD Interface for **Mochigotchi**.
//!
//! Designed around a single centered **66×22 Retro Handheld V-Pet Frame**
//! (`BorderType::Double` outer shell + `BorderType::Rounded` inner LCD):
//! - **Step-by-step Onboarding**: Step 1 (Select Companion with 24×16 Half-Block
//!   Pixel Art) -> press Enter -> switches screen to Step 2 (Select Shell Color)
//!   -> press Enter -> switches screen to Step 3 (Name Companion).
//! - **Minimalist LCD Habitat**: Displays only the 7-icon Tamagotchi Bezel, the
//!   centered 24×16 TrueColor Pixel Art Sprite, and a 2-line retro status ticker.
//! - **Step-by-step Subscreens**: Opening the Status Meter (`[I]` / `📏Meter`),
//!   Pantry (`[M]` / `🍙Food`), Sanctuary Roster (`[L]`), Arcade (`[G]`), or
//!   Keys Guide (`[?]`) cleanly replaces the inner LCD screen instead of
//!   cluttering the main view with side-by-side panels.

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

use crate::minigame::MiniGame;
use crate::pet::{
    format_duration_human, format_hearts, AttentionCall, EvolutionStage, FoodChoice, LcdIcon,
    OfflineReport, Pet, PetSpecies, Sanctuary, ShellTheme,
};
use crate::sprites::{
    mood_color, render_colored_preview, render_colored_sprite, sky_scene, speech_bubble,
    species_palette, theme_color,
};
use crate::storage::{save_sanctuary, StoragePaths};

const TICK_MS: u64 = 250;
/// In-game stat decay tick interval: 24 ticks * 250ms = 6 seconds.
const DECAY_EVERY_TICKS: usize = 24;

/// Minimum terminal dimensions required for the centered retro handheld LCD.
pub const MIN_TERM_WIDTH: u16 = 68;
pub const MIN_TERM_HEIGHT: u16 = 22;

/// Preferred retro handheld frame dimensions (centered inside larger terminals).
const HANDHELD_MAX_W: u16 = 74;
const HANDHELD_MAX_H: u16 = 25;

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
    pub screen: AppScreen,
    pub onboarding_step: OnboardingStep,
    pub selected_species_idx: usize,
    pub selected_theme_idx: usize,
    pub preview_stage_idx: usize,
    pub roster_selected_idx: usize,
    pub selected_icon_idx: usize,
    pub pantry_open: bool,
    pub pantry_selected_idx: usize,
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
            screen: if start_onboarding {
                AppScreen::Onboarding
            } else {
                AppScreen::Habitat
            },
            onboarding_step: OnboardingStep::SelectSpecies,
            selected_species_idx,
            selected_theme_idx,
            preview_stage_idx: 3,
            roster_selected_idx: 0,
            selected_icon_idx: 0,
            pantry_open: false,
            pantry_selected_idx: 0,
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

    fn switch_to_roster_pet(&mut self) {
        self.sanctuary.sync_active(&self.pet);
        if self.sanctuary.switch_pet(self.roster_selected_idx) {
            self.pet = self.sanctuary.active_pet().clone();
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
        self.save_now();
    }

    fn start_adopting(&mut self) {
        self.adopting_new = true;
        self.pantry_open = false;
        self.show_meter = false;
        self.show_help = false;
        self.screen = AppScreen::Onboarding;
        self.onboarding_step = OnboardingStep::SelectSpecies;
        self.selected_species_idx = 0;
        self.selected_theme_idx = 0;
        self.preview_stage_idx = 3;
        self.onboarding_name_input = PetSpecies::all()[0].default_name().to_string();
    }

    fn launch_minigame(&mut self) {
        let refusal = if self.pet.stage == EvolutionStage::Egg {
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
            Some(msg) => self.pet.log_action(msg),
            None => {
                self.minigame = Some(MiniGame::new(38, 10, 120));
                self.screen = AppScreen::MiniGame;
            }
        }
    }

    fn activate_selected_lcd_icon(&mut self) {
        let icon = LcdIcon::selectable()[self.selected_icon_idx % LcdIcon::selectable().len()];
        match icon {
            LcdIcon::Food => {
                if self.pet.stage == EvolutionStage::Egg {
                    let _ = self.pet.warm_egg();
                    self.save_now();
                } else {
                    self.pantry_open = true;
                }
            }
            LcdIcon::Lights => {
                let _ = self.pet.toggle_light();
                self.save_now();
            }
            LcdIcon::Play => {
                let _ = self.pet.play();
                self.save_now();
            }
            LcdIcon::Medicine => {
                let _ = self.pet.heal();
                self.save_now();
            }
            LcdIcon::Bath => {
                let _ = self.pet.clean();
                self.save_now();
            }
            LcdIcon::Meter => {
                self.show_meter = true;
                self.meter_page = 0;
            }
            LcdIcon::Discipline => {
                if self.pet.attention_call == AttentionCall::Crying {
                    let _ = self.pet.pet_love();
                } else {
                    let _ = self.pet.discipline_scold();
                }
                self.save_now();
            }
            LcdIcon::Attention => {}
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if self.offline_report.is_some() {
            if matches!(
                code,
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') | KeyCode::Char('q')
            ) {
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
                    self.meter_page = (self.meter_page + 1) % 4;
                }
                KeyCode::Left | KeyCode::Up => {
                    self.meter_page = (self.meter_page + 3) % 4;
                }
                KeyCode::Esc | KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('q') => {
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

    fn handle_pantry_key(&mut self, code: KeyCode) {
        let choices = FoodChoice::all();
        let count = choices.len();
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.pantry_selected_idx = (self.pantry_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                self.pantry_selected_idx = (self.pantry_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('f') | KeyCode::Char('F') => {
                let choice = choices[self.pantry_selected_idx];
                let _ = self.pet.feed_choice(choice);
                self.pantry_open = false;
                self.save_now();
            }
            KeyCode::Char(c @ '1'..='5') => {
                let idx = (c as usize) - ('1' as usize);
                if let Some(&choice) = choices.get(idx) {
                    let _ = self.pet.feed_choice(choice);
                    self.pantry_open = false;
                    self.save_now();
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
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
                    self.selected_species_idx =
                        (self.selected_species_idx + species_count - 1) % species_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.selected_species_idx = (self.selected_species_idx + 1) % species_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Char(' ') => {
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::Enter => {
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
                    self.selected_theme_idx =
                        (self.selected_theme_idx + theme_count - 1) % theme_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.selected_theme_idx = (self.selected_theme_idx + 1) % theme_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') => {
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::BackTab | KeyCode::Esc => {
                    self.onboarding_step = OnboardingStep::SelectSpecies;
                }
                KeyCode::Enter => {
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
        self.save_now();
    }

    fn handle_roster_key(&mut self, code: KeyCode) {
        let count = self.sanctuary.pets.len().max(1);
        match code {
            KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                self.roster_selected_idx = (self.roster_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Right | KeyCode::Char('j') | KeyCode::Char('l') => {
                self.roster_selected_idx = (self.roster_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.switch_to_roster_pet();
                self.screen = AppScreen::Habitat;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Esc | KeyCode::Char('L') | KeyCode::Char('q') => {
                self.screen = AppScreen::Habitat;
            }
            _ => {}
        }
    }

    fn handle_minigame_key(&mut self, code: KeyCode) {
        let finished = {
            let Some(game) = self.minigame.as_mut() else {
                self.screen = AppScreen::Habitat;
                return;
            };
            match code {
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('h') => {
                    game.move_left();
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('l') => {
                    game.move_right();
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    game.ticks_remaining = 0;
                }
                _ => {}
            }
            game.is_finished()
        };
        if finished {
            self.finish_minigame();
        }
    }

    fn finish_minigame(&mut self) {
        if let Some(game) = self.minigame.take() {
            if game.score > 0 || game.coins_earned > 0 || game.xp_earned > 0 {
                let _ = self
                    .pet
                    .apply_minigame_reward(game.score, game.coins_earned, game.xp_earned);
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
                self.selected_icon_idx = (self.selected_icon_idx + icon_count - 1) % icon_count;
            }
            KeyCode::Right | KeyCode::Char('.') | KeyCode::Char('a') | KeyCode::Char('A') => {
                self.selected_icon_idx = (self.selected_icon_idx + 1) % icon_count;
            }
            KeyCode::Enter | KeyCode::Char('b') | KeyCode::Char('B') => {
                self.activate_selected_lcd_icon();
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.show_meter = true;
                self.meter_page = 0;
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                let _ = self.pet.toggle_light();
                self.save_now();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                let _ = self.pet.discipline_scold();
                self.save_now();
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char(' ') => {
                if self.pet.stage == EvolutionStage::Egg {
                    let _ = self.pet.warm_egg();
                } else {
                    let _ = self.pet.pet_love();
                }
                self.save_now();
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('1') => {
                let _ = self.pet.feed();
                self.save_now();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.pet.stage == EvolutionStage::Egg {
                    let _ = self.pet.feed();
                } else {
                    self.pantry_open = true;
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char('2') => {
                let _ = self.pet.play();
                self.save_now();
            }
            KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('3') => {
                let _ = self.pet.pet_love();
                self.save_now();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('4') => {
                let _ = self.pet.train();
                self.save_now();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('5') => {
                let _ = self.pet.toggle_sleep();
                self.save_now();
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('6') => {
                let _ = self.pet.clean();
                self.save_now();
            }
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('7') => {
                let _ = self.pet.heal();
                self.save_now();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.launch_minigame();
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
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
                self.pet.theme = self.pet.theme.next();
                self.save_now();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

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
            self.sanctuary.sync_active(&self.pet);
            self.sanctuary.tick_decay_all();
            self.pet = self.sanctuary.active_pet().clone();
            if self.tick_count.is_multiple_of(DECAY_EVERY_TICKS * 4) {
                self.save_now();
            }
        }
    }
}

// ── Minimalist Retro Handheld Layout Helpers ───────────────────────────────

/// Compute a centered retro handheld LCD rectangle inside `area`.
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
        .padding(Padding::new(2, 2, 0, 0))
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
        .padding(Padding::new(2, 2, 0, 0))
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
    let mut spans = Vec::with_capacity(16);
    for &icon in LcdIcon::selectable() {
        let is_sel = icon == selected;
        let label = if icon == LcdIcon::Lights {
            if pet.light_on {
                "💡Light"
            } else {
                "🌙Dark"
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
        spans.push(Span::styled(format!(" {label} "), style));
        spans.push(Span::raw(" "));
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

    // Step-by-step screen routing: only ONE clean screen renders inside the
    // handheld device at any time!
    if let Some(report) = app.offline_report.as_ref() {
        draw_offline_report_screen(f, device, app, report);
    } else if app.show_help {
        draw_help_screen(f, device, app);
    } else if app.show_meter {
        draw_meter_screen(f, device, app);
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

/// Step-by-step Onboarding: Step 1 -> Step 2 -> Step 3 each have their own
/// dedicated minimalist retro screen!
fn draw_onboarding_step_screen(f: &mut Frame, device: Rect, app: &App) {
    let species = PetSpecies::all()[app.selected_species_idx];
    let theme = ShellTheme::all()[app.selected_theme_idx];
    let preview_stage = EvolutionStage::all()[app.preview_stage_idx % EvolutionStage::all().len()];
    let (sp_primary, _, sp_accent) = species_palette(species);

    let shell = outer_shell_block("🍡 MOCHIGOTCHI V-PET  •  NEW COMPANION", theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    match app.onboarding_step {
        OnboardingStep::SelectSpecies => {
            let lcd = lcd_screen_block("STEP 1 OF 3  •  CHOOSE COMPANION", sp_primary);
            let lcd_inner = lcd.inner(inner);
            f.render_widget(lcd, inner);

            let mut lines = vec![
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
                Line::from(Span::styled(
                    format!(
                        "{}  •  {}  •  Form: {}",
                        species.family(),
                        species.element_badge(),
                        preview_stage.badge()
                    ),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(""),
            ];
            lines.extend(render_colored_preview(
                species,
                preview_stage,
                app.tick_count,
            ));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                species.description().to_string(),
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(vec![
                Span::styled("Favorite Snack: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    species.favorite_snack().to_string(),
                    Style::default().fg(Color::LightGreen),
                ),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "[←/→ or ↑/↓] Browse   [V] Stage Preview   [Enter] Next Step",
                Style::default()
                    .fg(sp_accent)
                    .add_modifier(Modifier::BOLD),
            )));

            f.render_widget(
                Paragraph::new(Text::from(lines)).alignment(Alignment::Center),
                lcd_inner,
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
                "[↑/↓] Select Shell   [Enter] Next Step   [Esc] Back to Step 1",
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

            let mut lines = vec![
                Line::from(Span::styled(
                    format!(
                        "{}  •  Shell: {}",
                        species.display_name(),
                        theme.display_name()
                    ),
                    Style::default().fg(sp_primary).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];
            lines.extend(render_colored_preview(
                species,
                EvolutionStage::Egg,
                app.tick_count,
            ));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Enter a name for your new companion:",
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("   [  {}▏  ]   ", app.onboarding_name_input),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "[Enter] Adopt & Start   [Backspace] Edit   [Esc] Back to Step 2",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )));

            f.render_widget(
                Paragraph::new(Text::from(lines)).alignment(Alignment::Center),
                lcd_inner,
            );
        }
    }
}

/// Minimalist Retro Handheld LCD Habitat Screen.
fn draw_habitat_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let (sp_primary, _, _) = species_palette(pet.species);
    let selected_icon = LcdIcon::selectable()[app.selected_icon_idx % LcdIcon::selectable().len()];

    let shell_title = format!(
        "🍡 MOCHIGOTCHI  •  {} ({})",
        pet.name.to_uppercase(),
        pet.species.display_name()
    );
    let shell = outer_shell_block(&shell_title, pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(14),
            Constraint::Length(2),
        ])
        .split(inner);

    // 1. Top Bezel Icon Bar
    f.render_widget(
        Paragraph::new(render_unified_bezel(selected_icon, pet)).alignment(Alignment::Center),
        rows[0],
    );

    // 2. Center Retro LCD Screen (Pixel Art Sprite + minimal status strip)
    let (sky, _) = sky_scene(app.tick_count);
    let lcd_title = if pet.stage == EvolutionStage::Egg {
        format!(
            "{}  •  🥚 EGG ({}% WARM)  •  🔔 QUIET",
            sky, pet.incubation
        )
    } else {
        format!(
            "{}  •  {} LV.{}  •  {}G  •  💰{}  •  {}",
            sky,
            pet.stage.badge(),
            pet.level,
            pet.weight_grams,
            pet.coins,
            pet.attention_call.badge()
        )
    };
    let lcd = lcd_screen_block(&lcd_title, sp_primary);
    let lcd_inner = lcd.inner(rows[1]);
    f.render_widget(lcd, rows[1]);

    let mut lcd_lines = Vec::with_capacity(16);
    if !pet.light_on {
        lcd_lines.push(Line::from(Span::styled(
            "🌙 [ ROOM LIGHT OFF — NIGHT MODE ]",
            Style::default().fg(Color::LightBlue),
        )));
    } else if pet.has_toothache {
        lcd_lines.push(Line::from(Span::styled(
            "🦷 [ TOOTHACHE! PRESS [H] FOR MEDICINE ]",
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        lcd_lines.push(Line::from(Span::styled(
            speech_bubble(pet, app.tick_count),
            Style::default().fg(mood_color(pet.mood)),
        )));
    }

    lcd_lines.extend(render_colored_sprite(pet, app.tick_count));

    if pet.stage == EvolutionStage::Egg {
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        lcd_lines.push(Line::from(Span::styled(
            format!("Incubation: [{}] {}%", boxes.trim(), pet.incubation),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        lcd_lines.push(Line::from(vec![
            Span::styled(
                format!("🍙 {}   ", format_hearts(pet.hungry_hearts())),
                Style::default().fg(Color::LightGreen),
            ),
            Span::styled(
                format!("💖 {}   ", format_hearts(pet.happy_hearts())),
                Style::default().fg(Color::LightMagenta),
            ),
            Span::styled(
                format!("⚡ {}%", pet.energy),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    let latest_log = pet
        .action_log
        .last()
        .cloned()
        .unwrap_or_else(|| pet.advisor_tip(now));
    lcd_lines.push(Line::from(Span::styled(
        latest_log,
        Style::default().fg(Color::Gray),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lcd_lines)).alignment(Alignment::Center),
        lcd_inner,
    );

    // 3. Minimalist 2-line Footer (Advisor + Key hints)
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
                "[W/Space] Warm Egg  •  [I] Status Meter  •  [L] Sanctuary  •  [N] Adopt  •  [?] Keys",
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
                "[←/→+Enter] Bezel  [F] Meal  [M] Pantry  [P] Play  [K] Light  [I] Meter  [L] Pets  [?] Keys",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    };
    f.render_widget(
        Paragraph::new(Text::from(footer_lines)).alignment(Alignment::Center),
        rows[2],
    );
}

/// Step-by-step Retro Status Meter Screen (`[I]` or Bezel `📏Meter`).
fn draw_meter_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let shell = outer_shell_block("📏 TAMAGOTCHI STATUS METER", pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let title = if pet.stage == EvolutionStage::Egg {
        "🥚 EGG INCUBATOR STATUS".to_string()
    } else {
        format!(
            "📊 Tamagotchi Vitals  •  Page {}/4 (Space / ←/→)",
            (app.meter_page % 4) + 1
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
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Hungry, Happy, Discipline & Hygiene unlock after hatching!",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let xp_pct = (pet.xp_progress_ratio() * 100.0).round() as u8;
        let target_xp = pet.stage.next_stage().map_or(pet.xp, |n| n.min_xp());
        let disc_blocks = pet.discipline_blocks() as usize;
        let disc_bar = format!(
            "{}{}",
            "■".repeat(disc_blocks),
            "·".repeat(4usize.saturating_sub(disc_blocks))
        );

        lines.push(Line::from(Span::styled(
            format!(
                "{} ({})  •  {} Lv.{}  •  Age {}  •  Weight {}g (Base {}g)",
                pet.name,
                pet.species.display_name(),
                pet.stage.badge(),
                pet.level,
                format_duration_human(pet.age_seconds),
                pet.weight_grams,
                pet.stage.base_weight_grams()
            ),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        lines.push(custom_meter_line(
            "⭐ Stage XP",
            xp_pct,
            Color::LightMagenta,
            &format!("{}/{} XP", pet.xp, target_xp),
            Color::LightMagenta,
        ));
        lines.push(custom_meter_line(
            "🍙 Hungry",
            pet.hunger,
            Color::LightGreen,
            &format_hearts(pet.hungry_hearts()),
            Color::LightGreen,
        ));
        lines.push(custom_meter_line(
            "💖 Happy",
            pet.happiness,
            Color::LightMagenta,
            &format_hearts(pet.happy_hearts()),
            Color::LightMagenta,
        ));
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
            if pet.has_toothache {
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
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            pet.next_evolution_requirements(),
            Style::default().fg(Color::LightYellow),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Esc / I] Return to Habitat",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));

    f.render_widget(Paragraph::new(Text::from(lines)), lcd_inner);
}

/// Step-by-step Retro Pantry Screen (`[M]` or Bezel `🍙Food`).
fn draw_pantry_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("🍱 TAMAGOTCHI PANTRY  •  MEALS & SNACKS", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!("COINS: {} 💰  •  SWEET STREAK: {}/3", app.pet.coins, app.pet.snacks_in_a_row),
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

/// Step-by-step Welcome Back Offline Report Screen.
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

/// Step-by-step Help & Controls Screen (`[?]`).
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
        Line::from(""),
        Line::from(Span::styled(
            "🍙 Meals vs. Sweet Snacks & Weight (g)",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [F] Meal (+1 Hungry ♥, +1g) • [M] Pantry Menu"),
        Line::from(" • 3 Sweet Snacks in a row = Toothache 🦷 (cure with [H] Meds)"),
        Line::from(" • [P] Play / [E] Train burns -2g weight!"),
        Line::from(""),
        Line::from(Span::styled(
            "💡 Room Light, Discipline & Sanctuary",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [S] Sleep + [K] Turn Light OFF • [C] Bath • [D] Scold • [T] Praise"),
        Line::from(" • [I] Status Meter • [G] Arcade • [L] Sanctuary Pets • [N] Adopt New"),
        Line::from(""),
        Line::from(Span::styled(
            "[Esc / Enter / ?] Return to Habitat",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    ];

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        lcd_inner,
    );
}

/// Step-by-step Sanctuary Roster Screen (`[L]`): browse pets one card at a time!
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

    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "{}  •  {}  •  {} Lv.{} ({}g)",
                highlighted.name,
                highlighted.species.display_name(),
                highlighted.stage.badge(),
                highlighted.level,
                highlighted.weight_grams
            ),
            Style::default().fg(h_primary).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    lines.extend(render_colored_sprite(highlighted, app.tick_count));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(
            "Hungry: {}   Happy: {}   Incubation: {}%",
            format_hearts(highlighted.hungry_hearts()),
            format_hearts(highlighted.happy_hearts()),
            highlighted.incubation
        ),
        Style::default().fg(Color::White),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[←/→ or ↑/↓] Browse Pets   [Enter] Switch Active   [N] Adopt New   [Esc] Back",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).alignment(Alignment::Center),
        lcd_inner,
    );
}

/// Step-by-step Snack Catcher Arcade Screen (`[G]`).
fn draw_minigame_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("🎮 SNACK CATCHER ARCADE", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let (arena, status) = match app.minigame.as_ref() {
        Some(game) => {
            let lines: Vec<Line> = game
                .render_lines()
                .into_iter()
                .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::LightGreen))))
                .collect();
            (
                Text::from(lines),
                format!(
                    "SCORE {}  •  COINS +{}  •  XP +{}  •  TIME {}",
                    game.score, game.coins_earned, game.xp_earned, game.ticks_remaining
                ),
            )
        }
        None => (Text::from(""), "no game running".to_string()),
    };

    let lcd = lcd_screen_block(&status, Color::LightGreen);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(2)])
        .split(lcd_inner);

    f.render_widget(
        Paragraph::new(arena).alignment(Alignment::Center),
        rows[0],
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[←/→ or A/D] Move Basket   •   [Esc / Q] Finish & Claim Rewards",
            Style::default().fg(Color::LightYellow),
        )))
        .alignment(Alignment::Center),
        rows[1],
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
    fn onboarding_adopts_dragon_then_a_second_pet() {
        let paths = tmp_paths("adopt");
        let _ = std::fs::remove_dir_all(paths.json_path.parent().unwrap());

        let mut app = App::new_onboarding(paths.clone());
        assert_eq!(app.screen, AppScreen::Onboarding);

        for _ in 0..4 {
            app.handle_key(KeyCode::Down);
        }
        app.handle_key(KeyCode::Char('v'));
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
        assert_eq!(app.sanctuary.pets.len(), 1);

        app.handle_key(KeyCode::Char('n'));
        for _ in 0..5 {
            app.handle_key(KeyCode::Down);
        }
        app.handle_key(KeyCode::Enter);
        app.handle_key(KeyCode::Enter);
        app.handle_key(KeyCode::Enter);

        assert_eq!(app.sanctuary.pets.len(), 2);
        assert_eq!(app.pet.species, PetSpecies::AzureWyrm);
        assert_eq!(app.sanctuary.active_index, 1);

        app.handle_key(KeyCode::Char('['));
        assert_eq!(app.pet.species, PetSpecies::IgnisDragon);

        app.handle_key(KeyCode::Char('l'));
        assert_eq!(app.screen, AppScreen::SanctuaryRoster);
        app.handle_key(KeyCode::Down);
        app.handle_key(KeyCode::Enter);
        assert_eq!(app.screen, AppScreen::Habitat);
        assert_eq!(app.pet.species, PetSpecies::AzureWyrm);

        let _ = std::fs::remove_dir_all(paths.json_path.parent().unwrap());
    }

    #[test]
    fn renders_resize_guard_on_small_terminal_and_full_ui_on_standard_and_wide() {
        for (w, h) in [(54u16, 18u16), (68, 22), (96, 28), (140, 44)] {
            let backend = TestBackend::new(w, h);
            let mut terminal = Terminal::new(backend).unwrap();
            let paths = tmp_paths(&format!("render{w}"));
            let mut app = App::new(Pet::new("Mochi"), paths);
            app.pantry_open = true;
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
            app.pantry_open = false;
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
    fn every_screen_and_wizard_step_renders() {
        let paths = tmp_paths("screens");
        let mut app = App::new(Pet::new("Mochi"), paths.clone());
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        for screen in [
            AppScreen::Habitat,
            AppScreen::SanctuaryRoster,
            AppScreen::MiniGame,
            AppScreen::Onboarding,
        ] {
            app.screen = screen;
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
        }

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
