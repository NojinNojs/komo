use super::*;

/// Render the 1-line living window backdrop above the sprite stage (width 40).
pub fn render_window_backdrop(pet: &Pet, tick: usize) -> Line<'static> {
    if !pet.is_alive {
        return Line::from(Span::styled(
            "  🕯  ·  ·  [ MEMORIAL SANCTUARY ]  ·  ·  🕯  ",
            Style::default().fg(Color::Gray),
        ));
    }
    if !pet.light_on {
        let stars = if (tick / 4).is_multiple_of(2) {
            "  ·  ✦  .  *  ☾  ·  .  ✦  ·  *  .  ✦  ·  "
        } else {
            "  ✦  ·  *  .  ☾  .  ✦  ·  .  *  ✦  ·  .  "
        };
        return Line::from(Span::styled(stars, Style::default().fg(Color::LightBlue)));
    }

    match (tick / 90) % 4 {
        0 => {
            let sky = if (tick / 6).is_multiple_of(2) {
                "  ☁   ·   ☀   ·   ☁       ·   ☁   ·     "
            } else {
                "    ☁   ·   ☀   ·   ☁   ·       ☁   ·   "
            };
            Line::from(Span::styled(sky, Style::default().fg(Color::LightYellow)))
        }
        1 => {
            let breeze = if (tick / 4).is_multiple_of(2) {
                "  ~  ·  ✿  ~   ·   ~  ✿  ·   ~   ·  ✿   "
            } else {
                "   ·  ~  ✿  ·  ~  ·   ~  ✿  ·  ~   ·    "
            };
            Line::from(Span::styled(breeze, Style::default().fg(Color::LightGreen)))
        }
        2 => {
            let rain = if (tick / 2).is_multiple_of(2) {
                "  ╱  ·  ╱   ╱  ·  ╱   ╱  ·  ╱   ╱  ·  ╱ "
            } else {
                "   ╱  ·  ╱   ╱  ·  ╱   ╱  ·  ╱   ╱  ·   "
            };
            Line::from(Span::styled(rain, Style::default().fg(Color::LightCyan)))
        }
        _ => {
            let night = if (tick / 5).is_multiple_of(2) {
                "  ·  ✦   .   ☾   ·   ✦   .   *   ·  ✦   "
            } else {
                "  ✦   ·   .  ☾   ✦   ·   *   .   ✦   ·  "
            };
            Line::from(Span::styled(
                night,
                Style::default().fg(Color::LightMagenta),
            ))
        }
    }
}

/// Render the textured 2-row habitat floor with food bowl, sleeping mat, and garden details.
pub fn render_habitat_floor(pet: &Pet, species: PetSpecies) -> Vec<Line<'static>> {
    let (_, secondary, accent) = crate::sprites::species_palette(species);
    let floor_color = if !pet.light_on {
        Color::Rgb(55, 65, 95)
    } else {
        secondary
    };
    let trim_color = if !pet.light_on {
        Color::Rgb(35, 42, 68)
    } else {
        accent
    };

    let bowl = if pet.hunger <= 25 {
        "🥣(empty)"
    } else if pet.hunger >= 80 {
        "🍙(full) "
    } else {
        "🍙(bowl) "
    };

    let bed = if pet.is_sleeping {
        "🛏[sleeping mat]"
    } else {
        "🛋[woven mat]  "
    };

    vec![
        Line::from(vec![
            Span::styled("▀▀▄▄", Style::default().fg(trim_color)),
            Span::styled(
                "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀",
                Style::default().fg(floor_color),
            ),
            Span::styled("▄▄▀▀", Style::default().fg(trim_color)),
        ]),
        Line::from(vec![
            Span::styled(format!("  {bowl}   •   "), Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    "Gaze: {:<16}",
                    compute_posture_and_gaze(pet, 0, 0).1.label()
                ),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("   •   {bed}  "), Style::default().fg(Color::Gray)),
        ]),
    ]
}
