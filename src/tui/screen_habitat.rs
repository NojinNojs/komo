use super::*;

pub(super) fn draw_habitat_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let now = Utc::now();
    let (sp_primary, _, _) = species_palette(pet.species);
    let selected_icon = LcdIcon::selectable()[app.selected_icon_idx % LcdIcon::selectable().len()];

    let shell_title = format!(
        "🌿 KOMO • {} ({}) • KEEPER: {} • {}",
        pet.name.to_uppercase(),
        pet.species.display_name(),
        pet.caretaker_label().to_uppercase(),
        app.sound.status_badge()
    );
    let shell = outer_shell_block(&shell_title, pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(18),
            Constraint::Length(2),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(render_unified_bezel(selected_icon, pet)).alignment(Alignment::Center),
        rows[0],
    );

    let (sky, _) = sky_scene(app.tick_count);
    let lcd_title = if !pet.is_alive {
        format!(
            "🕯 MEMORIAL VIGIL ({}/3)  •  {}  •  [B] VIGIL / [H] REVIVE",
            pet.vigil_steps_completed, pet.name
        )
    } else if pet.stage == EvolutionStage::Egg {
        format!(
            "{}  •  🥚 EGG ({}% WARM)  •  {}",
            sky,
            pet.incubation,
            pet.mood.badge()
        )
    } else {
        format!(
            "{}  •  {} LV.{}  •  {}  •  💰{}  •  {}",
            sky,
            pet.stage.badge(),
            pet.level,
            pet.mood.badge(),
            pet.coins,
            pet.attention_call.badge()
        )
    };
    let lcd = lcd_screen_block(&lcd_title, sp_primary);
    let lcd_inner = lcd.inner(rows[1]);
    f.render_widget(lcd, rows[1]);

    let lcd_zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(13),
            Constraint::Length(2),
        ])
        .split(lcd_inner);

    // Zone 1: Live Sensory Perception (`👁 Sight / 👂 Hearing / 🫳 Feeling`) + Speech (`💬`)
    let snap = sense_environment(
        pet,
        app.tick_count,
        selected_icon,
        app.recent_key_ticks,
        app.key_burst,
    );
    let sensory_line = if let Some(ref thought) = pet.last_pet_thought {
        thought.clone()
    } else {
        match (app.tick_count / 28) % 3 {
            0 => format!("👁 Sight ({}) • {}", snap.gaze.short(), snap.sight),
            1 => format!("👂 Hearing • {}", snap.hearing),
            _ => format!("🫳 Feeling • {}", snap.feeling),
        }
    };

    // Word-aware fit so the retro LCD never shows a cut word in the sensory/speech feed.
    let fit_text = |text: String, max: usize| -> String {
        let chars: Vec<char> = text.chars().collect();
        if chars.len() > max {
            let cut = chars[..max.saturating_sub(1)]
                .iter()
                .rposition(|c| c.is_whitespace());
            let take = cut.unwrap_or(max.saturating_sub(1));
            format!("{}…", chars[..take].iter().collect::<String>().trim_end())
        } else {
            text
        }
    };
    let text_budget = (lcd_zones[0].width as usize).saturating_sub(6).max(36);
    let sensory_line = fit_text(sensory_line, text_budget);

    let speech_line = if !pet.light_on && pet.is_alive && pet.last_pet_speech.is_none() {
        Line::from(Span::styled(
            "🌙 [ ROOM LIGHT OFF — NIGHT MODE ]",
            Style::default().fg(Color::LightBlue),
        ))
    } else {
        Line::from(Span::styled(
            fit_text(speech_bubble(pet, app.tick_count), text_budget),
            Style::default()
                .fg(mood_color(pet.mood))
                .add_modifier(Modifier::BOLD),
        ))
    };

    let bubble_lines = vec![
        Line::from(Span::styled(
            sensory_line,
            Style::default().fg(Color::LightCyan),
        )),
        speech_line,
        Line::from(""),
    ];
    f.render_widget(
        Paragraph::new(Text::from(bubble_lines)).alignment(Alignment::Center),
        lcd_zones[0],
    );

    // Zone 2: 40×22 Pixel Art Sprite Stage (with autonomous strolling!)
    let sprite = render_colored_sprite(pet, app.tick_count);
    f.render_widget(
        Paragraph::new(Text::from(sprite)).alignment(Alignment::Center),
        lcd_zones[1],
    );
    draw_habitat_vitals(f, lcd_zones[2], pet);
    draw_habitat_footer(f, rows[2], app, pet, now);
}
