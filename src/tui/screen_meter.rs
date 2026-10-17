use super::*;

pub(super) fn draw_meter_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let shell = outer_shell_block("📏 TAMAGOTCHI STATUS METER", pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let page_idx = app.meter_page % 4;
    let title = if pet.stage == EvolutionStage::Egg {
        "🥚 Egg Incubator Status".to_string()
    } else {
        format!(
            "📊 Tamagotchi Vitals  •  Page {}/4 (←/→ or Space)",
            page_idx + 1
        )
    };
    let lcd = lcd_screen_block(&title, Color::LightCyan);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut lines = Vec::new();
    if pet.stage == EvolutionStage::Egg {
        let warm_cd = pet.warm_cooldown_remaining(now);
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let step_boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Egg Incubator Status",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        lines.push(custom_meter_line(
            "🥚 Incubation",
            pet.incubation,
            Color::LightYellow,
            "Hatch at 100%",
            Color::LightYellow,
        ));
        lines.push(Line::from(""));
        lines.push(Line::from(format!(
            "🔥 Warmings:   [{}] ({warm_sessions}/5 sessions)",
            step_boxes.trim()
        )));
        lines.push(Line::from(format!(
            "⏳ Warm Timer: {}",
            if warm_cd == 0 {
                "Ready now! Press [W]".to_string()
            } else {
                format!("Wait {}", format_duration_human(warm_cd))
            }
        )));
        lines.push(Line::from(""));
        lines.push(custom_meter_line(
            "🤝 Bond",
            pet.bond,
            Color::LightBlue,
            "Keeper Bond",
            Color::LightBlue,
        ));
    } else {
        push_meter_page_lines(&mut lines, app, pet, page_idx);
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[←/→ or Space] Flip Page   •   [Esc / I] Return to Habitat",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    f.render_widget(Paragraph::new(Text::from(lines)), lcd_inner);
}
