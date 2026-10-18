//! Sprite submodule: crisp pixel-art accessory overlays stamped onto the companion.
use super::*;

struct Head {
    top_y: usize,
    eye_y: usize,
    cx: isize,
    min_x: isize,
    max_x: isize,
}

fn put(grid: &mut [[char; CANVAS_W]; CANVAS_H], y: isize, x: isize, c: char) {
    if y >= 0 && x >= 0 && (y as usize) < CANVAS_H && (x as usize) < CANVAS_W {
        grid[y as usize][x as usize] = c;
    }
}

fn head_metrics(grid: &[[char; CANVAS_W]; CANVAS_H]) -> Option<Head> {
    let top_y = grid.iter().position(|row| row.iter().any(|&c| c != '.'))?;
    let eye_y = grid
        .iter()
        .position(|r| r.iter().any(|&c| c == 'E' || c == 'K'))
        .unwrap_or(top_y);
    let head_bottom = (eye_y + 2).min(CANVAS_H - 1);
    let mut min_x = CANVAS_W as isize;
    let mut max_x = -1isize;
    for row in &grid[top_y..=head_bottom] {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                min_x = min_x.min(x as isize);
                max_x = max_x.max(x as isize);
            }
        }
    }
    (max_x >= min_x).then(|| Head {
        top_y,
        eye_y,
        cx: (min_x + max_x) / 2,
        min_x,
        max_x,
    })
}

/// Stamp the equipped accessory onto the companion's head/neck region.
pub fn apply_accessory_overlay(
    grid: &mut [[char; CANVAS_W]; CANVAS_H],
    stage: EvolutionStage,
    mood: PetMood,
    accessory: Accessory,
) {
    if stage == EvolutionStage::Egg || mood == PetMood::PassedOut || accessory == Accessory::None {
        return;
    }
    let Some(head) = head_metrics(grid) else {
        return;
    };
    match accessory {
        Accessory::None => {}
        Accessory::SilkRibbon => draw_ribbon(grid, &head),
        Accessory::BellCollar => draw_collar(grid, &head),
        Accessory::CozyScarf => draw_scarf(grid, &head),
        Accessory::TinyCrown => draw_crown(grid, &head),
        Accessory::WizardHat => draw_hat(grid, &head),
        Accessory::StarlightHalo => draw_halo(grid, &head),
    }
}

fn draw_ribbon(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let x = head.cx + 1;
    let y = head.top_y as isize;
    put(grid, y, x - 1, 'P');
    put(grid, y, x, 'G');
    put(grid, y, x + 1, 'P');
    put(grid, y + 1, x - 1, 'P');
    put(grid, y + 1, x, 'G');
    put(grid, y + 1, x + 1, 'P');
}

fn draw_collar(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let y = (head.eye_y + 3).min(CANVAS_H - 3) as isize;
    for x in (head.min_x - 1)..=(head.max_x + 1) {
        put(grid, y, x, 'R');
    }
    put(grid, y, head.cx, 'G');
}

fn draw_scarf(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let y = (head.eye_y + 3).min(CANVAS_H - 3) as isize;
    for x in (head.min_x - 1)..=(head.max_x + 1) {
        put(grid, y, x, if x % 2 == 0 { 'A' } else { 'P' });
    }
    put(grid, y + 1, head.cx - 1, 'A');
    put(grid, y + 1, head.cx, 'P');
    put(grid, y + 1, head.cx + 1, 'A');
}

fn draw_crown(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let y = head.top_y.saturating_sub(2) as isize;
    for dx in [-4isize, 0, 4] {
        put(grid, y, head.cx + dx, 'G');
    }
    for dy in 1..=2isize {
        for dx in -4..=4isize {
            put(grid, y + dy, head.cx + dx, 'G');
        }
    }
    put(grid, y + 1, head.cx, 'R');
}

fn draw_hat(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let tip = head.top_y.saturating_sub(3) as isize;
    for dx in -4..=4isize {
        put(grid, tip + 3, head.cx + dx, 'O');
    }
    for dx in -2..=2isize {
        put(grid, tip + 2, head.cx + dx, 'A');
    }
    for dx in -1..=1isize {
        put(grid, tip + 1, head.cx + dx, 'A');
    }
    put(grid, tip, head.cx, 'G');
}

fn draw_halo(grid: &mut [[char; CANVAS_W]; CANVAS_H], head: &Head) {
    let y = head.top_y.saturating_sub(2) as isize;
    for dx in -2..=2isize {
        put(grid, y, head.cx + dx, if dx.abs() == 2 { 'G' } else { 'A' });
    }
    for dx in [-3isize, -2, 2, 3] {
        let c = if dx.abs() == 3 { 'G' } else { 'A' };
        put(grid, y + 1, head.cx + dx, c);
    }
}
