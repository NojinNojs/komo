use super::*;

pub(super) fn draw_roster_screen(f: &mut Frame, device: Rect, app: &App) {
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

pub(super) fn render_arcade_mode_tabs(active_mode: GameMode) -> Line<'static> {
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

pub(super) fn draw_minigame_screen(f: &mut Frame, device: Rect, app: &App) {
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
    f.render_widget(Paragraph::new(arena).alignment(Alignment::Center), rows[1]);
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
