//! Sprite submodule.
use super::*;

/// Build the 40×22 character grid for any `(species, stage, mood, frame_idx, walk_offset, gaze)`.
pub fn build_grid_with_gaze(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
    walk_offset: isize,
    gaze: PetGaze,
) -> [[char; CANVAS_W]; CANVAS_H] {
    let raw = if mood == PetMood::PassedOut {
        ghost_bitmap()
    } else {
        match stage {
            EvolutionStage::Egg => egg_bitmap(species, frame_idx),
            EvolutionStage::Baby => baby_bitmap(species),
            EvolutionStage::Teen => teen_bitmap(species),
            EvolutionStage::Adult => adult_bitmap(species),
            EvolutionStage::CyberElder => elder_bitmap(species),
        }
    };

    let mut grid = [['.'; CANVAS_W]; CANVAS_H];
    for (y, row) in raw.iter().enumerate() {
        for (x, ch) in row.chars().take(CANVAS_W).enumerate() {
            grid[y][x] = ch;
        }
    }

    // Apply 3-way gaze: Left (3/4 left), Right (3/4 right), or Center (looking directly at you!)
    if stage != EvolutionStage::Egg && mood != PetMood::PassedOut {
        match gaze {
            PetGaze::Left => {}
            PetGaze::Right => {
                for row in &mut grid {
                    row.reverse();
                }
            }
            PetGaze::Center => {
                apply_front_gaze(&mut grid, species, stage);
            }
        }
    }

    // Apply horizontal walking offset (-3..=3 columns) clamped so non-empty pixels never clip!
    if walk_offset != 0 && stage != EvolutionStage::Egg {
        let mut min_x = CANVAS_W;
        let mut max_x = 0usize;
        for row in &grid {
            for (x, &c) in row.iter().enumerate() {
                if c != '.' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                }
            }
        }
        if min_x <= max_x {
            let safe_shift = walk_offset
                .max(-(min_x as isize))
                .min((CANVAS_W - 1 - max_x) as isize);
            if safe_shift != 0 {
                let mut shifted_h = [['.'; CANVAS_W]; CANVAS_H];
                for (src_row, dst_row) in grid.iter().zip(shifted_h.iter_mut()) {
                    for (x, &cell) in src_row.iter().enumerate() {
                        let nx = x as isize + safe_shift;
                        if (0..CANVAS_W as isize).contains(&nx) {
                            dst_row[nx as usize] = cell;
                        }
                    }
                }
                grid = shifted_h;
            }
        }
    }

    // Ground-align the sprite so its feet rest directly on the bottom row (above the pedestal!)
    let mut last_non_empty = 0usize;
    for (y, row) in grid.iter().enumerate() {
        if row.iter().any(|&c| c != '.') {
            last_non_empty = y;
        }
    }
    let shift_down = (CANVAS_H - 1).saturating_sub(last_non_empty);
    if shift_down > 0 {
        let mut aligned = [['.'; CANVAS_W]; CANVAS_H];
        aligned[shift_down..(last_non_empty + shift_down + 1)]
            .copy_from_slice(&grid[..(last_non_empty + 1)]);
        grid = aligned;
    }

    // Apply full-body posture & rich expressions (Sleeping mat curl, Hungry slouch, Full, Sick)
    apply_pose_and_expression(&mut grid, stage, mood, frame_idx);

    // Natural blink on frame 1 when awake
    let blink = frame_idx % 2 == 1 || matches!(mood, PetMood::Sleeping | PetMood::Sleepy);
    if stage != EvolutionStage::Egg && mood != PetMood::PassedOut {
        if blink {
            for row in &mut grid {
                for cell in row.iter_mut() {
                    if *cell == 'E' || *cell == 'K' || (mood == PetMood::Sleeping && *cell == 'R') {
                        *cell = 'O';
                    }
                }
            }
        } else if mood == PetMood::Angry {
            for row in &mut grid {
                for cell in row.iter_mut() {
                    if *cell == 'E' {
                        *cell = 'R';
                    }
                }
            }
        }
    }

    // Gentle 1-pixel vertical breathing hop on frame 1 when awake & hatched
    if stage != EvolutionStage::Egg
        && !matches!(
            mood,
            PetMood::Sleeping | PetMood::PassedOut | PetMood::Lonely | PetMood::Sick
        )
        && frame_idx % 2 == 1
    {
        let mut shifted = [['.'; CANVAS_W]; CANVAS_H];
        shifted[..CANVAS_H - 1].copy_from_slice(&grid[1..CANVAS_H]);
        grid = shifted;
    }

    grid
}

/// Build the 40×22 grid for a full [`Pet`] with equipped Coat palette, Accessory, and Hygiene/Activity props.
pub fn build_pet_grid(
    pet: &Pet,
    frame_idx: usize,
    walk_offset: isize,
    gaze: PetGaze,
) -> [[char; CANVAS_W]; CANVAS_H] {
    let mut grid = build_grid_with_gaze(
        pet.species,
        pet.stage,
        pet.mood,
        frame_idx,
        walk_offset,
        gaze,
    );
    apply_accessory_overlay(&mut grid, pet.stage, pet.mood, pet.accessory);
    if pet.stage != EvolutionStage::Egg
        && pet.is_alive
        && (pet.cleanliness <= 45 || pet.poop_count > 0)
    {
        // Animated stink waves & buzzing flies around a dirty/smelly companion
        let wy = if frame_idx.is_multiple_of(2) { 2 } else { 3 };
        for x in [6usize, 33usize] {
            if grid[wy][x] == '.' {
                grid[wy][x] = 'G';
            }
            if grid[wy + 1][x] == '.' {
                grid[wy + 1][x] = 'O';
            }
        }
        let fx = if frame_idx.is_multiple_of(2) { 9 } else { 30 };
        if grid[4][fx] == '.' {
            grid[4][fx] = 'K';
        }
    }
    grid
}

pub(super) fn build_grid_with_motion(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
    walk_offset: isize,
    facing_right: bool,
) -> [[char; CANVAS_W]; CANVAS_H] {
    let gaze = if facing_right {
        PetGaze::Right
    } else {
        PetGaze::Left
    };
    build_grid_with_gaze(species, stage, mood, frame_idx, walk_offset, gaze)
}

pub(super) fn build_grid(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
) -> [[char; CANVAS_W]; CANVAS_H] {
    build_grid_with_motion(species, stage, mood, frame_idx, 0, false)
}
