//! Sprite submodule.
use super::*;

pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let g0 = build_grid(species, stage, mood, 0);
    let g1 = build_grid(species, stage, mood, 1);
    vec![render_grid_plain(&g0), render_grid_plain(&g1)]
}

/// Ambient retro status particles above the pixel sprite for all 13 rich emotions!
pub fn get_mood_particles(mood: PetMood, tick: usize) -> &'static str {
    let frames: &[&str] = match mood {
        PetMood::Idle => &[
            "·   ✦   ·   ★   ·   ✦   ·   ★   ·",
            "✦   ·   ★   ·   ✦   ·   ★   ·   ✦",
        ],
        PetMood::Eating => &[
            "♪   ♥   ★   ♥   ♪   ★   ♪   ♥   ♪",
            "★   ♪   ♥   ♪   ★   ♥   ★   ♪   ★",
        ],
        PetMood::Sleeping => &[
            "☾   z   Z   zZ  ·   ✦   ☾   z   Z",
            "✦   Z   zZ  z   ☾   ·   ✦   Z   z",
        ],
        PetMood::Sleepy => &[
            "·   z   ·   ☾   ·   z   ·   ☾   ·",
            "☾   ·   z   ·   ☾   ·   z   ·   ☾",
        ],
        PetMood::Happy | PetMood::Playful => &[
            "✦   ♥   ★   ♥   ✦   ♥   ✦   ★   ♥",
            "♥   ✦   ♥   ✦   ★   ✦   ♥   ✦   ★",
        ],
        PetMood::Excited | PetMood::Proud => &[
            "★   ✦   ★   ♦   ★   ✦   ★   ♦   ★",
            "♦   ★   ✦   ★   ♦   ★   ✦   ★   ♦",
        ],
        PetMood::Loved => &[
            "♥   ♥   ✦   ♥   ♥   ✦   ♥   ♥   ✦",
            "✦   ♥   ♥   ✦   ♥   ♥   ✦   ♥   ♥",
        ],
        PetMood::Lonely => &[
            ".   ·   💧  ·   .   ·   💧  ·   .",
            "·   .   ·   💧  ·   .   ·   💧  ·",
        ],
        PetMood::Angry => &[
            "💢  ·   ⚡  ·   💢  ·   ⚡  ·   💢",
            "⚡  ·   💢  ·   ⚡  ·   💢  ·   ⚡",
        ],
        PetMood::Overstuffed => &[
            "~   o   O   ~   o   O   ~   o   O",
            "O   ~   o   O   ~   o   O   ~   o",
        ],
        PetMood::Sick => &[
            ".   ~   x   ~   .   x   .   ~   x",
            "~   .   x   .   ~   .   x   .   ~",
        ],
        PetMood::Training => &[
            "★   ♦   ▲   ♦   ★   ▲   ★   ♦   ▲",
            "♦   ★   ▲   ★   ♦   ★   ♦   ▲   ★",
        ],
        PetMood::PassedOut => &[
            "✝   ·   👻  ·   ✦   ·   👻  ·   ✝",
            "·   👻  ·   ✦   ·   👻  ·   ✦   ·",
        ],
    };
    frames[(tick / 4) % frames.len()]
}

pub fn render_animated_sprite(pet: &Pet, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];

    let mut lines = vec![get_mood_particles(pet.mood, tick).to_string()];
    lines.extend(frame.lines().map(|l| l.to_string()));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("{} (needs bath [C]!)", messes.trim()));
    }
    lines
}

/// TrueColor 40×22 Half-Block Pixel Art [`Line`]s for the habitat LCD screen.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 8 == 7);
    let (walk_offset, gaze) = compute_posture_and_gaze(pet, tick, 0);
    let grid = build_grid_with_gaze(
        pet.species,
        pet.stage,
        pet.mood,
        frame_idx,
        walk_offset,
        gaze,
    );
    let (_, secondary, accent) = species_palette(pet.species);
    let floor_color = if !pet.light_on {
        Color::Rgb(55, 65, 95)
    } else {
        secondary
    };

    let mut out = Vec::with_capacity(15);
    out.push(render_window_backdrop(pet, tick));
    out.push(Line::from(Span::styled(
        get_mood_particles(pet.mood, tick).to_string(),
        Style::default().fg(accent),
    )));
    out.extend(render_grid_colored(&grid, pet.species));
    out.push(Line::from(vec![
        Span::styled("▀▀▄▄", Style::default().fg(accent)),
        Span::styled(
            "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀",
            Style::default().fg(floor_color),
        ),
        Span::styled("▄▄▀▀", Style::default().fg(accent)),
    ]));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        out.push(Line::from(Span::styled(
            format!("{} (press [C] to wash floor)", messes.trim()),
            Style::default().fg(Color::LightRed),
        )));
    }

    out
}

/// TrueColor 40×22 Half-Block Pixel Art [`Line`]s for the Step 1 Companion Selector.
pub fn render_colored_preview(
    species: PetSpecies,
    stage: EvolutionStage,
    tick: usize,
) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 8 == 7);
    let gaze = match (tick / 12) % 3 {
        0 => PetGaze::Center,
        1 => PetGaze::Left,
        _ => PetGaze::Right,
    };
    let grid = build_grid_with_gaze(species, stage, PetMood::Happy, frame_idx, 0, gaze);
    let (_, secondary, _) = species_palette(species);
    let mut out = render_grid_colored(&grid, species);
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        Style::default().fg(secondary),
    )));
    out
}

pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}
