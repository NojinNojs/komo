use super::*;

pub(super) fn draw_onb_theme(
    f: &mut Frame,
    inner: Rect,
    app: &App,
    species: PetSpecies,
    theme: ShellTheme,
    sp_primary: Color,
) {
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

pub(super) fn draw_onb_name(
    f: &mut Frame,
    inner: Rect,
    app: &App,
    species: PetSpecies,
    theme: ShellTheme,
    sp_primary: Color,
) {
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

    let name_style = if !app.onboarding_edit_caretaker {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightYellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let caretaker_style = if app.onboarding_edit_caretaker {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let bottom_lines = vec![
        Line::from(vec![
            Span::styled("Companion Name: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    " [ {}{} ] ",
                    app.onboarding_name_input,
                    if !app.onboarding_edit_caretaker {
                        "▏"
                    } else {
                        ""
                    }
                ),
                name_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Call You As:    ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    " [ {}{} ] ",
                    app.onboarding_caretaker_input,
                    if app.onboarding_edit_caretaker {
                        "▏"
                    } else {
                        ""
                    }
                ),
                caretaker_style,
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "[Tab] Switch Field   [Enter] Adopt & Start   [Backspace] Edit   [Esc] Back",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    f.render_widget(
        Paragraph::new(Text::from(bottom_lines)).alignment(Alignment::Center),
        zones[2],
    );
}
