use super::*;

fn pill_style(active: bool, bg: Color) -> Style {
    if active {
        Style::default()
            .fg(Color::Black)
            .bg(bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    }
}

fn wardrobe_row(sel: bool, name: &str, eq: bool, unl: bool, lv: u32, cost: u32, hl: Color) -> ListItem<'static> {
    let m = if sel { "▸" } else { " " };
    let b = if eq {
        "[EQUIPPED]".to_string()
    } else if unl {
        "[UNLOCKED]".to_string()
    } else {
        format!("[🔒 Lv.{lv} or {cost} 💰]")
    };
    let st = if sel {
        Style::default().fg(hl).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    ListItem::new(Line::from(Span::styled(format!("{m} {name} {b}"), st)))
}

pub(super) fn draw_wardrobe_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let shell = outer_shell_block("👗 COMPANION WARDROBE & COAT STUDIO", pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let (cn, an) = (pet.coat.display_name(), pet.accessory.display_name());
    let title = format!("COINS: {} 💰  •  LV.{}  •  WORN: {cn} + {an}", pet.coins, pet.level);
    let lcd = lcd_screen_block(&title, Color::LightMagenta);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(12), Constraint::Length(2)])
        .split(lcd_inner);

    let tab0 = pill_style(app.wardrobe_tab == 0, Color::LightYellow);
    let tab1 = pill_style(app.wardrobe_tab == 1, Color::LightCyan);
    let tabs = Line::from(vec![
        Span::styled(" [1] Coat Presets ", tab0),
        Span::raw("   "),
        Span::styled(" [2] Accessories ", tab1),
    ]);
    f.render_widget(Paragraph::new(tabs).alignment(Alignment::Center), rows[0]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(rows[1]);

    let items: Vec<ListItem> = if app.wardrobe_tab == 0 {
        CoatPreset::all()
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let s = i == app.wardrobe_coat_idx;
                let u = pet.is_coat_unlocked(c);
                wardrobe_row(s, c.display_name(), pet.coat == c, u, c.unlock_level(), c.unlock_cost(), Color::LightYellow)
            })
            .collect()
    } else {
        Accessory::all()
            .iter()
            .enumerate()
            .map(|(i, &a)| {
                let s = i == app.wardrobe_acc_idx;
                let u = pet.is_accessory_unlocked(a);
                wardrobe_row(s, a.display_name(), pet.accessory == a, u, a.unlock_level(), a.unlock_cost(), Color::LightCyan)
            })
            .collect()
    };
    f.render_widget(List::new(items), cols[0]);
    let sprite = Paragraph::new(Text::from(render_colored_sprite(pet, app.tick_count)));
    f.render_widget(sprite.alignment(Alignment::Center), cols[1]);

    let foot_st = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let foot = Span::styled(
        "[↑/↓] Select  •  [Tab/←/→] Coats/Gear  •  [Enter/Space] Equip/Buy  •  [Esc/V] Close",
        foot_st,
    );
    f.render_widget(Paragraph::new(Line::from(foot)).alignment(Alignment::Center), rows[2]);
}

pub(super) fn draw_photo_booth_screen(f: &mut Frame, device: Rect, app: &App) {
    let pet = &app.pet;
    let shell = outer_shell_block("📸 RETRO PIXEL-ART PHOTO BOOTH", pet.theme);
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    let (nw, nh) = app.photo_ratio.native();
    let s = app.photo_ratio.scale();
    let name_up = pet.name.to_uppercase();
    let title = format!("RATIO {} ({}×{} PX)  •  {name_up}", app.photo_ratio.label(), nw * s, nh * s);
    let lcd = lcd_screen_block(&title, Color::LightCyan);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(11), Constraint::Length(4)])
        .split(lcd_inner);

    let ratio_labels = [
        (ShareRatio::Landscape16x9, "16:9 (1152×648)"),
        (ShareRatio::Story9x16, "9:16 Story (648×1152)"),
        (ShareRatio::Classic4x3, "4:3 Classic (864×648)"),
        (ShareRatio::Portrait3x4, "3:4 Portrait (648×864)"),
    ];
    let mut ratio_spans = Vec::with_capacity(8);
    for (idx, (r, text)) in ratio_labels.iter().enumerate() {
        let st = pill_style(*r == app.photo_ratio, Color::LightYellow);
        ratio_spans.push(Span::styled(format!(" [{}] {text} ", idx + 1), st));
        if idx < 3 {
            ratio_spans.push(Span::raw(" "));
        }
    }
    f.render_widget(Paragraph::new(Line::from(ratio_spans)).alignment(Alignment::Center), zones[0]);
    let sprite = Paragraph::new(Text::from(render_colored_sprite(pet, app.tick_count)));
    f.render_widget(sprite.alignment(Alignment::Center), zones[1]);

    let (cn, an, rk) = (pet.coat.display_name(), pet.accessory.display_name(), pet.care_rank().badge());
    let out = default_share_output_path(pet, app.photo_ratio);
    let default_st = format!("Target PNG: {}  •  {cn}  •  {rk}", out.display());
    let status = app.photo_status.clone().unwrap_or(default_st);
    let st_green = Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
    let st_cyan = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let lines = vec![
        Line::from(Span::styled(format!("Coat: {cn}  •  Gear: {an}  •  Rank: {rk}"), Style::default().fg(Color::White))),
        Line::from(Span::styled(status, st_green)),
        Line::from(Span::styled(
            "[←/→ or 1-4] Aspect Ratio  •  [Enter / Space] Snap PNG Card  •  [Esc / X] Close",
            st_cyan,
        )),
    ];
    f.render_widget(Paragraph::new(Text::from(lines)).alignment(Alignment::Center), zones[2]);
}
