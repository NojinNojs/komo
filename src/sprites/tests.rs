//! Sprite rendering tests.
use super::*;

#[test]
fn test_all_species_stages_and_moods_render_40x22_half_block_pixel_sprites() {
    for species in PetSpecies::all() {
        let baby = get_sprite_frames(*species, EvolutionStage::Baby, PetMood::Idle);
        let teen = get_sprite_frames(*species, EvolutionStage::Teen, PetMood::Idle);
        let adult = get_sprite_frames(*species, EvolutionStage::Adult, PetMood::Idle);
        let elder = get_sprite_frames(*species, EvolutionStage::CyberElder, PetMood::Idle);
        assert_ne!(baby[0], teen[0]);
        assert_ne!(teen[0], adult[0]);
        assert_ne!(adult[0], elder[0]);

        for stage in EvolutionStage::all() {
            for mood in PetMood::all() {
                let frames = get_sprite_frames(*species, *stage, *mood);
                assert_eq!(frames.len(), 2, "{species:?}/{stage:?}/{mood:?}");
                for frame in &frames {
                    assert!(!frame.trim().is_empty());
                    let lines: Vec<&str> = frame.lines().collect();
                    assert_eq!(lines.len(), CANVAS_H / 2);
                    for line in lines {
                        assert_eq!(line.chars().count(), CANVAS_W);
                    }
                }
            }
        }
    }
}

#[test]
fn test_white_body_pixels_do_not_turn_dark_when_blinking() {
    let g0 = build_grid(
        PetSpecies::StarPengu,
        EvolutionStage::Adult,
        PetMood::Idle,
        0,
    );
    let g1 = build_grid(
        PetSpecies::StarPengu,
        EvolutionStage::Adult,
        PetMood::Idle,
        1,
    );
    let w_count_0 = g0.iter().flatten().filter(|&&c| c == 'W').count();
    let w_count_1 = g1.iter().flatten().filter(|&&c| c == 'W').count();
    assert_eq!(w_count_0, w_count_1);
}

#[test]
fn test_autonomous_strolling_and_slow_readable_thoughts() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Adult;
    let s0 = render_colored_sprite(&pet, 0);
    let s16 = render_colored_sprite(&pet, 16);
    assert!(!s0.is_empty());
    assert!(!s16.is_empty());
    assert_eq!(thought_bubble(&pet, 0), thought_bubble(&pet, 30));
    assert_eq!(speech_bubble(&pet, 0), speech_bubble(&pet, 30));
}

#[test]
fn test_colors_particles_and_previews() {
    for theme in ShellTheme::all() {
        let _ = theme_color(*theme);
    }
    for stage in EvolutionStage::all() {
        let _ = stage_color(*stage);
    }
    for mood in PetMood::all() {
        let _ = mood_color(*mood);
        assert!(!get_mood_particles(*mood, 0).is_empty());
    }
    for species in PetSpecies::all() {
        let preview = render_species_preview(*species);
        assert_eq!(preview.len(), CANVAS_H / 2);
        let colored_preview = render_colored_preview(*species, EvolutionStage::CyberElder, 0);
        assert!(!colored_preview.is_empty());
    }
    let _ = sky_scene(500);
}

#[test]
fn test_coat_palettes_and_accessory_overlays() {
    let mut pet = Pet::new("Mochi");
    pet.stage = EvolutionStage::Adult;
    pet.mood = PetMood::Excited;
    let base_grid = build_pet_grid(&pet, 1, 0, PetGaze::TowardKeeper);
    for &coat in CoatPreset::all() {
        pet.coat = coat;
        let _ = species_palette_with_coat(pet.species, coat);
        assert!(pixel_color_with_coat('B', pet.species, coat).is_some());
    }
    for &acc in Accessory::all() {
        if acc != Accessory::None {
            pet.accessory = acc;
            let acc_grid = build_pet_grid(&pet, 1, 0, PetGaze::TowardKeeper);
            assert_ne!(base_grid, acc_grid, "accessory {acc:?} should modify grid");
        }
    }
}
