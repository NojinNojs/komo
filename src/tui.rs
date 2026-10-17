use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::minigame::MiniGame;
use crate::pet::{EvolutionStage, Pet, PetAction, PetSpecies, ShellTheme};
use crate::sprites::{
    get_sprite_frames, mood_color, render_animated_sprite, speech_bubble, stage_color, theme_color,
};
use crate::storage::{save_dual, StoragePaths};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Onboarding,
    Habitat,
    MiniGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingStep {
    SelectSpecies,
    SelectTheme,
    EnterName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutTier {
    Compact,
    Standard,
    Wide,
}

impl LayoutTier {
    fn from_width(width: u16) -> Self {
        if width < 78 {
            Self::Compact
        } else if width < 110 {
            Self::Standard
        } else {
            Self::Wide
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Compact => "Compact LCD",
            Self::Standard => "Standard LCD",
            Self::Wide => "Wide Deluxe LCD",
        }
    }
}

const CARE_ACTIONS: [(PetAction, &str, &str); 8] = [
    (PetAction::WarmEgg, "Warm/Love", "W/0"),
    (PetAction::Feed, "Feed", "F/1"),
    (PetAction::Play, "Play", "P/2"),
    (PetAction::PetLove, "Headpat", "V/3"),
    (PetAction::Train, "Train", "T/4"),
    (PetAction::ToggleSleep, "Sleep", "S/5"),
    (PetAction::Clean, "Clean", "C/6"),
    (PetAction::Heal, "Heal", "H/7"),
];

pub struct App {
    pub pet: Pet,
    pub paths: StoragePaths,
    pub screen: AppScreen,
    pub onboarding_step: OnboardingStep,
    pub selected_species_idx: usize,
    pub selected_theme_idx: usize,
    pub name_input: String,
    pub selected_action_idx: usize,
    pub tick_count: usize,
    pub status_banner: String,
    pub minigame: Option<MiniGame>,
    pub should_quit: bool,
}

impl App {
    pub fn new(pet: Pet, paths: StoragePaths) -> Self {
        let banner = pet
            .action_log
            .last()
            .cloned()
            .unwrap_or_else(|| "Welcome to Mochigotchi!".to_string());
        let selected_species_idx = PetSpecies::all()
            .iter()
            .position(|&s| s == pet.species)
            .unwrap_or(0);
        let selected_theme_idx = ShellTheme::all()
            .iter()
            .position(|&t| t == pet.theme)
            .unwrap_or(0);
        let name_input = pet.name.clone();

        Self {
            pet,
            paths,
            screen: AppScreen::Habitat,
            onboarding_step: OnboardingStep::SelectSpecies,
            selected_species_idx,
            selected_theme_idx,
            name_input,
            selected_action_idx: 0,
            tick_count: 0,
            status_banner: banner,
            minigame: None,
            should_quit: false,
        }
    }

    pub fn new_onboarding(paths: StoragePaths) -> Self {
        let default_species = PetSpecies::MochiCat;
        let pet = Pet::new_custom(
            default_species.default_name(),
            default_species,
            ShellTheme::StrawberryPink,
        );
        Self {
            pet,
            paths,
            screen: AppScreen::Onboarding,
            onboarding_step: OnboardingStep::SelectSpecies,
            selected_species_idx: 0,
            selected_theme_idx: 0,
            name_input: default_species.default_name().to_string(),
            selected_action_idx: 0,
            tick_count: 0,
            status_banner: "✨ Welcome! Choose your companion species, shell theme, and name."
                .to_string(),
            minigame: None,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

        if self.screen == AppScreen::MiniGame {
            if let Some(game) = &mut self.minigame {
                game.step();
                if game.is_finished() {
                    let score = game.score;
                    let coins = game.coins_earned;
                    let xp = game.xp_earned;
                    self.minigame = None;
                    self.screen = AppScreen::Habitat;
                    self.status_banner = self.pet.apply_minigame_reward(score, coins, xp);
                    let _ = save_dual(&self.pet, &self.paths);
                }
            }
            return;
        }

        if self.screen == AppScreen::Habitat && self.tick_count.is_multiple_of(20) {
            self.pet.tick_decay();
            let _ = save_dual(&self.pet, &self.paths);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match self.screen {
            AppScreen::Onboarding => self.handle_onboarding_key(key),
            AppScreen::Habitat => self.handle_habitat_key(key),
            AppScreen::MiniGame => self.handle_minigame_key(key),
        }
    }

    fn handle_onboarding_key(&mut self, key: KeyCode) {
        let species_list = PetSpecies::all();
        let theme_list = ShellTheme::all();

        match self.onboarding_step {
            OnboardingStep::SelectSpecies => match key {
                KeyCode::Left | KeyCode::Up | KeyCode::Char('h') | KeyCode::Char('k') => {
                    self.selected_species_idx =
                        (self.selected_species_idx + species_list.len() - 1) % species_list.len();
                    self.name_input =
                        species_list[self.selected_species_idx].default_name().to_string();
                }
                KeyCode::Right | KeyCode::Down | KeyCode::Char('l') | KeyCode::Char('j') => {
                    self.selected_species_idx =
                        (self.selected_species_idx + 1) % species_list.len();
                    self.name_input =
                        species_list[self.selected_species_idx].default_name().to_string();
                }
                KeyCode::Enter => {
                    self.name_input =
                        species_list[self.selected_species_idx].default_name().to_string();
                    self.onboarding_step = OnboardingStep::SelectTheme;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            },
            OnboardingStep::SelectTheme => match key {
                KeyCode::Left | KeyCode::Up | KeyCode::Char('h') | KeyCode::Char('k') => {
                    self.selected_theme_idx =
                        (self.selected_theme_idx + theme_list.len() - 1) % theme_list.len();
                    self.pet.theme = theme_list[self.selected_theme_idx];
                }
                KeyCode::Right | KeyCode::Down | KeyCode::Char('l') | KeyCode::Char('j') => {
                    self.selected_theme_idx = (self.selected_theme_idx + 1) % theme_list.len();
                    self.pet.theme = theme_list[self.selected_theme_idx];
                }
                KeyCode::Enter => {
                    self.onboarding_step = OnboardingStep::EnterName;
                }
                KeyCode::Backspace | KeyCode::Esc => {
                    self.onboarding_step = OnboardingStep::SelectSpecies;
                }
                _ => {}
            },
            OnboardingStep::EnterName => match key {
                KeyCode::Backspace => {
                    self.name_input.pop();
                }
                KeyCode::Char(c) if !c.is_control() => {
                    if self.name_input.chars().count() < 18 {
                        self.name_input.push(c);
                    }
                }
                KeyCode::Enter => {
                    let chosen_species = species_list[self.selected_species_idx];
                    let chosen_theme = theme_list[self.selected_theme_idx];
                    self.pet = Pet::new_custom(&self.name_input, chosen_species, chosen_theme);
                    self.status_banner = self
                        .pet
                        .action_log
                        .last()
                        .cloned()
                        .unwrap_or_else(|| "Egg adopted!".to_string());
                    let _ = save_dual(&self.pet, &self.paths);
                    self.screen = AppScreen::Habitat;
                }
                KeyCode::Esc => {
                    self.onboarding_step = OnboardingStep::SelectTheme;
                }
                _ => {}
            },
        }
    }

    fn execute_pet_action(&mut self, idx: usize) {
        self.selected_action_idx = idx % CARE_ACTIONS.len();
        let (action, _, _) = CARE_ACTIONS[self.selected_action_idx];
        self.status_banner = self.pet.perform_action(action);
        let _ = save_dual(&self.pet, &self.paths);
    }

    fn handle_habitat_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => {
                self.selected_action_idx =
                    (self.selected_action_idx + CARE_ACTIONS.len() - 1) % CARE_ACTIONS.len();
            }
            KeyCode::Right => {
                self.selected_action_idx = (self.selected_action_idx + 1) % CARE_ACTIONS.len();
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.execute_pet_action(self.selected_action_idx);
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char('0') => {
                self.execute_pet_action(0);
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('1') => {
                self.execute_pet_action(1);
            }
            KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char('2') => {
                self.execute_pet_action(2);
            }
            KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Char('3') => {
                self.execute_pet_action(3);
            }
            KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('4') => {
                self.execute_pet_action(4);
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('5') => {
                self.execute_pet_action(5);
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('6') => {
                self.execute_pet_action(6);
            }
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('7') => {
                self.execute_pet_action(7);
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.minigame = Some(MiniGame::new(36, 12, 60));
                self.screen = AppScreen::MiniGame;
                self.status_banner =
                    "🎮 Snack & Star Catcher! Use [←/→] to catch [O]/[*] and dodge [X]!".to_string();
            }
            KeyCode::Tab => {
                let themes = ShellTheme::all();
                let next_idx = themes
                    .iter()
                    .position(|&t| t == self.pet.theme)
                    .map(|i| (i + 1) % themes.len())
                    .unwrap_or(0);
                self.pet.theme = themes[next_idx];
                self.selected_theme_idx = next_idx;
                self.status_banner =
                    format!("🎨 Switched shell theme to {}!", self.pet.theme.display_name());
                let _ = save_dual(&self.pet, &self.paths);
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.screen = AppScreen::Onboarding;
                self.onboarding_step = OnboardingStep::SelectSpecies;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                let _ = save_dual(&self.pet, &self.paths);
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_minigame_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left
            | KeyCode::Char('a')
            | KeyCode::Char('A')
            | KeyCode::Char('h')
            | KeyCode::Char('H') => {
                if let Some(game) = &mut self.minigame {
                    game.move_left();
                }
            }
            KeyCode::Right
            | KeyCode::Char('d')
            | KeyCode::Char('D')
            | KeyCode::Char('l')
            | KeyCode::Char('L') => {
                if let Some(game) = &mut self.minigame {
                    game.move_right();
                }
            }
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('Q') => {
                if let Some(game) = self.minigame.take() {
                    if game.score > 0 || game.coins_earned > 0 || game.xp_earned > 0 {
                        self.status_banner = self.pet.apply_minigame_reward(
                            game.score,
                            game.coins_earned,
                            game.xp_earned,
                        );
                    } else {
                        self.status_banner =
                            "🎮 Returned from Snack Catcher arcade!".to_string();
                    }
                    let _ = save_dual(&self.pet, &self.paths);
                }
                self.screen = AppScreen::Habitat;
            }
            _ => {}
        }
    }
}

fn themed_block(title: impl Into<String>, border_color: Color) -> Block<'static> {
    Block::default()
        .title(format!(" {} ", title.into()))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
}

pub fn draw_ui(frame: &mut Frame, app: &App) {
    match app.screen {
        AppScreen::Onboarding => draw_onboarding(frame, app),
        AppScreen::Habitat => draw_habitat(frame, app),
        AppScreen::MiniGame => draw_minigame(frame, app),
    }
}

fn draw_onboarding(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let themes = ShellTheme::all();
    let active_theme = themes
        .get(app.selected_theme_idx)
        .copied()
        .unwrap_or(app.pet.theme);
    let border_col = theme_color(active_theme);

    let outer = themed_block("🥚 Mochigotchi Adoption & Shell Wizard", border_col);
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(inner);

    let step_label = match app.onboarding_step {
        OnboardingStep::SelectSpecies => "Step 1/3: Choose Your Companion Species",
        OnboardingStep::SelectTheme => "Step 2/3: Choose Your Pastel LCD Shell Theme",
        OnboardingStep::EnterName => "Step 3/3: Name Your New Pocket Pet",
    };

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            step_label,
            Style::default()
                .fg(border_col)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  •  Use [←/→] to browse, [Enter] to confirm"),
    ]))
    .alignment(Alignment::Center)
    .block(themed_block("Wizard Progress", border_col));
    frame.render_widget(header, chunks[0]);

    match app.onboarding_step {
        OnboardingStep::SelectSpecies => {
            let species_list = PetSpecies::all();
            let chosen = species_list
                .get(app.selected_species_idx)
                .copied()
                .unwrap_or(PetSpecies::MochiCat);
            let preview_frames =
                get_sprite_frames(chosen, EvolutionStage::Baby, crate::pet::PetMood::Happy);
            let sprite = preview_frames[app.tick_count % preview_frames.len()];

            let mut lines = Vec::new();
            let mut tabs = Vec::new();
            for (idx, sp) in species_list.iter().enumerate() {
                if idx == app.selected_species_idx {
                    tabs.push(Span::styled(
                        format!(" [ {} ] ", sp.display_name()),
                        Style::default()
                            .fg(Color::Black)
                            .bg(border_col)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    tabs.push(Span::styled(
                        format!("   {}   ", sp.display_name()),
                        Style::default().fg(Color::Gray),
                    ));
                }
            }
            lines.push(Line::from(tabs));
            lines.push(Line::from(""));
            for sl in sprite.lines() {
                lines.push(Line::from(Span::styled(
                    sl.to_string(),
                    Style::default()
                        .fg(border_col)
                        .add_modifier(Modifier::BOLD),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                chosen.description(),
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(Span::styled(
                format!(
                    "Default Name: {}   |   Favorite Snack: {}",
                    chosen.default_name(),
                    chosen.favorite_snack()
                ),
                Style::default().fg(Color::Yellow),
            )));

            let body = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false })
                .block(themed_block("Species Preview", border_col));
            frame.render_widget(body, chunks[1]);
        }
        OnboardingStep::SelectTheme => {
            let mut lines = Vec::new();
            lines.push(Line::from(""));
            for (idx, th) in themes.iter().enumerate() {
                let col = theme_color(*th);
                if idx == app.selected_theme_idx {
                    lines.push(Line::from(Span::styled(
                        format!("  ▶ [ {} ] ◀  ", th.display_name()),
                        Style::default()
                            .fg(Color::Black)
                            .bg(col)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(
                        format!("      {}      ", th.display_name()),
                        Style::default().fg(col),
                    )));
                }
                lines.push(Line::from(""));
            }

            let body = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .block(themed_block("Shell Casing Palette", border_col));
            frame.render_widget(body, chunks[1]);
        }
        OnboardingStep::EnterName => {
            let chosen_species = PetSpecies::all()
                .get(app.selected_species_idx)
                .copied()
                .unwrap_or(PetSpecies::MochiCat);
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("Species: {}  |  Theme: {}", chosen_species.display_name(), active_theme.display_name()),
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Type your pet's name (or leave blank for default):",
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!("  > {}█ <  ", app.name_input),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
            ];
            let body = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .block(themed_block("Pet Name", border_col));
            frame.render_widget(body, chunks[1]);
        }
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " [←/→] ",
            Style::default().fg(border_col).add_modifier(Modifier::BOLD),
        ),
        Span::raw("Select   "),
        Span::styled(
            " [Enter] ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Next / Hatch Egg   "),
        Span::styled(
            " [Esc] ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("Back / Quit"),
    ]))
    .alignment(Alignment::Center)
    .block(themed_block("Controls", border_col));
    frame.render_widget(footer, chunks[2]);
}

fn draw_habitat(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let tier = LayoutTier::from_width(area.width);
    let border_col = theme_color(app.pet.theme);

    let root_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    // Header
    let header_line = Line::from(vec![
        Span::styled(
            format!(" {} ", app.pet.name),
            Style::default()
                .fg(Color::Black)
                .bg(border_col)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            app.pet.species.display_name(),
            Style::default().fg(border_col).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" • "),
        Span::styled(
            format!(
                "{} (Lv.{} | {} XP)",
                app.pet.stage.display_name(),
                app.pet.level,
                app.pet.xp
            ),
            Style::default().fg(stage_color(app.pet.stage)),
        ),
        Span::raw(" • "),
        Span::styled(
            format!("🪙 {} Coins", app.pet.coins),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" • "),
        Span::styled(
            format!("[{}]", tier.label()),
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    let header = Paragraph::new(header_line)
        .alignment(Alignment::Center)
        .block(themed_block(
            format!("🍓 Mochigotchi — {}", app.pet.theme.display_name()),
            border_col,
        ));
    frame.render_widget(header, root_chunks[0]);

    // Responsive 3-tier middle section
    match tier {
        LayoutTier::Compact => {
            let mid_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
                .split(root_chunks[1]);
            draw_lcd_viewport(frame, app, mid_chunks[0], border_col);
            draw_vital_gauges(frame, app, mid_chunks[1], border_col);
        }
        LayoutTier::Standard => {
            let mid_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
                .split(root_chunks[1]);
            draw_lcd_viewport(frame, app, mid_chunks[0], border_col);
            draw_vital_gauges(frame, app, mid_chunks[1], border_col);
        }
        LayoutTier::Wide => {
            let mid_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(38),
                    Constraint::Percentage(34),
                    Constraint::Percentage(28),
                ])
                .split(root_chunks[1]);
            draw_lcd_viewport(frame, app, mid_chunks[0], border_col);
            draw_vital_gauges(frame, app, mid_chunks[1], border_col);
            draw_diary_and_profile(frame, app, mid_chunks[2], border_col);
        }
    }

    // Bottom LCD Care Buttons + Status Banner
    let mut button_spans = Vec::new();
    for (idx, (_, label, shortcut)) in CARE_ACTIONS.iter().enumerate() {
        let display_label = if idx == 0 && app.pet.stage == EvolutionStage::Egg {
            "Warm Egg"
        } else {
            *label
        };
        if idx == app.selected_action_idx {
            button_spans.push(Span::styled(
                format!(" [{}:{}] ", shortcut, display_label),
                Style::default()
                    .fg(Color::Black)
                    .bg(border_col)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            button_spans.push(Span::styled(
                format!(" [{}:{}] ", shortcut, display_label),
                Style::default().fg(border_col),
            ));
        }
    }

    let footer_lines = vec![
        Line::from(button_spans),
        Line::from(Span::styled(
            &app.status_banner,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "[←/→/Enter] Care  [G] Mini-Game  [Tab] Shell Theme  [N] New Pet  [Q] Save & Quit",
            Style::default().fg(Color::Gray),
        )),
    ];

    let footer = Paragraph::new(footer_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(themed_block("LCD Care Deck", border_col));
    frame.render_widget(footer, root_chunks[2]);
}

fn draw_lcd_viewport(frame: &mut Frame, app: &App, area: Rect, border_col: Color) {
    let sprite_lines = render_animated_sprite(&app.pet, app.tick_count);
    let bubble = speech_bubble(&app.pet, app.tick_count);
    let m_col = mood_color(app.pet.mood);

    let mut text_lines = Vec::new();
    text_lines.push(Line::from(Span::styled(
        format!(
            "Mood: {}  |  Bond: {}%  |  Snack: {}",
            app.pet.mood.label(),
            app.pet.bond,
            app.pet.species.favorite_snack()
        ),
        Style::default().fg(m_col).add_modifier(Modifier::BOLD),
    )));
    text_lines.push(Line::from(""));

    for line in sprite_lines {
        text_lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(m_col).add_modifier(Modifier::BOLD),
        )));
    }

    text_lines.push(Line::from(""));
    text_lines.push(Line::from(Span::styled(
        bubble,
        Style::default().fg(Color::White),
    )));

    let viewport = Paragraph::new(text_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
        .block(themed_block("LCD Habitat Screen", border_col));
    frame.render_widget(viewport, area);
}

fn draw_vital_gauges(frame: &mut Frame, app: &App, area: Rect, border_col: Color) {
    let block = themed_block("Vital Telemetry", border_col);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner);

    let progress_pct = (app.pet.xp_progress_ratio() * 100.0).round() as u16;
    let evo_label = if app.pet.stage == EvolutionStage::Egg {
        format!("🥚 Egg Incubation: {}%", app.pet.incubation)
    } else if let Some(next) = app.pet.stage.next_stage() {
        format!(
            "✨ Evolution to {}: {}% ({} XP)",
            next.display_name(),
            progress_pct,
            app.pet.xp
        )
    } else {
        format!("🌟 Max Stage ({} XP)", app.pet.xp)
    };

    let gauges: [(&str, u16, Color); 6] = [
        (&evo_label, progress_pct.min(100), border_col),
        (
            &format!("🍙 Fullness: {}%", app.pet.hunger),
            app.pet.hunger as u16,
            Color::Green,
        ),
        (
            &format!("💖 Happiness: {}%", app.pet.happiness),
            app.pet.happiness as u16,
            Color::LightMagenta,
        ),
        (
            &format!("⚡ Energy: {}%", app.pet.energy),
            app.pet.energy as u16,
            Color::Cyan,
        ),
        (
            &format!("🧼 Cleanliness: {}% (💩x{})", app.pet.cleanliness, app.pet.poop_count),
            app.pet.cleanliness as u16,
            Color::Blue,
        ),
        (
            &format!("💊 Health: {}%", app.pet.health),
            app.pet.health as u16,
            if app.pet.is_sick {
                Color::LightRed
            } else {
                Color::LightGreen
            },
        ),
    ];

    for (i, (label, pct, color)) in gauges.iter().enumerate() {
        if i < rows.len() {
            let g = Gauge::default()
                .gauge_style(Style::default().fg(*color).bg(Color::Black))
                .percent((*pct).min(100))
                .label(label.to_string());
            frame.render_widget(g, rows[i]);
        }
    }
}

fn draw_diary_and_profile(frame: &mut Frame, app: &App, area: Rect, border_col: Color) {
    let items: Vec<ListItem> = app
        .pet
        .action_log
        .iter()
        .rev()
        .take(12)
        .map(|entry| ListItem::new(Line::from(Span::raw(entry.clone()))))
        .collect();

    let list = List::new(items).block(themed_block("Care Diary & Activity Log", border_col));
    frame.render_widget(list, area);
}

fn draw_minigame(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let border_col = theme_color(app.pet.theme);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    if let Some(game) = &app.minigame {
        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                format!(" 🎮 Snack & Star Catcher — {} ", app.pet.name),
                Style::default()
                    .fg(Color::Black)
                    .bg(border_col)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "  Score: {}  |  Coins: +{}  |  XP: +{}  |  Time Left: {} ticks",
                game.score, game.coins_earned, game.xp_earned, game.ticks_remaining
            )),
        ]))
        .alignment(Alignment::Center)
        .block(themed_block("Arcade Mode", border_col));
        frame.render_widget(header, chunks[0]);

        let board_lines: Vec<Line> = game
            .render_lines()
            .into_iter()
            .map(|l| {
                Line::from(Span::styled(
                    l,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();

        let board = Paragraph::new(board_lines)
            .alignment(Alignment::Center)
            .block(themed_block(
                "Catch [O] Snack (+15) & [*] Star (+25) — Dodge [X] Storm (-15)",
                border_col,
            ));
        frame.render_widget(board, chunks[1]);
    }

    let footer = Paragraph::new(Line::from(
        "[← / A / H] Move Left    [→ / D / L] Move Right    [Esc / Enter / Q] Collect Rewards & Exit",
    ))
    .alignment(Alignment::Center)
    .block(themed_block("Arcade Controls", border_col));
    frame.render_widget(footer, chunks[2]);
}

pub fn run_tui(pet: Pet, paths: StoragePaths) -> io::Result<Pet> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = if !crate::storage::has_existing_save(&paths)
        && pet.age_seconds == 0
        && pet.xp == 0
        && pet.incubation == 0
    {
        App::new_onboarding(paths.clone())
    } else {
        App::new(pet, paths.clone())
    };
    let tick_rate = Duration::from_millis(250);
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
    let _ = save_dual(&app.pet, &paths);
    Ok(app.pet)
}
