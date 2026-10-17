use super::*;

/// Build a grounded sensory snapshot (`Sight`, `Hearing`, `Feeling`) based on habitat state.
pub fn sense_environment(
    pet: &Pet,
    tick: usize,
    selected_icon: LcdIcon,
    recent_key_ticks: u8,
    key_burst: u8,
) -> SensorySnapshot {
    let (_, gaze) = compute_posture_and_gaze(pet, tick, recent_key_ticks);
    let caretaker = pet.caretaker_label();

    if !pet.is_alive {
        return SensorySnapshot {
            gaze: PetGaze::Center,
            sight: "A still memorial stone rests in the quiet room.".to_string(),
            hearing: "Only the wind outside the sanctuary window.".to_string(),
            feeling: format!(
                "Awaiting burial rites [B] or a sacred Phoenix Tear [H] from {caretaker}."
            ),
        };
    }

    if pet.stage == EvolutionStage::Egg {
        return SensorySnapshot {
            gaze: PetGaze::Center,
            sight: "Soft amber light filtering through the shell.".to_string(),
            hearing: if recent_key_ticks > 0 {
                format!("Hears {caretaker}'s hands tapping near the nest.")
            } else {
                "Listening to the quiet hum of the incubator.".to_string()
            },
            feeling: if pet.incubation >= 80 {
                "Shell feels tight and warm; ready to crack open soon.".to_string()
            } else {
                format!(
                    "Warmth at {}%, resting safely on the cushion.",
                    pet.incubation
                )
            },
        };
    }

    let sight = if pet.is_sleeping {
        if pet.light_on {
            "Eyelids twitching against the bright room lamp.".to_string()
        } else {
            "Eyes closed in the cool, dark room.".to_string()
        }
    } else if pet.mood == PetMood::Angry {
        format!("Looking away from {caretaker} with folded ears.")
    } else if recent_key_ticks > 0 {
        match selected_icon {
            LcdIcon::Food => format!("Watching {caretaker} reach toward the pantry bowl."),
            LcdIcon::Lights => "Glancing up at the hanging room lamp.".to_string(),
            LcdIcon::Play => "Eyes wide, tracking the arcade cabinet.".to_string(),
            LcdIcon::Medicine => "Eyeing the herbal medicine bottle warily.".to_string(),
            LcdIcon::Bath => "Looking at the warm washbasin and brush.".to_string(),
            LcdIcon::Meter => format!("Looking straight at {caretaker} through the glass."),
            LcdIcon::Discipline => "Ears pinned back, reading your expression.".to_string(),
            LcdIcon::Talk => format!("Locking eyes with {caretaker}, waiting to talk."),
            LcdIcon::Attention => format!("Trying to catch {caretaker}'s eye."),
        }
    } else {
        match gaze {
            PetGaze::Center => format!("Looking right at {caretaker} with steady eyes."),
            PetGaze::Left => "Watching dust motes drift past the window.".to_string(),
            PetGaze::Right => "Inspecting the mossy stones on the right side.".to_string(),
        }
    };

    let hearing = if key_burst >= 6 {
        "Ears flattend: startled by rapid, heavy key clattering.".to_string()
    } else if recent_key_ticks > 0 {
        format!("One ear perked toward {caretaker}'s steady keystrokes.")
    } else if pet.is_sleeping {
        "Slow, even breathing in the quiet room.".to_string()
    } else {
        match (tick / 90) % 4 {
            0 => "Listening to distant birdsong outside the window.".to_string(),
            1 => "Hears dry leaves rustling in the breeze.".to_string(),
            2 => "Listening to rain tapping against the glass.".to_string(),
            _ => "Hears crickets and the soft hum of the night lamp.".to_string(),
        }
    };

    let feeling = if pet.has_toothache {
        "Sharp, pulsing ache in a back molar from too much sugar.".to_string()
    } else if pet.is_sick {
        "Shivering with a low fever; limbs feel heavy and cold.".to_string()
    } else if pet.hunger == 0 {
        "Stomach cramping hard from emptiness; dizzy and weak.".to_string()
    } else if pet.hunger <= 25 {
        "Belly rumbling; mouth watering for a warm meal.".to_string()
    } else if pet.mood == PetMood::Overstuffed || pet.hunger >= 92 {
        "Belly round, warm, and completely full.".to_string()
    } else if pet.poop_count > 0 || pet.cleanliness < 45 {
        "Coat feels sticky and damp; wants a warm wash.".to_string()
    } else if pet.is_sleeping {
        if pet.light_on {
            "Restless sleep because the room light is still on.".to_string()
        } else {
            "Curled up warm and safe on the woven mat.".to_string()
        }
    } else if pet.energy <= 25 {
        "Paws heavy and eyelids drooping; ready for bed.".to_string()
    } else if pet.bond >= 70 {
        format!("Feels safe, warm, and deeply attached to {caretaker}.")
    } else {
        "Pulse steady, paws warm on the sanctuary floor.".to_string()
    };

    SensorySnapshot {
        gaze,
        sight,
        hearing,
        feeling,
    }
}
