//! Sensory perception, gaze direction, and living diorama environment for **KOMO**.
//!
//! Gives each companion four grounded senses (`Sight`, `Hearing`, `Feeling`, `Speech`),
//! three gaze directions (`Left`, `Center` facing the player, `Right`), and a living
//! window/floor habitat diorama that reacts to light, weather, and player keystrokes.

use crate::pet::{AttentionCall, EvolutionStage, LcdIcon, Pet, PetMood, PetSpecies};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Which way the companion is currently looking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PetGaze {
    Left,
    #[default]
    Center,
    Right,
}

impl PetGaze {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Left => "Looking Left ◀",
            Self::Center => "Looking at You ●",
            Self::Right => "Looking Right ▶",
        }
    }
}

/// Real-time snapshot of what the companion perceives right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensorySnapshot {
    pub gaze: PetGaze,
    pub sight: String,
    pub hearing: String,
    pub feeling: String,
}

/// Compute the companion's current gaze (`Left`, `Center` at the player, or `Right`)
/// and horizontal stroll offset (`-2..=2`).
pub fn compute_posture_and_gaze(pet: &Pet, tick: usize, recent_key_ticks: u8) -> (isize, PetGaze) {
    if !pet.is_alive
        || pet.stage == EvolutionStage::Egg
        || pet.is_sleeping
        || pet.is_sick
        || pet.has_toothache
    {
        return (0, PetGaze::Center);
    }

    // If the caretaker just pressed a key or the pet is speaking/calling, look straight at the player!
    if recent_key_ticks > 0
        || pet.reaction_ticks_left > 0
        || pet.attention_call != AttentionCall::None
        || pet.hunger <= 25
        || matches!(
            pet.mood,
            PetMood::Eating | PetMood::Loved | PetMood::Overstuffed
        )
    {
        // Unless they are grumpy from an unfair scold—then they turn away!
        if pet.mood == PetMood::Angry {
            return (1, PetGaze::Right);
        }
        return (0, PetGaze::Center);
    }

    // Autonomous habitat stroll cycle: walks left, pauses to look at you, walks right, looks at you.
    let phase = (tick / 8) % 16;
    let cycle: [(isize, PetGaze); 16] = [
        (0, PetGaze::Center),
        (0, PetGaze::Center),
        (-1, PetGaze::Left),
        (-2, PetGaze::Left),
        (-2, PetGaze::Center),
        (-1, PetGaze::Right),
        (0, PetGaze::Center),
        (0, PetGaze::Center),
        (1, PetGaze::Right),
        (2, PetGaze::Right),
        (2, PetGaze::Center),
        (1, PetGaze::Left),
        (0, PetGaze::Center),
        (0, PetGaze::Center),
        (0, PetGaze::Left),
        (0, PetGaze::Center),
    ];
    cycle[phase]
}

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
                format!("Warmth at {}%, resting safely on the cushion.", pet.incubation)
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
            PetGaze::Left => "Watching dust motes drift past the left window.".to_string(),
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
            Line::from(Span::styled(night, Style::default().fg(Color::LightMagenta)))
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
            Span::styled(
                format!("  {bowl}   •   "),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                format!("Gaze: {:<16}", compute_posture_and_gaze(pet, 0, 0).1.label()),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("   •   {bed}  "), Style::default().fg(Color::Gray)),
        ]),
    ]
}
