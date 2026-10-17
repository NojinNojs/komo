//! Sprite submodule.
use super::*;

/// Transform the head/face region into a symmetric **Front-Facing Gaze (`PetGaze::Center`)**
/// so the companion looks directly at the player with two expressive eyes!
pub(super) fn apply_front_gaze(
    grid: &mut [[char; CANVAS_W]; CANVAS_H],
    species: PetSpecies,
    stage: EvolutionStage,
) {
    if stage == EvolutionStage::Egg {
        return;
    }

    // Find the first eye pixel ('E' or 'K') to anchor the face center
    let mut eye_pos: Option<(usize, usize)> = None;
    'search: for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'E' || c == 'K' {
                eye_pos = Some((x, y));
                break 'search;
            }
        }
    }

    let Some((_, eye_y)) = eye_pos else {
        return;
    };

    // Paint a clear front-facing pair of eyes and central nose/beak on the head rows
    let y0 = eye_y.min(CANVAS_H - 3);
    let y1 = (y0 + 1).min(CANVAS_H - 2);
    let cx = match stage {
        EvolutionStage::Baby => 19,
        EvolutionStage::Teen => 17,
        EvolutionStage::Adult => 14,
        EvolutionStage::CyberElder => 12,
        EvolutionStage::Egg => 20,
    };

    if cx >= 5 && cx + 5 < CANVAS_W {
        // Ensure solid face backdrop between the two front-facing eyes
        for dx in -4isize..=4 {
            let x = (cx as isize + dx) as usize;
            if grid[y0][x] == '.' {
                grid[y0][x] = 'B';
            }
            if grid[y1][x] == '.' {
                grid[y1][x] = 'B';
            }
        }
        // Left eye looking at you
        grid[y0][cx - 3] = 'E';
        grid[y0][cx - 2] = 'K';
        grid[y1][cx - 3] = 'K';
        grid[y1][cx - 2] = 'K';
        // Right eye looking at you
        grid[y0][cx + 2] = 'E';
        grid[y0][cx + 3] = 'K';
        grid[y1][cx + 2] = 'K';
        grid[y1][cx + 3] = 'K';
        // Rosy cheeks & center nose/beak
        grid[y1][cx - 4] = 'P';
        grid[y1][cx + 4] = 'P';
        grid[y1][cx] = match species {
            PetSpecies::StarPengu | PetSpecies::PhoenixChick => 'G',
            _ => 'O',
        };
    }
}

/// Apply full-body posture and detailed facial expressions for Sleeping, Hungry, Full, Sick, etc.
pub(super) fn apply_pose_and_expression(
    grid: &mut [[char; CANVAS_W]; CANVAS_H],
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
) {
    if stage == EvolutionStage::Egg || mood == PetMood::PassedOut {
        return;
    }

    // 1. Sleeping Pose: curl down low onto a woven sleeping cushion with rising zZz!
    if mood == PetMood::Sleeping {
        let mut curled = [['.'; CANVAS_W]; CANVAS_H];
        // Drop the upper body down by 3 rows so legs tuck into a lying/curled posture
        let drop = if frame_idx.is_multiple_of(2) { 3 } else { 4 };
        curled[drop..(CANVAS_H - 2)].copy_from_slice(&grid[..((CANVAS_H - 2) - drop)]);
        // Close eyes into peaceful sleeping arcs
        for row in &mut curled {
            for cell in row.iter_mut() {
                if *cell == 'E' || *cell == 'K' || *cell == 'R' {
                    *cell = 'O';
                }
            }
        }
        // Draw cozy woven sleeping mat under the curled companion
        for (x, cell) in curled[CANVAS_H - 2].iter_mut().enumerate().take(34).skip(6) {
            *cell = if x % 2 == 0 { 'S' } else { 'A' };
        }
        for cell in curled[CANVAS_H - 1].iter_mut().take(34).skip(6) {
            *cell = 'O';
        }
        // Pixel 'z' floating above the sleeping head
        let z_y = if frame_idx.is_multiple_of(2) { 2 } else { 1 };
        curled[z_y][28] = 'A';
        curled[z_y][29] = 'A';
        curled[z_y + 1][28] = 'A';
        *grid = curled;
        return;
    }

    // 2. Hungry / Lonely Slouch: droop top rows by 1 pixel, watery eyes, belly rumble wave
    if mood == PetMood::Lonely {
        let mut slouched = [['.'; CANVAS_W]; CANVAS_H];
        slouched[1..CANVAS_H].copy_from_slice(&grid[..CANVAS_H - 1]);
        for row in &mut slouched {
            for cell in row.iter_mut() {
                if *cell == 'E' {
                    *cell = 'A'; // watery tear glint
                }
            }
        }
        // Belly rumble wave on the side
        if frame_idx.is_multiple_of(2) {
            slouched[13][4] = 'G';
            slouched[14][3] = 'G';
        }
        *grid = slouched;
        return;
    }

    // 3. Full / Eating / Overstuffed Pose: happy curved eyes, warm blush, plump belly
    if matches!(mood, PetMood::Eating | PetMood::Overstuffed) {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 'E' {
                    *cell = 'O';
                } else if *cell == 'K' {
                    *cell = if mood == PetMood::Overstuffed {
                        'P'
                    } else {
                        'O'
                    };
                }
            }
        }
    }

    // 4. Sick / Feverish Pose: heavy half-lidded eyes, fever cheeks, cold sweat drop
    if mood == PetMood::Sick {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 'E' {
                    *cell = 'R';
                }
            }
        }
        grid[4][8] = 'A';
        grid[5][8] = 'W';
    }
}
