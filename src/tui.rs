//! The Ratatui interface: adoption wizard, LCD habitat, Sanctuary roster and
//! the Snack Catcher arcade.

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::minigame::MiniGame;
use crate::pet::{EvolutionStage, Pet, PetSpecies, Sanctuary, ShellTheme};
use crate::sprites::{
    get_mood_particles, mood_color, render_animated_sprite, render_species_preview, sky_scene,
    speech_bubble, stage_color, theme_color,
};
use crate::storage::{save_sanctuary, StoragePaths};

const TICK_MS: u64 = 250;
const DECAY_EVERY_TICKS: usize = 20;

/// Responsive layout tiers: Compact < Standard < Wide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutTier {
    Compact,
    Standard,
    Wide,
}

impl LayoutTier {
    pub fn from_width(width: u16) -> Self {
        if width < 78 {
            Self::Compact
        } else if width < 110 {
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
    pub roster_selected_idx: usize,
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
            roster_selected_idx: 0,
            onboarding_name_input: String::new(),
            adopting_new: false,
            tick_count: 0,
            should_quit: false,
            minigame: None,
            rename_buffer: None,
            last_storage_status: String::new(),
        }
    }

    pub fn new_onboarding(paths: StoragePaths) -> Self {
        let mut app = Self::new_with_sanctuary(Sanctuary::new_with_pet(Pet::new("Mochi")), paths, true);
        app.onboarding_name_input = "Mochi".to_string();
        app
    }

    pub fn save_now(&mut self) {
        self.sanctuary.sync_active(&self.pet);
        match save_sanctuary(&self.sanctuary, &self.paths) {
            Ok(()) => self.last_storage_status = "💾 saved".to_string(),
            Err(err) => self.last_storage_status = format!("⚠️ save failed: {err}"),
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
        self.screen = AppScreen::Onboarding;
        self.onboarding_step = OnboardingStep::SelectSpecies;
        self.selected_species_idx = 0;
        self.selected_theme_idx = 0;
        self.onboarding_name_input = PetSpecies::all()[0].default_name().to_string();
    }

    fn launch_minigame(&mut self) {
        let refusal = if self.pet.stage == EvolutionStage::Egg {
            Some("🥚 The egg can't play yet — warm it first!".to_string())
        } else if self.pet.is_sleeping {
            Some(format!("💤 {} is asleep — wake them first!", self.pet.name))
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

    pub fn handle_key(&mut self, code: KeyCode) {
        if self.rename_buffer.is_some() {
            self.handle_rename_key(code);
            return;
        }

        match self.screen {
            AppScreen::Onboarding => self.handle_onboarding_key(code),
            AppScreen::SanctuaryRoster => self.handle_roster_key(code),
            AppScreen::MiniGame => self.handle_minigame_key(code),
            AppScreen::Habitat => self.handle_habitat_key(code),
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
                KeyCode::Enter => {
                    self.onboarding_step = OnboardingStep::SelectTheme;
                    self.onboarding_name_input =
                        PetSpecies::all()[self.selected_species_idx].default_name().to_string();
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    if self.sanctuary.hatched_count() > 0 {
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
                let msg = self
                    .pet
                    .apply_minigame_reward(game.score, game.coins_earned, game.xp_earned);
                self.pet.log_action(msg);
            }
        }
        self.screen = AppScreen::Habitat;
        self.save_now();
    }

    fn handle_habitat_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.save_now();
                self.should_quit = true;
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char(' ') => {
                let msg = if self.pet.stage == EvolutionStage::Egg {
                    self.pet.warm_egg()
                } else {
                    self.pet.pet_love()
                };
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('1') => {
                let msg = self.pet.feed();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char('2') => {
                let msg = self.pet.play();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('3') => {
                let msg = self.pet.pet_love();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('4') => {
                let msg = self.pet.train();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('5') => {
                let msg = self.pet.toggle_sleep();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('6') => {
                let msg = self.pet.clean();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('7') => {
                let msg = self.pet.heal();
                self.pet.log_action(msg);
                self.save_now();
            }
            KeyCode::Char('g') | KeyCode::Char('G') | KeyCode::Char('m') | KeyCode::Char('M') => {
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

fn shell_block(title: &str, pet: &Pet) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme_color(pet.theme)))
        .title(Span::styled(
            title.to_string(),
            Style::default()
                .fg(theme_color(pet.theme))
                .add_modifier(Modifier::BOLD),
        ))
}

fn ratio_of(value: u8) -> f64 {
    (value as f64 / 100.0).clamp(0.0, 1.0)
}

fn stat_gauge(label: &str, value: u8, color: Color) -> Gauge<'static> {
    Gauge::default()
        .block(
            Block::default().title(Span::styled(
                label.to_string(),
                Style::default().fg(Color::Gray),
            )),
        )
        .gauge_style(Style::default().fg(color))
        .ratio(ratio_of(value))
        .label(format!("{value}%"))
}

fn habitat_areas(area: Rect, tier: LayoutTier) -> (Rect, Rect, Vec<Rect>, Rect) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(7),
            Constraint::Length(5),
        ])
        .split(area);

    let body = vertical[1];
    let columns = match tier {
        LayoutTier::Compact => Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(9)])
            .split(body)
            .to_vec(),
        _ => Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
            .split(body)
            .to_vec(),
    };

    (vertical[0], vertical[2], columns, vertical[3])
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    match app.screen {
        AppScreen::Onboarding => draw_onboarding(f, app),
        AppScreen::SanctuaryRoster => draw_roster(f, app),
        AppScreen::MiniGame => draw_minigame(f, app),
        AppScreen::Habitat => draw_habitat(f, app),
    }
}

fn draw_onboarding(f: &mut Frame, app: &App) {
    let area = f.area();
    let species = PetSpecies::all()[app.selected_species_idx];
    let block = shell_block(" 🍡 Mochigotchi — Adopt a Companion ", &app.pet);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let current = match app.onboarding_step {
        OnboardingStep::SelectSpecies => 1,
        OnboardingStep::SelectTheme => 2,
        OnboardingStep::EnterName => 3,
    };
    let step_line = Line::from(
        ["1. Species", "2. Shell", "3. Name"]
            .iter()
            .enumerate()
            .flat_map(|(i, s)| {
                let style = if i + 1 == current {
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                vec![
                    Span::styled(format!(" {s} "), style),
                    Span::styled("→", Style::default().fg(Color::DarkGray)),
                ]
            })
            .collect::<Vec<_>>(),
    );

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(inner);
    f.render_widget(Paragraph::new(step_line), rows[0]);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(rows[1]);

    let items: Vec<ListItem> = PetSpecies::all()
        .iter()
        .enumerate()
        .map(|(idx, sp)| {
            let selected = idx == app.selected_species_idx;
            let style = if selected {
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(
                format!(
                    "{} {} · {} · {} · {}",
                    if selected { "▶" } else { " " },
                    sp.display_name(),
                    sp.family(),
                    sp.rarity(),
                    sp.element_badge()
                ),
                style,
            )))
        })
        .collect();
    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Choose your companion (↑/↓) "),
        ),
        columns[0],
    );

    let mut preview_lines = vec![
        Line::from(Span::styled(
            species.display_name().to_string(),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            species.description().to_string(),
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];
    for line in render_species_preview(species) {
        preview_lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(stage_color(EvolutionStage::Adult)),
        )));
    }
    preview_lines.push(Line::from(""));
    preview_lines.push(Line::from(Span::styled(
        format!("Loves: {}", species.favorite_snack()),
        Style::default().fg(Color::LightGreen),
    )));

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(6)])
        .split(columns[1]);

    f.render_widget(
        Paragraph::new(Text::from(preview_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Preview "),
            )
            .wrap(Wrap { trim: false }),
        right[0],
    );

    let detail = match app.onboarding_step {
        OnboardingStep::SelectSpecies => vec![
            Line::from(Span::styled(
                "Cozy Classics or Mythical & Fantasy.",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                "Enter = choose shell color",
                Style::default().fg(Color::LightMagenta),
            )),
        ],
        OnboardingStep::SelectTheme => {
            let mut lines = vec![Line::from(Span::styled(
                "Shell colorway (Enter confirms):",
                Style::default().fg(Color::Gray),
            ))];
            for (idx, theme) in ShellTheme::all().iter().enumerate() {
                let selected = idx == app.selected_theme_idx;
                lines.push(Line::from(Span::styled(
                    format!(
                        "{} {}",
                        if selected { "▶" } else { " " },
                        theme.display_name()
                    ),
                    if selected {
                        Style::default()
                            .fg(theme_color(*theme))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                )));
            }
            lines
        }
        OnboardingStep::EnterName => vec![
            Line::from(Span::styled(
                "Type a name, then press Enter:",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                format!("▏{}▕", app.onboarding_name_input),
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
        ],
    };
    f.render_widget(
        Paragraph::new(Text::from(detail))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Step details "),
            )
            .wrap(Wrap { trim: false }),
        right[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "↑/↓ choose · Enter next · Esc back/quit",
            Style::default().fg(Color::DarkGray),
        ))),
        rows[2],
    );
}

fn draw_habitat(f: &mut Frame, app: &App) {
    let area = f.area();
    let tier = LayoutTier::from_width(area.width);
    let pet = &app.pet;
    let title = format!(
        " 🍡 {} · {} · {} · {} ",
        pet.name,
        pet.species.display_name(),
        pet.species.rarity(),
        pet.stage.badge()
    );
    let block = shell_block(&title, pet);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let (header_area, dock_area, columns, log_area) = habitat_areas(inner, tier);

    let (sky, mood_name) = sky_scene(app.tick_count);
    let warm_cd = pet.warm_cooldown_remaining(chrono::Utc::now());
    let train_cd = pet.train_cooldown_remaining(chrono::Utc::now());
    let cooldown_note = if pet.stage == EvolutionStage::Egg {
        if warm_cd > 0 {
            format!("🕯️ warming rest {warm_cd}s")
        } else {
            "🕯️ ready to warm".to_string()
        }
    } else if train_cd > 0 {
        format!("💪 training rest {train_cd}s")
    } else {
        "💪 ready to train".to_string()
    };

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{sky}  {mood_name}  "),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                format!(
                    "🏰 {} pets · Lv.{}  ",
                    app.sanctuary.pets.len(),
                    app.sanctuary.sanctuary_level
                ),
                Style::default().fg(Color::LightBlue),
            ),
            Span::styled(cooldown_note, Style::default().fg(Color::LightYellow)),
        ])),
        header_area,
    );

    let mut sprite_lines = vec![Line::from(Span::styled(
        speech_bubble(pet, app.tick_count),
        Style::default()
            .fg(mood_color(pet.mood))
            .add_modifier(Modifier::ITALIC),
    ))];
    for line in render_animated_sprite(pet, app.tick_count) {
        sprite_lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(stage_color(pet.stage)),
        )));
    }
    sprite_lines.push(Line::from(Span::styled(
        format!(
            "mood: {} {}",
            pet.mood.label(),
            get_mood_particles(pet.mood, app.tick_count)
        ),
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(
        Paragraph::new(Text::from(sprite_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Habitat "),
            )
            .wrap(Wrap { trim: false }),
        columns[0],
    );

    let gauges = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(columns[1]);

    if pet.stage == EvolutionStage::Egg {
        f.render_widget(
            stat_gauge("🥚 Incubation", pet.incubation, Color::LightYellow),
            gauges[0],
        );
    } else {
        let target = pet.stage.next_stage().map_or(pet.xp, |n| n.min_xp());
        f.render_widget(
            Gauge::default()
                .block(Block::default().title(Span::styled(
                    format!("⭐ XP {} / {}", pet.xp, target),
                    Style::default().fg(Color::Gray),
                )))
                .gauge_style(Style::default().fg(Color::LightMagenta))
                .ratio(pet.xp_progress_ratio())
                .label(format!("Lv.{}", pet.level)),
            gauges[0],
        );
    }
    f.render_widget(stat_gauge("🍙 Hunger", pet.hunger, Color::LightGreen), gauges[1]);
    f.render_widget(
        stat_gauge("💖 Happiness", pet.happiness, Color::LightMagenta),
        gauges[2],
    );
    f.render_widget(
        stat_gauge("⚡ Energy", pet.energy, Color::LightYellow),
        gauges[3],
    );
    f.render_widget(stat_gauge("❤️ Health", pet.health, Color::LightRed), gauges[4]);
    f.render_widget(
        stat_gauge(
            &format!("🫧 Cleanliness (poop: {})", pet.poop_count),
            pet.cleanliness,
            Color::LightCyan,
        ),
        gauges[5],
    );
    f.render_widget(stat_gauge("🤝 Bond", pet.bond, Color::LightBlue), gauges[6]);

    let egg = pet.stage == EvolutionStage::Egg;
    let mut dock_lines = vec![Line::from(Span::styled(
        pet.next_evolution_requirements(),
        Style::default().fg(Color::LightYellow),
    ))];
    dock_lines.push(Line::from(""));
    let keys = if let Some(buffer) = app.rename_buffer.as_ref() {
        format!("🏷️ Rename to: {buffer}▏ (Enter saves, Esc cancels)")
    } else if egg {
        "[W] Warm egg · [N] Adopt another · [L] Sanctuary · [[/]] switch · [Tab] shell color · [R] rename · [Q] save & quit".to_string()
    } else {
        "[F] Feed  [P] Play  [E] Train  [S] Sleep  [C] Clean  [H] Heal  [G] Arcade  [N] Adopt  [L] Sanctuary  [[/]] switch  [Tab] color  [R] rename  [Q] quit".to_string()
    };
    dock_lines.push(Line::from(Span::styled(keys, Style::default().fg(Color::Gray))));
    dock_lines.push(Line::from(Span::styled(
        format!("💰 {} coins   {}", pet.coins, app.last_storage_status),
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(
        Paragraph::new(Text::from(dock_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme_color(pet.theme)))
                    .title(" Care console "),
            )
            .wrap(Wrap { trim: false }),
        dock_area,
    );

    let log_lines: Vec<Line> = pet
        .action_log
        .iter()
        .rev()
        .take(3)
        .map(|msg| Line::from(Span::styled(msg.clone(), Style::default().fg(Color::DarkGray))))
        .collect();
    f.render_widget(
        Paragraph::new(Text::from(log_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Care diary "),
            )
            .wrap(Wrap { trim: false }),
        log_area,
    );
}

fn draw_roster(f: &mut Frame, app: &App) {
    let area = f.area();
    let block = shell_block(
        &format!(
            " 🏰 Mythical Sanctuary — {} companions · Sanctuary Lv.{} ",
            app.sanctuary.pets.len(),
            app.sanctuary.sanctuary_level
        ),
        &app.pet,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(inner);

    let items: Vec<ListItem> = app
        .sanctuary
        .pets
        .iter()
        .enumerate()
        .map(|(idx, pet)| {
            let selected = idx == app.roster_selected_idx;
            let marker = if idx == app.sanctuary.active_index {
                "★"
            } else if selected {
                "▶"
            } else {
                " "
            };
            let style = if selected {
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(
                format!(
                    "{marker} {}. {} [{}] {} Lv.{} · hunger {}% · incub {}%",
                    idx + 1,
                    pet.name,
                    pet.species.display_name(),
                    pet.stage.badge(),
                    pet.level,
                    pet.hunger,
                    pet.incubation
                ),
                style,
            )))
        })
        .collect();
    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Roster (↑/↓ · Enter switch · N adopt) "),
        ),
        columns[0],
    );

    let highlighted = app
        .sanctuary
        .pets
        .get(app.roster_selected_idx)
        .unwrap_or(&app.pet);
    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "{} · {} · {}",
                highlighted.name,
                highlighted.species.display_name(),
                highlighted.species.rarity()
            ),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            highlighted.status_summary(),
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];
    for line in render_animated_sprite(highlighted, app.tick_count) {
        lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(stage_color(highlighted.stage)),
        )));
    }
    f.render_widget(
        Paragraph::new(Text::from(lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Companion card "),
            )
            .wrap(Wrap { trim: false }),
        columns[1],
    );
}

fn draw_minigame(f: &mut Frame, app: &App) {
    let area = f.area();
    let block = shell_block(" 🎮 Snack Catcher Arcade ", &app.pet);
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
                    "score {} · coins {} · xp {} · ticks left {}",
                    game.score, game.coins_earned, game.xp_earned, game.ticks_remaining
                ),
            )
        }
        None => (Text::from(""), "no game running".to_string()),
    };

    f.render_widget(
        Paragraph::new(arena).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Catch the falling snacks! "),
        ),
        rows[0],
    );
    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(status, Style::default().fg(Color::LightYellow))),
            Line::from(Span::styled(
                "←/→ or A/D to move · Esc/Q to stop",
                Style::default().fg(Color::DarkGray),
            )),
        ])),
        rows[1],
    );
}

pub fn run_tui(
    sanctuary: Sanctuary,
    paths: StoragePaths,
    start_onboarding: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new_with_sanctuary(sanctuary, paths, start_onboarding);
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
    fn renders_at_compact_standard_and_wide_sizes_without_panic() {
        for (w, h) in [(50u16, 18u16), (100, 32), (140, 44)] {
            let backend = TestBackend::new(w, h);
            let mut terminal = Terminal::new(backend).unwrap();
            let paths = tmp_paths(&format!("render{w}"));
            let app = App::new(Pet::new("Mochi"), paths);
            terminal.draw(|f| draw_ui(f, &app)).unwrap();
            assert_eq!(LayoutTier::from_width(w), match w {
                50 => LayoutTier::Compact,
                100 => LayoutTier::Standard,
                _ => LayoutTier::Wide,
            });
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
