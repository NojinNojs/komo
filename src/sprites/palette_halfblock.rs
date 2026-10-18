//! Sprite submodule.
use super::*;

/// Primary, secondary, and accent RGB colors for each species.
pub fn species_palette(species: PetSpecies) -> (Color, Color, Color) {
    match species {
        PetSpecies::MochiCat => (
            Color::Rgb(255, 175, 205),
            Color::Rgb(255, 235, 245),
            Color::Rgb(255, 95, 145),
        ),
        PetSpecies::BobaPup => (
            Color::Rgb(220, 170, 120),
            Color::Rgb(130, 88, 56),
            Color::Rgb(255, 215, 120),
        ),
        PetSpecies::CloudBun => (
            Color::Rgb(210, 235, 255),
            Color::Rgb(255, 255, 255),
            Color::Rgb(255, 155, 195),
        ),
        PetSpecies::StarPengu => (
            Color::Rgb(110, 180, 255),
            Color::Rgb(240, 248, 255),
            Color::Rgb(255, 210, 85),
        ),
        PetSpecies::IgnisDragon => (
            Color::Rgb(245, 80, 75),
            Color::Rgb(255, 205, 115),
            Color::Rgb(255, 155, 50),
        ),
        PetSpecies::AzureWyrm => (
            Color::Rgb(65, 210, 235),
            Color::Rgb(195, 245, 255),
            Color::Rgb(120, 155, 255),
        ),
        PetSpecies::EmberFox => (
            Color::Rgb(255, 140, 65),
            Color::Rgb(255, 238, 205),
            Color::Rgb(90, 235, 255),
        ),
        PetSpecies::PhoenixChick => (
            Color::Rgb(255, 185, 55),
            Color::Rgb(255, 240, 165),
            Color::Rgb(255, 90, 60),
        ),
        PetSpecies::CrystalUni => (
            Color::Rgb(210, 165, 255),
            Color::Rgb(245, 235, 255),
            Color::Rgb(120, 230, 255),
        ),
        PetSpecies::SlimeKing => (
            Color::Rgb(95, 225, 190),
            Color::Rgb(180, 250, 230),
            Color::Rgb(255, 215, 80),
        ),
    }
}

/// Coat-aware palette: a non-`Classic` coat overrides the species' base RGB triple.
pub fn species_palette_with_coat(species: PetSpecies, coat: CoatPreset) -> (Color, Color, Color) {
    if let Some(((pr, pg, pb), (sr, sg, sb), (ar, ag, ab))) = coat.palette_override() {
        return (
            Color::Rgb(pr, pg, pb),
            Color::Rgb(sr, sg, sb),
            Color::Rgb(ar, ag, ab),
        );
    }
    species_palette(species)
}

pub(super) fn scale_rgb(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            ((r as f32 * factor).round() as u16).min(255) as u8,
            ((g as f32 * factor).round() as u16).min(255) as u8,
            ((b as f32 * factor).round() as u16).min(255) as u8,
        ),
        other => other,
    }
}

pub fn pixel_color_with_coat(code: char, species: PetSpecies, coat: CoatPreset) -> Option<Color> {
    let (primary, secondary, accent) = species_palette_with_coat(species, coat);
    match code {
        '.' => None,
        'O' | '0' => Some(Color::Rgb(27, 22, 44)),
        'D' => Some(scale_rgb(primary, 0.68)),
        'B' | 'b' => Some(primary),
        'L' => Some(scale_rgb(primary, 1.22)),
        'S' | 's' => Some(secondary),
        'A' | 'a' => Some(accent),
        'R' | 'r' => Some(Color::Rgb(255, 55, 75)),
        'W' | 'E' => Some(Color::Rgb(250, 252, 255)),
        'K' => Some(Color::Rgb(18, 14, 30)),
        'P' => Some(Color::Rgb(255, 110, 165)),
        'G' | 'g' => Some(Color::Rgb(255, 220, 70)),
        _ => Some(primary),
    }
}

pub fn pixel_color(code: char, species: PetSpecies) -> Option<Color> {
    pixel_color_with_coat(code, species, CoatPreset::Classic)
}

pub(super) fn render_grid_colored_with_coat(
    grid: &[[char; CANVAS_W]; CANVAS_H],
    species: PetSpecies,
    coat: CoatPreset,
) -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(CANVAS_H / 2);
    for pair in 0..(CANVAS_H / 2) {
        let top_row = &grid[pair * 2];
        let bot_row = &grid[pair * 2 + 1];
        let mut spans = Vec::with_capacity(CANVAS_W);

        for x in 0..CANVAS_W {
            let t = pixel_color_with_coat(top_row[x], species, coat);
            let b = pixel_color_with_coat(bot_row[x], species, coat);
            let span = match (t, b) {
                (None, None) => Span::raw(" "),
                (Some(tc), None) => Span::styled("▀", Style::default().fg(tc)),
                (None, Some(bc)) => Span::styled("▄", Style::default().fg(bc)),
                (Some(tc), Some(bc)) if tc == bc => Span::styled("█", Style::default().fg(tc)),
                (Some(tc), Some(bc)) => Span::styled("▀", Style::default().fg(tc).bg(bc)),
            };
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }
    lines
}

pub(super) fn render_grid_colored(
    grid: &[[char; CANVAS_W]; CANVAS_H],
    species: PetSpecies,
) -> Vec<Line<'static>> {
    render_grid_colored_with_coat(grid, species, CoatPreset::Classic)
}

pub(super) fn render_grid_plain(grid: &[[char; CANVAS_W]; CANVAS_H]) -> String {
    let mut rows = Vec::with_capacity(CANVAS_H / 2);
    for pair in 0..(CANVAS_H / 2) {
        let top_row = &grid[pair * 2];
        let bot_row = &grid[pair * 2 + 1];
        let mut line = String::with_capacity(CANVAS_W);
        for x in 0..CANVAS_W {
            let t = top_row[x] != '.';
            let b = bot_row[x] != '.';
            let ch = match (t, b) {
                (false, false) => ' ',
                (true, false) => '▀',
                (false, true) => '▄',
                (true, true) => '█',
            };
            line.push(ch);
        }
        rows.push(line);
    }
    rows.join("\n")
}
