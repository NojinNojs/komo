use super::*;

pub(super) fn draw_offline_report_screen(
    f: &mut Frame,
    device: Rect,
    app: &App,
    report: &OfflineReport,
) {
    let shell = outer_shell_block("🌙 WELCOME HOME TO YOUR SANCTUARY", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!(
            "AWAY FOR {}  •  {}",
            report.formatted_duration().to_uppercase(),
            app.pet.name.to_uppercase()
        ),
        Color::LightCyan,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    if let Some(host) = &app.pet.pending_hosting {
        let zones = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(11),
                Constraint::Length(5),
            ])
            .split(lcd_inner);

        let sub_note = host
            .gift_note
            .clone()
            .or_else(|| report.events.first().cloned())
            .unwrap_or_default();

        f.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(Span::styled(
                    host.greeting_thought.clone(),
                    Style::default().fg(Color::LightCyan),
                )),
                Line::from(Span::styled(
                    host.greeting_speech.clone(),
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    sub_note,
                    Style::default().fg(Color::LightGreen),
                )),
            ]))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
            zones[0],
        );

        f.render_widget(
            Paragraph::new(Text::from(render_colored_sprite(&app.pet, app.tick_count)))
                .alignment(Alignment::Center),
            zones[1],
        );

        let opts = [
            (1u8, host.option_1.as_str()),
            (2u8, host.option_2.as_str()),
            (3u8, host.option_3.as_str()),
        ];
        let mut opt_lines = Vec::with_capacity(5);
        for (idx, text) in opts {
            let sel = app.hosting_selected_idx == idx;
            let prefix = if sel { "▸ " } else { "  " };
            let style = if sel {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            opt_lines.push(Line::from(Span::styled(format!(" {prefix}{text} "), style)));
        }
        opt_lines.push(Line::from(Span::styled(
            "[↑/↓ or 1-3] Choose Greeting  •  [Enter / Space] Reply  •  [Esc] Skip",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));

        f.render_widget(
            Paragraph::new(Text::from(opt_lines)).alignment(Alignment::Center),
            zones[2],
        );
        return;
    }

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
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        lcd_inner,
    );
}
