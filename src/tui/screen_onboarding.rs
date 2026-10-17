use super::*;

pub(super) fn stage_power_title(species: PetSpecies, stage: EvolutionStage) -> &'static str {
    match stage {
        EvolutionStage::Egg => "🥚 ELEMENTAL EGG — DORMANT SPIRIT",
        EvolutionStage::Baby => "🍼 BABY FORM — CUTE TINY HATCHLING",
        EvolutionStage::Teen => "🌱 TEEN FORM — AGILE STRIKER",
        EvolutionStage::Adult => "🌸 ADULT FORM — FIERCE GUARDIAN",
        EvolutionStage::CyberElder => match species {
            PetSpecies::IgnisDragon => "⚡ APEX ELDER — INFERNO BAHAMUT SOVEREIGN",
            PetSpecies::AzureWyrm => "⚡ APEX ELDER — TIDAL LEVIATHAN SOVEREIGN",
            PetSpecies::EmberFox => "⚡ APEX ELDER — NINE-TAILED KYUUBI SPIRIT",
            PetSpecies::PhoenixChick => "⚡ APEX ELDER — SOLAR GARUDA DEITY",
            PetSpecies::CrystalUni => "⚡ APEX ELDER — CELESTIAL KIRIN PEGASUS",
            PetSpecies::SlimeKing => "⚡ APEX ELDER — TITAN SLIME EMPEROR",
            PetSpecies::MochiCat => "⚡ APEX ELDER — BYAKKO SABER TIGER SHOGUN",
            PetSpecies::BobaPup => "⚡ APEX ELDER — FENRIR DIRE WOLF GUARDIAN",
            PetSpecies::CloudBun => "⚡ APEX ELDER — RAIJIN STORM VALKYRIE",
            PetSpecies::StarPengu => "⚡ APEX ELDER — EMPOLEON FROST KAISER",
        },
    }
}

pub(super) fn render_evolution_morph_bar(
    active_stage: EvolutionStage,
    auto_morph: bool,
) -> Line<'static> {
    let mut spans = Vec::with_capacity(12);
    for (idx, &stage) in EvolutionStage::all().iter().enumerate() {
        let is_cur = stage == active_stage;
        let style = if is_cur {
            Style::default()
                .fg(Color::Black)
                .bg(stage_color(stage))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(format!(" {} ", stage.badge()), style));
        if idx + 1 < EvolutionStage::all().len() {
            spans.push(Span::styled("▸", Style::default().fg(Color::DarkGray)));
        }
    }
    spans.push(Span::styled(
        if auto_morph { " 🔄Auto" } else { " ⏸Hold" },
        Style::default().fg(Color::LightCyan),
    ));
    Line::from(spans)
}

/// Step-by-step Onboarding with a balanced 3-Zone vertical layout (Header / Sprite Stage / Footer)
/// so nothing is cramped at the top and there is no empty void at the bottom!
pub(super) fn draw_onboarding_step_screen(f: &mut Frame, device: Rect, app: &App) {
    let species = PetSpecies::all()[app.selected_species_idx];
    let theme = ShellTheme::all()[app.selected_theme_idx];
    let preview_stage = EvolutionStage::all()[app.preview_stage_idx % EvolutionStage::all().len()];
    let (sp_primary, _, sp_accent) = species_palette(species);

    let shell = outer_shell_block(
        &format!(
            "🍡 MOCHIGOTCHI V-PET  •  NEW COMPANION  •  {}",
            app.sound.status_badge()
        ),
        theme,
    );
    let inner = shell.inner(device);
    f.render_widget(shell, device);

    match app.onboarding_step {
        OnboardingStep::SelectSpecies => {
            draw_onb_species(f, inner, app, species, preview_stage, sp_primary, sp_accent)
        }
        OnboardingStep::SelectTheme => draw_onb_theme(f, inner, app, species, theme, sp_primary),
        OnboardingStep::EnterName => draw_onb_name(f, inner, app, species, theme, sp_primary),
    }
}

pub(super) fn draw_onb_species(
    f: &mut Frame,
    inner: Rect,
    app: &App,
    species: PetSpecies,
    preview_stage: EvolutionStage,
    sp_primary: Color,
    sp_accent: Color,
) {
    let lcd = lcd_screen_block("STEP 1 OF 3  •  CHOOSE COMPANION", sp_primary);
    let lcd_inner = lcd.inner(inner);
    f.render_widget(lcd, inner);

    let zones = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(4),
        ])
        .split(lcd_inner);

    // Zone 1: Top Header with breathing room
    let header_lines = vec![
        Line::from(vec![
            Span::styled("◀  ", Style::default().fg(Color::LightYellow)),
            Span::styled(
                format!(
                    "{:02}/10  {}  ",
                    app.selected_species_idx + 1,
                    species.display_name()
                ),
                Style::default().fg(sp_primary).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                species.star_bar().to_string(),
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ▶", Style::default().fg(Color::LightYellow)),
        ]),
        render_evolution_morph_bar(preview_stage, app.preview_auto_morph),
        Line::from(Span::styled(
            stage_power_title(species, preview_stage),
            Style::default()
                .fg(stage_color(preview_stage))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    f.render_widget(
        Paragraph::new(Text::from(header_lines)).alignment(Alignment::Center),
        zones[0],
    );

    // Zone 2: 40×22 3/4 Side-Profile Pixel Art Stage
    let sprite_lines = render_colored_preview(species, preview_stage, app.tick_count);
    f.render_widget(
        Paragraph::new(Text::from(sprite_lines)).alignment(Alignment::Center),
        zones[1],
    );

    // Zone 3: Bottom Lore & Controls anchored cleanly at the bottom of the LCD
    let footer_lines = vec![
        Line::from(Span::styled(
            species.description().to_string(),
            Style::default().fg(Color::White),
        )),
        Line::from(vec![
            Span::styled(
                format!(
                    "{} • {} • Snack: ",
                    species.family(),
                    species.element_badge()
                ),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                species.favorite_snack().to_string(),
                Style::default().fg(Color::LightGreen),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "[←/→] Species   [V/Space] Step Form   [M] Auto-Morph   [Enter] Next Step",
            Style::default().fg(sp_accent).add_modifier(Modifier::BOLD),
        )),
    ];
    f.render_widget(
        Paragraph::new(Text::from(footer_lines)).alignment(Alignment::Center),
        zones[2],
    );
}
