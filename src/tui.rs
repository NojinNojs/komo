//! The Ratatui interface for **Mochigotchi v0.5.2**:
//! - Minimum terminal size guard (`MIN_TERM_WIDTH = 80`, `MIN_TERM_HEIGHT = 24`)
//! - Unified single-row 7-icon Bandai Tamagotchi LCD Bezel (`🍙Food`, `💡Light`,
//!   `🎮Game`, `💉Meds`, `🚽Bath`, `📏Meter`, `📢Scold`) inside the Care Console
//! - High-contrast custom `[████████░░]` vital meters (with a dedicated
//!   `🥚 Egg Incubator` panel while unhatched so eggs don't show adult hunger/poop
//!   bars, and full `📊 Tamagotchi Vitals` once hatched)
//! - Vertically & horizontally centered TrueColor ASCII sprites with zero
//!   text truncation or wrapping bugs.

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
    species_palette, stage_color, theme_color,
};
use crate::storage::{save_sanctuary, StoragePaths};

const TICK_MS: u64 = 250;
/// In-game stat decay tick interval: 24 ticks * 250ms = 6 seconds.
const DECAY_EVERY_TICKS: usize = 24;

/// Minimum terminal dimensions required for a clean, unclipped layout.
pub const MIN_TERM_WIDTH: u16 = 80;
pub const MIN_TERM_HEIGHT: u16 = 24;

/// Responsive layout tiers: Compact (<80, triggers resize notice) < Standard < Wide.
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
        } else if width < 112 {
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
                self.minigame = Some(MiniGame::new(38, 12, 120));
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
            KeyCode::Up | KeyCode::Char('k') => {
                self.roster_selected_idx = (self.roster_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.roster_selected_idx = (self.roster_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.switch_to_roster_pet();
                self.screen = AppScreen::Habitat;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Esc | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Char('q') => {
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

// ── Rendering ──────────────────────────────────────────────────────────────

fn shell_block(title: &str, theme: ShellTheme) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme_color(theme)))
        .padding(Padding::new(1, 1, 0, 0))
        .title(Span::styled(
            title.to_string(),
            Style::default()
                .fg(theme_color(theme))
                .add_modifier(Modifier::BOLD),
        ))
}

fn padded_block(title: &str, border_color: Color, active: bool) -> Block<'static> {
    let title_style = if active {
        Style::default()
            .fg(border_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(1, 1, 0, 0))
        .title(Span::styled(title.to_string(), title_style))
}

/// Crisp, high-contrast 1-line custom progress meter (`[████████░░]  80%  suffix`).
/// Eliminates the washed-out text-over-bar contrast issue of Ratatui's `Gauge`.
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Unified single-row 7-icon Tamagotchi LCD Bezel so `←` / `→` moves across one
/// horizontal bar inside the Care Console.
fn render_unified_bezel(selected: LcdIcon, pet: &Pet) -> Line<'static> {
    let mut spans = vec![Span::styled(
        "📟 ",
        Style::default().fg(Color::LightYellow),
    )];
    for &icon in LcdIcon::selectable() {
        let is_sel = icon == selected;
        let label = if icon == LcdIcon::Lights {
            if pet.light_on {
                "💡Light:ON"
            } else {
                "🌙Light:OFF"
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
    let block = shell_block(" 📐 Mochigotchi — Please Enlarge Terminal Window ", theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let w_ok = area.width >= MIN_TERM_WIDTH;
    let h_ok = area.height >= MIN_TERM_HEIGHT;

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🍡 Window Size Notice",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Current terminal size:  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} cols × {} rows", area.width, area.height),
                Style::default()
                    .fg(if w_ok && h_ok {
                        Color::LightGreen
                    } else {
                        Color::LightRed
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Minimum required size:  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{MIN_TERM_WIDTH} cols × {MIN_TERM_HEIGHT} rows"),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Please widen or maximize this terminal window so the LCD Bezel,",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "TrueColor ASCII sprites, and status meters can render cleanly.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press [Q] or [Esc] to save & exit",
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

    match app.screen {
        AppScreen::Onboarding => draw_onboarding(f, app),
        AppScreen::SanctuaryRoster => draw_roster(f, app),
        AppScreen::MiniGame => draw_minigame(f, app),
        AppScreen::Habitat => draw_habitat(f, app),
    }

    if app.pantry_open {
        draw_pantry_modal(f, app);
    }
    if app.show_meter {
        draw_meter_modal(f, app);
    }
    if let Some(report) = app.offline_report.as_ref() {
        draw_offline_report_modal(f, app, report);
    }
    if app.show_help {
        draw_help_modal(f, app);
    }
}

fn draw_onboarding(f: &mut Frame, app: &App) {
    let area = f.area();
    let species = PetSpecies::all()[app.selected_species_idx];
    let theme = ShellTheme::all()[app.selected_theme_idx];
    let preview_stage = EvolutionStage::all()[app.preview_stage_idx % EvolutionStage::all().len()];
    let (sp_primary, _, sp_accent) = species_palette(species);

    let block = shell_block(" 🍡 Mochigotchi — Companion Adoption Wizard ", theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let current = match app.onboarding_step {
        OnboardingStep::SelectSpecies => 1,
        OnboardingStep::SelectTheme => 2,
        OnboardingStep::EnterName => 3,
    };

    let steps = [
        "1. Choose Species",
        "2. Choose Shell",
        "3. Name & Adopt",
    ];
    let mut step_spans: Vec<Span> = Vec::new();
    for (i, label) in steps.iter().enumerate() {
        let is_cur = i + 1 == current;
        let is_done = i + 1 < current;
        let style = if is_cur {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        } else if is_done {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        step_spans.push(Span::styled(format!(" {label} "), style));
        if i + 1 < steps.len() {
            step_spans.push(Span::styled(" ──▸ ", Style::default().fg(Color::DarkGray)));
        }
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(14),
            Constraint::Length(2),
        ])
        .split(inner);
    f.render_widget(
        Paragraph::new(Line::from(step_spans)).alignment(Alignment::Center),
        rows[0],
    );

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(rows[1]);

    let left_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(6)])
        .split(columns[0]);

    let step1_active = app.onboarding_step == OnboardingStep::SelectSpecies;
    let items: Vec<ListItem> = PetSpecies::all()
        .iter()
        .enumerate()
        .map(|(idx, sp)| {
            let selected = idx == app.selected_species_idx;
            let (p_col, _, _) = species_palette(*sp);
            let cursor = if selected {
                if step1_active {
                    "▸"
                } else {
                    "•"
                }
            } else {
                " "
            };
            let name_style = if selected {
                Style::default().fg(p_col).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            let star_style = if selected {
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{cursor} {:<16} ", sp.display_name()), name_style),
                Span::styled(sp.star_bar().to_string(), star_style),
            ]))
        })
        .collect();

    let list_border = if step1_active {
        Color::LightYellow
    } else {
        Color::DarkGray
    };
    f.render_widget(
        List::new(items).block(padded_block(
            " 1. Companions (↑/↓) ",
            list_border,
            step1_active,
        )),
        left_rows[0],
    );

    let lore_lines = vec![
        Line::from(vec![
            Span::styled("Family:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} ({})", species.family(), species.rarity()),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Element:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                species.element_badge().to_string(),
                Style::default().fg(sp_accent),
            ),
        ]),
        Line::from(vec![
            Span::styled("Favorite: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                species.favorite_snack().to_string(),
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            species.description().to_string(),
            Style::default().fg(Color::Gray),
        )),
    ];
    f.render_widget(
        Paragraph::new(Text::from(lore_lines))
            .block(padded_block(" Companion Dossier ", sp_primary, true))
            .wrap(Wrap { trim: false }),
        left_rows[1],
    );

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(8)])
        .split(columns[1]);

    let mut preview_lines = vec![Line::from(vec![
        Span::styled(
            format!("Form: {} ", preview_stage.badge()),
            Style::default()
                .fg(stage_color(preview_stage))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("([V/Space] cycle)", Style::default().fg(Color::DarkGray)),
    ])];
    preview_lines.extend(render_colored_preview(
        species,
        preview_stage,
        app.tick_count,
    ));

    f.render_widget(
        Paragraph::new(Text::from(preview_lines))
            .alignment(Alignment::Center)
            .block(padded_block(" TrueColor Sprite Preview ", sp_accent, true)),
        right_rows[0],
    );

    let (step_title, step_border, step_active, detail_lines) = match app.onboarding_step {
        OnboardingStep::SelectSpecies => (
            " Next Step ",
            Color::DarkGray,
            false,
            vec![
                Line::from(Span::styled(
                    "Browse all 10 companions with ↑/↓.",
                    Style::default().fg(Color::White),
                )),
                Line::from(Span::styled(
                    "Press [V] or [Space] to preview all 5 growth stages.",
                    Style::default().fg(Color::LightCyan),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "▸ Press [Enter] to choose Shell Color!",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )),
            ],
        ),
        OnboardingStep::SelectTheme => {
            let mut lines = vec![Line::from(Span::styled(
                "Choose Shell Color (↑/↓, Enter):",
                Style::default().fg(Color::White),
            ))];
            for (idx, t) in ShellTheme::all().iter().enumerate() {
                let selected = idx == app.selected_theme_idx;
                lines.push(Line::from(Span::styled(
                    format!(" {} {}", if selected { "▸" } else { " " }, t.display_name()),
                    if selected {
                        Style::default()
                            .fg(theme_color(*t))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                )));
            }
            (" 2. Shell Color (Active) ", Color::LightYellow, true, lines)
        }
        OnboardingStep::EnterName => (
            " 3. Name & Adopt (Active) ",
            Color::LightYellow,
            true,
            vec![
                Line::from(Span::styled(
                    "Type a name for your companion:",
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Name: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("▏{}▕", app.onboarding_name_input),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "▸ Press [Enter] to Adopt!",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )),
            ],
        ),
    };

    f.render_widget(
        Paragraph::new(Text::from(detail_lines))
            .block(padded_block(step_title, step_border, step_active))
            .wrap(Wrap { trim: false }),
        right_rows[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "↑/↓ Select • [V/Space] Cycle Stage • [Enter] Next / Adopt • [Esc] Back",
            Style::default().fg(Color::Gray),
        )))
        .alignment(Alignment::Center),
        rows[2],
    );
}

fn draw_habitat(f: &mut Frame, app: &App) {
    let area = f.area();
    let pet = &app.pet;
    let now = Utc::now();
    let (sp_primary, _, _) = species_palette(pet.species);
    let selected_icon = LcdIcon::selectable()[app.selected_icon_idx % LcdIcon::selectable().len()];

    let title = format!(
        " 🍡 {} · {} · {} · Lv.{} · {}g ",
        pet.name,
        pet.species.display_name(),
        pet.stage.badge(),
        pet.level,
        pet.weight_grams
    );
    let block = shell_block(&title, pet.theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(6),
            Constraint::Length(4),
        ])
        .split(inner);

    let header_area = vertical[0];
    let body_area = vertical[1];
    let dock_area = vertical[2];
    let log_area = vertical[3];

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(48), Constraint::Percentage(52)])
        .split(body_area);

    let (sky, mood_name) = sky_scene(app.tick_count);
    let call_style = if pet.attention_call.is_active() {
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let header_lines = vec![
        Line::from(vec![
            Span::styled(
                format!("{sky} ({mood_name})   "),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                format!(
                    "🏰 Sanctuary: {} pets (Lv.{})   ",
                    app.sanctuary.pets.len(),
                    app.sanctuary.sanctuary_level
                ),
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("💰 {} coins   ", pet.coins),
                Style::default().fg(Color::LightYellow),
            ),
            Span::styled(pet.attention_call.badge().to_string(), call_style),
        ]),
        Line::from(vec![
            Span::styled(
                "💡 Advisor: ",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(pet.advisor_tip(now), Style::default().fg(Color::White)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(Text::from(header_lines)).wrap(Wrap { trim: false }),
        header_area,
    );

    // Left column: Habitat box with vertically + horizontally centered sprite
    let mut raw_sprite_lines = vec![
        Line::from(Span::styled(
            speech_bubble(pet, app.tick_count),
            Style::default()
                .fg(mood_color(pet.mood))
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
    ];
    if !pet.light_on {
        raw_sprite_lines.push(Line::from(Span::styled(
            "🌙 [Room Light OFF — Cozy Night Mode]",
            Style::default().fg(Color::LightBlue),
        )));
    }
    if pet.has_toothache {
        raw_sprite_lines.push(Line::from(Span::styled(
            "🦷 [TOOTHACHE! Press [H] for Medicine!]",
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )));
    }
    raw_sprite_lines.extend(render_colored_sprite(pet, app.tick_count));
    raw_sprite_lines.push(Line::from(""));
    raw_sprite_lines.push(Line::from(vec![
        Span::styled(
            format!("{} {} ", pet.mood.emoji(), pet.mood.label()),
            Style::default()
                .fg(mood_color(pet.mood))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("• Age {}", format_duration_human(pet.age_seconds)),
            Style::default().fg(Color::Gray),
        ),
    ]));

    let hab_block = padded_block(
        &format!(" {} Habitat ", pet.species.element_badge()),
        sp_primary,
        true,
    );
    let hab_inner = hab_block.inner(columns[0]);
    f.render_widget(hab_block, columns[0]);

    let content_h = raw_sprite_lines.len() as u16;
    let top_pad = hab_inner.height.saturating_sub(content_h) / 2;
    let mut centered_sprite = Vec::with_capacity((top_pad + content_h) as usize);
    for _ in 0..top_pad {
        centered_sprite.push(Line::from(""));
    }
    centered_sprite.extend(raw_sprite_lines);

    f.render_widget(
        Paragraph::new(Text::from(centered_sprite)).alignment(Alignment::Center),
        hab_inner,
    );

    // Right column: Dedicated Egg Incubator when Egg, or Tamagotchi Vitals when hatched
    let egg = pet.stage == EvolutionStage::Egg;
    if egg {
        let warm_cd = pet.warm_cooldown_remaining(now);
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let step_boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        let cd_text = if warm_cd == 0 {
            "Ready now! Press [W] or [Space] (+20%)".to_string()
        } else {
            format!("Next manual warm in {}", format_duration_human(warm_cd))
        };

        let inc_lines = vec![
            Line::from(""),
            custom_meter_line(
                "🥚 Incubation",
                pet.incubation,
                Color::LightYellow,
                if pet.incubation >= 80 {
                    "Almost Ready!"
                } else {
                    "Warming..."
                },
                Color::LightYellow,
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔥 Warmings   ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("[{}] ", step_boxes.trim()),
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{warm_sessions}/5 sessions"),
                    Style::default().fg(Color::Gray),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⏳ Warm Timer ", Style::default().fg(Color::White)),
                Span::styled(cd_text, Style::default().fg(Color::LightCyan)),
            ]),
            Line::from(""),
            custom_meter_line(
                "🤝 Keeper Bond",
                pet.bond,
                Color::LightBlue,
                "Growing",
                Color::LightBlue,
            ),
            Line::from(""),
            Line::from(Span::styled(
                "💡 Hungry, Happy, Discipline, Light & Bath",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "   meters unlock as soon as the egg hatches!",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        f.render_widget(
            Paragraph::new(Text::from(inc_lines)).block(padded_block(
                " 🥚 Egg Incubator Status ",
                Color::LightYellow,
                true,
            )),
            columns[1],
        );
    } else {
        let xp_pct = (pet.xp_progress_ratio() * 100.0).round() as u8;
        let target_xp = pet.stage.next_stage().map_or(pet.xp, |n| n.min_xp());
        let disc_blocks = pet.discipline_blocks() as usize;
        let disc_bar = format!(
            "{}{}",
            "■".repeat(disc_blocks),
            "·".repeat(4usize.saturating_sub(disc_blocks))
        );

        let vital_lines = vec![
            custom_meter_line(
                "⭐ Stage XP",
                xp_pct,
                Color::LightMagenta,
                &format!("{}/{} XP", pet.xp, target_xp),
                Color::LightMagenta,
            ),
            Line::from(""),
            custom_meter_line(
                "🍙 Hungry",
                pet.hunger,
                Color::LightGreen,
                &format_hearts(pet.hungry_hearts()),
                Color::LightGreen,
            ),
            custom_meter_line(
                "💖 Happy",
                pet.happiness,
                Color::LightMagenta,
                &format_hearts(pet.happy_hearts()),
                Color::LightMagenta,
            ),
            custom_meter_line(
                "📢 Discipline",
                pet.discipline,
                Color::LightYellow,
                &format!("{disc_bar} {}/4", disc_blocks),
                Color::LightYellow,
            ),
            Line::from(""),
            custom_meter_line(
                "⚡ Energy",
                pet.energy,
                Color::Yellow,
                if pet.is_sleeping { "Sleeping" } else { "Awake" },
                Color::Yellow,
            ),
            custom_meter_line(
                "🚽 Hygiene",
                pet.cleanliness,
                Color::LightCyan,
                &format!("Poop: {}", pet.poop_count),
                if pet.poop_count > 0 {
                    Color::LightRed
                } else {
                    Color::LightCyan
                },
            ),
            custom_meter_line(
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
                if pet.is_sick || pet.has_toothache {
                    Color::LightRed
                } else {
                    Color::LightGreen
                },
            ),
            custom_meter_line(
                "🤝 Bond",
                pet.bond,
                Color::LightBlue,
                &format!("Mistakes: {}", pet.care_mistakes),
                Color::Gray,
            ),
        ];

        f.render_widget(
            Paragraph::new(Text::from(vital_lines)).block(padded_block(
                " 📊 Tamagotchi Vitals & Hearts ",
                Color::LightCyan,
                true,
            )),
            columns[1],
        );
    }

    // Bottom Care Console with unified 1-row LCD Bezel
    let mut dock_lines = vec![
        render_unified_bezel(selected_icon, pet),
        Line::from(Span::styled(
            pet.next_evolution_requirements(),
            Style::default().fg(Color::LightYellow),
        )),
    ];
    let keys = if let Some(buffer) = app.rename_buffer.as_ref() {
        vec![Line::from(Span::styled(
            format!("🔖 Rename to: {buffer}▏  ([Enter] save, [Esc] cancel)"),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))]
    } else if egg {
        vec![
            Line::from(vec![
                Span::styled(
                    "[W/Space] Warm (+20%)  ",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("[N] Adopt Pet  ", Style::default().fg(Color::LightCyan)),
                Span::styled("[L] Sanctuary  ", Style::default().fg(Color::LightBlue)),
                Span::styled("[[/]] Switch Pet", Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled(
                    "←/→ Bezel • [Enter] Confirm • [I] Meter • [?] Help • [Q] Quit  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    app.last_storage_status.clone(),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled(
                    "[F] Meal ",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("[M] Pantry ", Style::default().fg(Color::LightYellow)),
                Span::styled("[K] Light ", Style::default().fg(Color::Cyan)),
                Span::styled("[P] Play ", Style::default().fg(Color::LightMagenta)),
                Span::styled("[D] Scold ", Style::default().fg(Color::LightRed)),
                Span::styled("[T] Praise ", Style::default().fg(Color::LightCyan)),
                Span::styled("[E] Train ", Style::default().fg(Color::Yellow)),
                Span::styled("[S] Sleep ", Style::default().fg(Color::LightBlue)),
                Span::styled("[C] Bath ", Style::default().fg(Color::Cyan)),
                Span::styled("[H] Meds ", Style::default().fg(Color::Red)),
                Span::styled("[I] Meter ", Style::default().fg(Color::White)),
                Span::styled("[G] Arcade", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled(
                    "←/→ Bezel + [Enter] • [N] Adopt • [L] Sanctuary • [[/]] Switch • [?] Help • [Q] Quit ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    app.last_storage_status.clone(),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ]
    };
    dock_lines.extend(keys);

    f.render_widget(
        Paragraph::new(Text::from(dock_lines)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme_color(pet.theme)))
                .padding(Padding::new(1, 1, 0, 0))
                .title(" 📟 LCD Bezel & Controls (←/→ Select • Enter Confirm) "),
        ),
        dock_area,
    );

    let log_lines: Vec<Line> = pet
        .action_log
        .iter()
        .rev()
        .take(2)
        .map(|msg| Line::from(Span::styled(msg.clone(), Style::default().fg(Color::Gray))))
        .collect();
    f.render_widget(
        Paragraph::new(Text::from(log_lines)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .padding(Padding::new(1, 1, 0, 0))
                .title(" 📜 Care Diary "),
        ),
        log_area,
    );
}

fn draw_meter_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(66, 56, f.area());
    f.render_widget(Clear, area);
    let pet = &app.pet;

    let block = padded_block(
        &format!(" 📏 Health Meter — Page {}/4 (←/→ or Space) ", app.meter_page + 1),
        Color::LightCyan,
        true,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let page_lines = match app.meter_page % 4 {
        0 => vec![
            Line::from(Span::styled(
                "📄 PAGE 1: AGE, WEIGHT & STAGE",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!(" • Companion: {} ({})", pet.name, pet.species.display_name())),
            Line::from(format!(" • Stage:     {} (Lv.{} • {} XP)", pet.stage.badge(), pet.level, pet.xp)),
            Line::from(format!(" • Real Age:  {}", format_duration_human(pet.age_seconds))),
            Line::from(format!(
                " • Weight:    {}g (Base Weight: {}g{})",
                pet.weight_grams,
                pet.stage.base_weight_grams(),
                if pet.is_overweight() {
                    " — Overweight! Exercise with [P]/[E]"
                } else {
                    " — Healthy!"
                }
            )),
        ],
        1 => {
            let blocks = pet.discipline_blocks() as usize;
            let bar = format!(
                "{}{}",
                "■ ".repeat(blocks),
                "□ ".repeat(4usize.saturating_sub(blocks))
            );
            vec![
                Line::from(Span::styled(
                    "📄 PAGE 2: DISCIPLINE & CARE MISTAKES",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!(" • Discipline:    [{}] ({}%)", bar.trim(), pet.discipline)),
                Line::from(format!(" • Care Mistakes: {} recorded", pet.care_mistakes)),
                Line::from(format!(" • Attention Call: {}", pet.attention_call.badge())),
                Line::from(""),
                Line::from(" 💡 Scold [D] when acting spoiled; Praise [T] when crying!"),
            ]
        }
        2 => vec![
            Line::from(Span::styled(
                "📄 PAGE 3: HUNGRY HEARTS & DIGESTION",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!(
                " • Hungry Hearts: {}  ({}%)",
                format_hearts(pet.hungry_hearts()),
                pet.hunger
            )),
            Line::from(format!(
                " • Sweet Streak:  {}/3 snacks in a row {}",
                pet.snacks_in_a_row,
                if pet.has_toothache {
                    "(🦷 TOOTHACHE!)"
                } else {
                    "(Safe)"
                }
            )),
            Line::from(format!(
                " • Floor Hygiene: {}% Clean (Poop: {}/4)",
                pet.cleanliness, pet.poop_count
            )),
        ],
        _ => vec![
            Line::from(Span::styled(
                "📄 PAGE 4: HAPPY HEARTS, BOND & SLEEP",
                Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!(
                " • Happy Hearts:   {}  ({}%)",
                format_hearts(pet.happy_hearts()),
                pet.happiness
            )),
            Line::from(format!(" • Companion Bond: {}%", pet.bond)),
            Line::from(format!(
                " • Room Light:     {}",
                if pet.light_on { "💡 ON" } else { "🌙 OFF" }
            )),
            Line::from(format!(" • Coins Purse:    {} 💰", pet.coins)),
        ],
    };

    let mut lines = page_lines;
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press [Space/→] for next page • [Esc] or [I] to close",
        Style::default().fg(Color::Cyan),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        inner,
    );
}

fn draw_pantry_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(68, 58, f.area());
    f.render_widget(Clear, area);

    let block = padded_block(
        &format!(
            " 🍱 Tamagotchi Pantry (Meals vs. Snacks) — Coins: {} 💰 ",
            app.pet.coins
        ),
        Color::LightYellow,
        true,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

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
                format!("      {}", choice.summary()),
                Style::default().fg(Color::Gray),
            )),
        ]));
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(2)])
        .split(inner);

    f.render_widget(List::new(items), rows[0]);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "↑/↓ or [1-5] Select • [Enter] Feed • [Esc] Close",
            Style::default().fg(Color::Cyan),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );
}

fn draw_offline_report_modal(f: &mut Frame, app: &App, report: &OfflineReport) {
    let area = centered_rect(72, 58, f.area());
    f.render_widget(Clear, area);

    let block = padded_block(
        &format!(
            " 🌙 Welcome Back! Away for {} ",
            report.formatted_duration()
        ),
        Color::LightCyan,
        true,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "While the terminal was closed, {} lived on in the Sanctuary:",
                report.pet_name
            ),
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
        format!("💡 Next step: {}", app.pet.advisor_tip(Utc::now())),
        Style::default().fg(Color::LightGreen),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press [Enter] or [Space] to continue",
        Style::default().fg(Color::Cyan),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        inner,
    );
}

fn draw_help_modal(f: &mut Frame, _app: &App) {
    let area = centered_rect(78, 76, f.area());
    f.render_widget(Clear, area);

    let block = padded_block(
        " 📖 Tamagotchi Hardware & Care Guide ",
        Color::LightMagenta,
        true,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled(
            "📟 1. 7-Icon LCD Bezel & 3-Button Controls",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("   • Button A (←/→ or A): Cycle across the 7 LCD Bezel Icons."),
        Line::from("   • Button B (Enter or B): Activate the highlighted Bezel Icon."),
        Line::from("   • Button C (Esc): Cancel out of any menu or modal."),
        Line::from(""),
        Line::from(Span::styled(
            "🍙 2. Meals vs. Sweet Snacks, Weight (g) & Toothache (🦷)",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("   • Meals ([F] or Pantry #1/#2/#4) fill Hungry Hearts (+1..2g) and refuse when 4/4 full."),
        Line::from("   • Sweet Snacks (Pantry #3/#5) fill Happy Hearts (+2..3g), but 3 in a row cause a Toothache 🦷!"),
        Line::from("   • Play [P] or Train [E] to burn weight (-2g) and keep your companion fit!"),
        Line::from(""),
        Line::from(Span::styled(
            "💡 3. Room Light ([K]), Discipline ([D]/[T]) & Care Mistakes",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("   • When sleeping [S], press [K] to turn the Light OFF! Leaving it on causes a Care Mistake."),
        Line::from("   • If 🔔 CALL says 'Scold [D]', press [D] (+25% Discipline). If 'Praise [T]', press [T]!"),
        Line::from("   • Press [I] anytime to flip through the 4-Screen Bandai Health Meter!"),
        Line::from(""),
        Line::from(Span::styled(
            "Press [Esc], [Enter], or [?] to close",
            Style::default().fg(Color::Gray),
        )),
    ];

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        inner,
    );
}

fn draw_roster(f: &mut Frame, app: &App) {
    let area = f.area();
    let block = shell_block(
        &format!(
            " 🏰 Sanctuary — {} companions · Lv.{} ",
            app.sanctuary.pets.len(),
            app.sanctuary.sanctuary_level
        ),
        app.pet.theme,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(inner);

    let items: Vec<ListItem> = app
        .sanctuary
        .pets
        .iter()
        .enumerate()
        .map(|(idx, pet)| {
            let selected = idx == app.roster_selected_idx;
            let (p_col, _, _) = species_palette(pet.species);
            let marker = if idx == app.sanctuary.active_index {
                "★"
            } else if selected {
                "▸"
            } else {
                " "
            };
            let style = if selected {
                Style::default().fg(p_col).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(
                format!(
                    "{marker} {}. {:<10} {:<15} {} Lv.{} · {}g",
                    idx + 1,
                    pet.name,
                    pet.species.display_name(),
                    pet.stage.badge(),
                    pet.level,
                    pet.weight_grams,
                ),
                style,
            )))
        })
        .collect();
    f.render_widget(
        List::new(items).block(padded_block(
            " Roster (↑/↓ select • [Enter] switch • [N] adopt) ",
            Color::LightYellow,
            true,
        )),
        columns[0],
    );

    let highlighted = app
        .sanctuary
        .pets
        .get(app.roster_selected_idx)
        .unwrap_or(&app.pet);
    let (h_primary, _, _) = species_palette(highlighted.species);
    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "{} · {} · {}",
                highlighted.name,
                highlighted.species.display_name(),
                highlighted.species.rarity()
            ),
            Style::default().fg(h_primary).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "{} Lv.{} ({}g) • Hungry {} • Happy {}",
                highlighted.stage.badge(),
                highlighted.level,
                highlighted.weight_grams,
                format_hearts(highlighted.hungry_hearts()),
                format_hearts(highlighted.happy_hearts())
            ),
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];
    lines.extend(render_colored_sprite(highlighted, app.tick_count));
    f.render_widget(
        Paragraph::new(Text::from(lines))
            .alignment(Alignment::Center)
            .block(padded_block(" Companion Card ", h_primary, true)),
        columns[1],
    );
}

fn draw_minigame(f: &mut Frame, app: &App) {
    let area = f.area();
    let block = shell_block(" 🎮 Snack Catcher Arcade ", app.pet.theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(3)])
        .split(inner);

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
                    "Score {} · Coins +{} · XP +{} · Time left {}",
                    game.score, game.coins_earned, game.xp_earned, game.ticks_remaining
                ),
            )
        }
        None => (Text::from(""), "no game running".to_string()),
    };

    f.render_widget(
        Paragraph::new(arena)
            .alignment(Alignment::Center)
            .block(padded_block(
                " Catch snacks & stars! Avoid [X]! ",
                Color::LightGreen,
                true,
            )),
        rows[0],
    );
    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(status, Style::default().fg(Color::LightYellow))),
            Line::from(Span::styled(
                "←/→ or A/D to move basket · Esc/Q to finish & claim rewards",
                Style::default().fg(Color::Gray),
            )),
        ]))
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
        for (w, h) in [(60u16, 20u16), (80, 24), (104, 30), (140, 44)] {
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
        let backend = TestBackend::new(120, 40);
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
