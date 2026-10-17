use super::*;

// ── Spacious Retro Handheld Layout Helpers ─────────────────────────────────

pub(super) fn handheld_rect(area: Rect) -> Rect {
    let w = area.width.min(HANDHELD_MAX_W);
    let h = area.height.min(HANDHELD_MAX_H);
    let x = area.x + area.width.saturating_sub(w) / 2;
    let y = area.y + area.height.saturating_sub(h) / 2;
    Rect::new(x, y, w, h)
}

pub(super) fn outer_shell_block(title: &str, theme: ShellTheme) -> Block<'static> {
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

pub(super) fn lcd_screen_block(title: &str, accent: Color) -> Block<'static> {
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

pub(super) fn custom_meter_line(
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

pub(super) fn render_unified_bezel(selected: LcdIcon, pet: &Pet) -> Line<'static> {
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

pub(super) fn draw_too_small_notice(f: &mut Frame, area: Rect, theme: ShellTheme) {
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
