use super::*;

pub(super) fn draw_pantry_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("🍱 TAMAGOTCHI PANTRY  •  MEALS & SNACKS", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        &format!(
            "COINS: {} 💰  •  SWEET STREAK: {}/3",
            app.pet.coins, app.pet.snacks_in_a_row
        ),
        Color::LightYellow,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut items: Vec<ListItem> = Vec::new();
    for (idx, choice) in FoodChoice::all().iter().enumerate() {
        let selected = idx == app.pantry_selected_idx;
        let marker = if selected { "▸" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        items.push(ListItem::new(vec![
            Line::from(Span::styled(
                format!("{marker} [{}] {}", idx + 1, choice.title(app.pet.species)),
                style,
            )),
            Line::from(Span::styled(
                format!("    {}", choice.summary()),
                Style::default().fg(Color::Gray),
            )),
        ]));
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(2)])
        .split(lcd_inner);

    f.render_widget(List::new(items), rows[0]);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[↑/↓ or 1-5] Select   [Enter] Feed   [Esc] Return to Habitat",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );
}

pub(super) fn draw_help_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block("📖 TAMAGOTCHI HARDWARE & KEYS GUIDE", app.pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block("CONTROLS & RULES", Color::LightMagenta);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let lines = vec![
        Line::from(Span::styled(
            "📟 Bezel Navigation (3-Button Hardware Style)",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • Button A (←/→): Select icon on top bezel"),
        Line::from(" • Button B (Enter): Open highlighted icon"),
        Line::from(" • Button C (Esc): Return to main LCD Habitat"),
        Line::from(Span::styled(
            "🍙 Meals, Sweet Snacks, Chat & 3-in-1 Arcade",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [F] Meal (+1 Hungry ♥, +1g) • [M] Pantry Menu"),
        Line::from(" • [Y] Heart-to-Heart Chat (see thoughts & reply!)"),
        Line::from(" • [G] 3-in-1 Arcade (1:Catcher 2:MindReader 3:Dash)"),
        Line::from(Span::styled(
            "💡 Room Light, Discipline, Revival & Sanctuary",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(" • [S] Sleep + [K] Light OFF • [C] Bath • [D] Scold"),
        Line::from(" • [H] Medicine / Phoenix Tear (1× max) • [B] 3-Step Vigil"),
        Line::from(" • [V] Wardrobe & Coats • [X] Photo Booth (PNG Share Card)"),
        Line::from(" • [O] Custom Caretaker Name • [R] Rename Pet • [L] Roster"),
        Line::from(Span::styled(
            "[Esc / Enter / ?] Return to Habitat",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    f.render_widget(Paragraph::new(Text::from(lines)), lcd_inner);
}

pub(super) fn draw_changelog_screen(f: &mut Frame, device: Rect, app: &App) {
    let shell = outer_shell_block(
        &format!("🎉 WHAT'S NEW IN KOMO v{}", CURRENT_APP_VERSION),
        app.pet.theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let lcd = lcd_screen_block(
        "✨ NEW COMPANION FEATURES & UPDATE GUIDE",
        Color::LightYellow,
    );
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "Welcome to KOMO v{}! Here is what is new for your companion:",
                CURRENT_APP_VERSION
            ),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for item in WHATS_NEW_FEATURES {
        lines.push(Line::from(Span::styled(
            format!(" • {item}"),
            Style::default().fg(Color::White),
        )));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "🔄 How to Update Next Time: run `komo --update` in your terminal!",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter / Space / Esc] Start Playing!",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    )));

    f.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        lcd_inner,
    );
}
