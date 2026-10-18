use super::*;

pub(super) fn draw_habitat_vitals(f: &mut Frame, area: Rect, pet: &Pet) {
    // Zone 3: Vitals Bar anchored cleanly at the bottom of the LCD box
    let vitals_line = if pet.stage == EvolutionStage::Egg {
        let warm_sessions = (pet.incubation / 20).min(5) as usize;
        let boxes = format!(
            "{}{}",
            "■ ".repeat(warm_sessions),
            "□ ".repeat(5usize.saturating_sub(warm_sessions))
        );
        Line::from(Span::styled(
            format!(
                "Incubation: [{}] {}%   •   Bond: {}%",
                boxes.trim(),
                pet.incubation,
                pet.bond
            ),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
    } else {
        let poop_suffix = if pet.poop_count > 0 {
            format!("   💩×{}", pet.poop_count)
        } else {
            String::new()
        };
        Line::from(vec![
            Span::styled(
                format!("🍙 {}   ", format_hearts(pet.hungry_hearts())),
                Style::default().fg(if pet.hunger <= 15 {
                    Color::LightRed
                } else {
                    Color::LightGreen
                }),
            ),
            Span::styled(
                format!("💖 {}   ", format_hearts(pet.happy_hearts())),
                Style::default().fg(Color::LightMagenta),
            ),
            Span::styled(
                format!("💚 {}%   ", pet.health),
                Style::default().fg(if pet.health <= 35 {
                    Color::LightRed
                } else {
                    Color::LightCyan
                }),
            ),
            Span::styled(
                format!(
                    "⚡ {}%   ⚖ {}{poop_suffix}",
                    pet.energy,
                    pet.format_weight()
                ),
                Style::default().fg(Color::Yellow),
            ),
        ])
    };
    f.render_widget(
        Paragraph::new(Text::from(vec![Line::from(""), vitals_line])).alignment(Alignment::Center),
        area,
    );
}

pub(super) fn draw_habitat_footer(
    f: &mut Frame,
    area: Rect,
    app: &App,
    pet: &Pet,
    now: chrono::DateTime<Utc>,
) {
    // Bottom 2-line Footer
    let footer_lines = if let Some(buf) = app.rename_buffer.as_ref() {
        vec![
            Line::from(Span::styled(
                format!("🔖 Rename Companion: [ {buf}▏ ]  (Enter = Save, Esc = Cancel)"),
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ]
    } else if let Some(buf) = app.caretaker_buffer.as_ref() {
        vec![
            Line::from(Span::styled(
                format!("🔖 Call You As: [ {buf}▏ ]  (Enter = Save, Esc = Cancel)"),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ]
    } else if pet.stage == EvolutionStage::Egg {
        vec![
            Line::from(Span::styled(
                pet.advisor_tip(now),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                "[W] Warm  [Y] Talk  [O] Call-Me  [I] Meter  [L] Pets  [U] Sound  [?] Keys",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                pet.advisor_tip(now),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                "[←/→+Enter] Bezel  [F] Meal  [Y] Talk  [V] Wardrobe  [X] Photo  [G] Arcade  [I] Meter  [?]",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    };
    f.render_widget(
        Paragraph::new(Text::from(footer_lines)).alignment(Alignment::Center),
        area,
    );
}
