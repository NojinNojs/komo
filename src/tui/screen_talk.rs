use super::*;

pub(super) fn draw_talk_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let (sp_primary, _, _) = species_palette(pet.species);
    let prompt = pet.dialogue_prompt(app.tick_count / 20);

    let shell = outer_shell_block(
        &format!("💬 HEART-TO-HEART CHAT WITH {}", pet.name.to_uppercase()),
        pet.theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!(
            "{}  •  {}  •  BOND {}%",
            pet.species.display_name(),
            pet.mood.badge(),
            pet.bond
        ),
        sp_primary,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(13),
            Constraint::Length(4),
        ])
        .split(lcd_inner);

    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                prompt.inner_thought,
                Style::default().fg(Color::LightCyan),
            )),
            Line::from(Span::styled(
                prompt.pet_line,
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ]))
        .alignment(Alignment::Center),
        zones[0],
    );

    f.render_widget(
        Paragraph::new(Text::from(render_colored_sprite(pet, app.tick_count)))
            .alignment(Alignment::Center),
        zones[1],
    );

    let opt1_style = if app.talk_selected_idx == 1 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let opt2_style = if app.talk_selected_idx == 2 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    f.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from(Span::styled(format!(" {} ", prompt.option_1), opt1_style)),
            Line::from(Span::styled(format!(" {} ", prompt.option_2), opt2_style)),
            Line::from(""),
            Line::from(Span::styled(
                "[↑/↓ or 1/2] Choose Response   •   [Enter] Reply   •   [Esc] Back",
                Style::default().fg(Color::Cyan),
            )),
        ]))
        .alignment(Alignment::Center),
        zones[2],
    );
}
