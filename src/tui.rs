use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use crate::git_hook::sync_latest_commits;
use crate::minigame::MiniGame;
use crate::pet::Pet;
use crate::sprites::{mood_color, render_animated_sprite, speech_bubble, stage_color};
use crate::storage::{save_dual, StoragePaths};

pub struct App {
    pub pet: Pet,
    pub paths: StoragePaths,
    pub tick_count: usize,
    pub should_quit: bool,
    pub minigame: Option<MiniGame>,
    pub rename_buffer: Option<String>,
    pub last_storage_status: String,
}

impl App {
    pub fn new(pet: Pet, paths: StoragePaths) -> Self {
        Self {
            pet,
            paths,
            tick_count: 0,
            should_quit: false,
            minigame: None,
            rename_buffer: None,
            last_storage_status: "Synced (JSON + SQLite)".to_string(),
        }
    }

    fn persist(&mut self) {
        match save_dual(&self.pet, &self.paths) {
            Ok(()) => {
                self.last_storage_status = "Saved (JSON + SQLite)".to_string();
            }
            Err(err) => {
                self.last_storage_status = format!("Save error: {err}");
            }
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

        let mut finished_reward = None;
        if let Some(game) = &mut self.minigame {
            if !game.is_finished() {
                game.step();
                if game.is_finished() {
                    finished_reward = Some((game.score, game.coins_earned, game.xp_earned));
                }
            }
        }

        if let Some((score, coins, xp)) = finished_reward {
            self.pet.apply_minigame_reward(score, coins, xp);
            self.persist();
        }

        if self.tick_count.is_multiple_of(10) {
            self.pet.tick_decay();
            self.persist();
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if let Some(buf) = &mut self.rename_buffer {
            match code {
                KeyCode::Enter => {
                    let name = buf.trim().to_string();
                    self.rename_buffer = None;
                    if !name.is_empty() {
                        self.pet.rename(&name);
                        self.persist();
                    }
                }
                KeyCode::Esc => {
                    self.rename_buffer = None;
                }
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Char(c) if buf.chars().count() < 24 => {
                    buf.push(c);
                }
                _ => {}
            }
            return;
        }

        if let Some(mut game) = self.minigame.take() {
            match code {
                KeyCode::Esc => {
                    if !game.is_finished() && (game.coins_earned > 0 || game.xp_earned > 0) {
                        self.pet.apply_minigame_reward(
                            game.score,
                            game.coins_earned,
                            game.xp_earned,
                        );
                        self.persist();
                    }
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                    game.move_left();
                    self.minigame = Some(game);
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                    game.move_right();
                    self.minigame = Some(game);
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if game.is_finished() {
                        // Already rewarded when finished; close minigame view
                    } else {
                        game.step();
                        if game.is_finished() {
                            self.pet.apply_minigame_reward(
                                game.score,
                                game.coins_earned,
                                game.xp_earned,
                            );
                            self.persist();
                        }
                        self.minigame = Some(game);
                    }
                }
                _ => {
                    self.minigame = Some(game);
                }
            }
            return;
        }

        match code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.persist();
                self.should_quit = true;
            }
            KeyCode::Char('f') | KeyCode::Char('1') => {
                self.pet.feed();
                self.persist();
            }
            KeyCode::Char('p') | KeyCode::Char('2') => {
                self.pet.play();
                self.persist();
            }
            KeyCode::Char('s') | KeyCode::Char('3') => {
                self.pet.toggle_sleep();
                self.persist();
            }
            KeyCode::Char('c') | KeyCode::Char('4') => {
                self.pet.clean();
                self.persist();
            }
            KeyCode::Char('k') | KeyCode::Char('5') => {
                self.pet.code_session();
                self.persist();
            }
            KeyCode::Char('h') | KeyCode::Char('6') => {
                self.pet.heal();
                self.persist();
            }
            KeyCode::Char('m') | KeyCode::Char('7') => {
                self.minigame = Some(MiniGame::new(24, 10));
            }
            KeyCode::Char('g') | KeyCode::Char('8') => {
                match sync_latest_commits(&mut self.pet, ".", 5) {
                    Ok(count) => {
                        self.persist();
                        self.last_storage_status =
                            format!("Git sync: {count} new commit(s) rewarded");
                    }
                    Err(err) => {
                        self.last_storage_status = format!("Git sync: {err}");
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('9') => {
                self.rename_buffer = Some(String::new());
            }
            _ => {}
        }
    }
}

fn stat_color(value: u8) -> Color {
    if value >= 70 {
        Color::Green
    } else if value >= 35 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn render_stat_gauge(frame: &mut Frame, area: Rect, label: &str, value: u8) {
    let clamped = value.min(100);
    let color = stat_color(clamped);
    let gauge = Gauge::default()
        .block(Block::default().title(label).borders(Borders::ALL))
        .gauge_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .percent(u16::from(clamped))
        .label(format!("{clamped}%"));
    frame.render_widget(gauge, area);
}

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let root_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(7),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // 1. Header Banner
    let stage_col = stage_color(app.pet.stage);
    let mood_col = mood_color(app.pet.mood);
    let header_line = Line::from(vec![
        Span::styled(
            format!(" 🦀 {} ", app.pet.name),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("| Stage: "),
        Span::styled(
            app.pet.stage.name(),
            Style::default().fg(stage_col).add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            " | Lv.{} ({} XP) | Mood: ",
            app.pet.level, app.pet.xp
        )),
        Span::styled(
            app.pet.mood.label(),
            Style::default().fg(mood_col).add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            " | Coins: {} | Commits: {} | {}",
            app.pet.coins, app.pet.commits_fed, app.last_storage_status
        )),
    ]);

    let header = Paragraph::new(header_line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Rustagotchi Dashboard ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    frame.render_widget(header, root_chunks[0]);

    // 2. Center Split: Left (Sprite / MiniGame / Rename) & Right (Gauges)
    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(root_chunks[1]);

    if let Some(buf) = &app.rename_buffer {
        let prompt_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Enter a new name for your Rustagotchi:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!("> {buf}_"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Press [Enter] to confirm or [Esc] to cancel."),
        ];
        let rename_widget = Paragraph::new(prompt_lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(" Rename Pet ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );
        frame.render_widget(rename_widget, center_chunks[0]);
    } else if let Some(game) = &app.minigame {
        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            format!(
                "Score: {} | Coins: +{} | XP: +{} | Ticks: {}",
                game.score, game.coins_earned, game.xp_earned, game.ticks_remaining
            ),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        for row in game.render_lines() {
            lines.push(Line::from(Span::styled(
                row,
                Style::default().fg(Color::Cyan),
            )));
        }
        lines.push(Line::from(Span::styled(
            game.status_msg.clone(),
            Style::default().fg(Color::Green),
        )));
        lines.push(Line::from(
            "Controls: [←/A/H] Left | [→/D/L] Right | [Space/Enter] Step/Close | [Esc] Exit",
        ));

        let game_widget = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(" Mini-Game: Crate & Bug Catcher ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            );
        frame.render_widget(game_widget, center_chunks[0]);
    } else {
        let sprite_lines = render_animated_sprite(app.pet.stage, app.pet.mood, app.tick_count);
        let bubble = speech_bubble(&app.pet, app.tick_count);
        let mut lines = Vec::new();
        lines.push(Line::from(""));
        for line in sprite_lines {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(stage_col).add_modifier(Modifier::BOLD),
            )));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            bubble,
            Style::default().fg(mood_col),
        )));

        let pet_view = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(format!(" {} ({}) ", app.pet.name, app.pet.stage.name()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(stage_col)),
            );
        frame.render_widget(pet_view, center_chunks[0]);
    }

    // Right Panel: Gauges
    let gauge_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(2),
        ])
        .split(center_chunks[1]);

    render_stat_gauge(frame, gauge_chunks[0], " Hunger ", app.pet.hunger);
    render_stat_gauge(frame, gauge_chunks[1], " Happiness ", app.pet.happiness);
    render_stat_gauge(frame, gauge_chunks[2], " Energy ", app.pet.energy);
    render_stat_gauge(frame, gauge_chunks[3], " Health ", app.pet.health);
    render_stat_gauge(frame, gauge_chunks[4], " Cleanliness ", app.pet.cleanliness);

    let xp_ratio = app.pet.xp_progress_ratio().clamp(0.0, 1.0);
    let xp_percent = (xp_ratio * 100.0).round() as u16;
    let xp_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Evolution XP Progress ")
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(xp_ratio)
        .label(format!("{} XP ({xp_percent}%)", app.pet.xp));
    frame.render_widget(xp_gauge, gauge_chunks[5]);

    // 3. Action Log Panel
    let log_items: Vec<ListItem> = app
        .pet
        .action_log
        .iter()
        .rev()
        .take(5)
        .map(|entry| ListItem::new(Line::from(Span::raw(format!("• {entry}")))))
        .collect();
    let log_list = List::new(log_items).block(
        Block::default()
            .title(" Recent Activity Log ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)),
    );
    frame.render_widget(log_list, root_chunks[2]);

    // 4. Controls / Help Bar
    let controls_text = "[1/F] Feed [2/P] Play [3/S] Sleep [4/C] Clean [5/K] Code [6/H] Heal [7/M] MiniGame [8/G] Git [9/R] Rename [Q] Quit";
    let controls = Paragraph::new(controls_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(" Controls ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(controls, root_chunks[3]);
}

pub fn run_tui(pet: Pet, paths: StoragePaths) -> io::Result<Pet> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(pet, paths);
    let loop_res = run_event_loop(&mut terminal, &mut app);

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    let _ = save_dual(&app.pet, &app.paths);
    loop_res?;
    Ok(app.pet)
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| draw_ui(frame, app))?;

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
}
