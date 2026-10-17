use super::*;

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

    pub(super) fn tmp_paths(tag: &str) -> StoragePaths {
        let dir = std::env::temp_dir().join(format!("mochi_tui_{tag}_{}", std::process::id()));
        StoragePaths::new(dir.join("save.json"), dir.join("save.db"))
    }

    #[test]
    pub(super) fn onboarding_auto_morphs_evolution_and_adopts_companions() {
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
    pub(super) fn renders_resize_guard_on_small_terminal_and_full_ui_on_standard_and_wide() {
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
    pub(super) fn every_screen_and_arcade_mode_renders() {
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
    pub(super) fn tick_loop_decays_stats_and_arcade_finishes() {
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
