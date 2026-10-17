//! Sprite submodule.
use super::*;

/// What your companion is saying out loud right now (`💬`).
/// Grounded, behavioral English that uses your custom `caretaker_title` (`pet.caretaker_label()`).
pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if let Some(ref reaction) = pet.last_pet_speech {
        return reaction.clone();
    }
    let caretaker = pet.caretaker_label();
    if !pet.is_alive || pet.mood == PetMood::PassedOut {
        if pet.buried {
            return format!(
                "💬 \"[ Epitaph: Here rests {}, remembered by {}. ]\"",
                pet.name, caretaker
            );
        }
        return format!(
            "💬 \"{} lies still. Hold a Vigil [B] or spend 50 coins on a Phoenix Tear [H].\"",
            pet.name
        );
    }
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 80 {
            format!(
                "💬 \"*tap... tap...* Hairline cracks spread across the warm shell, {caretaker}.\""
            )
        } else {
            "💬 \"A steady pulse beats beneath the shell.\"".to_string()
        };
    }
    if pet.has_toothache {
        return format!("💬 \"My back tooth is throbbing, {caretaker}. Do we have medicine [H]?\"");
    }
    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return if pet.light_on {
            format!(
                "💬 \"*squints* The lamp is still bright, {caretaker}... could you switch it off [K]?\""
            )
        } else {
            format!(
                "💬 \"*soft breathing* ...warm mat... save the last {} for {}...\"",
                pet.species.favorite_snack(),
                caretaker
            )
        };
    }
    if pet.is_sick || pet.mood == PetMood::Sick {
        return format!(
            "💬 \"I'm shivering, {caretaker}... my paws feel heavy. Need herbal medicine [H].\""
        );
    }
    if pet.hunger == 0 {
        return format!(
            "💬 \"My stomach is cramping hard, {caretaker}. Please feed me [F] before I faint.\""
        );
    }
    if pet.poop_count > 0 {
        return format!("💬 \"The floor is getting messy, {caretaker}. Could we wash up [C]?\"");
    }
    if pet.hunger <= 30 {
        return format!(
            "💬 \"My belly just rumbled, {caretaker}. Do we have any {} [F]?\"",
            pet.species.favorite_snack()
        );
    }
    if pet.energy <= 25 {
        return format!(
            "💬 \"*blinks slowly* My eyelids are heavy, {caretaker}. Time for sleep [S]?\""
        );
    }

    let chatter = [
        format!(
            "💬 \"You're back at the desk, {caretaker}. Let's talk [Y] whenever you have a minute.\""
        ),
        format!(
            "💬 \"I can smell {} from the pantry [M].\"",
            pet.species.favorite_snack()
        ),
        format!(
            "💬 \"Want to run a round in the Arcade [G], {caretaker}?\""
        ),
        format!(
            "💬 \"Looking right at you, {caretaker}. Everything feels calm in here.\""
        ),
    ];
    chatter[(tick / 48) % chatter.len()].clone()
}

/// Inner sensory & thought line (`💭`) showing Sight, Hearing, and Feeling in real time!
pub fn thought_bubble(pet: &Pet, tick: usize) -> String {
    if let Some(ref thought) = pet.last_pet_thought {
        return thought.clone();
    }
    let window_tick = (tick / 48) * 48;
    let snap = sense_environment(pet, window_tick, LcdIcon::Meter, 0, 0);
    match (tick / 48) % 3 {
        0 => format!("💭 👁 Sight ({}) • {}", snap.gaze.label(), snap.sight),
        1 => format!("💭 👂 Hearing • {}", snap.hearing),
        _ => format!("💭 🫳 Feeling • {}", snap.feeling),
    }
}

pub fn mood_dialogue(pet: &Pet) -> String {
    speech_bubble(pet, 0)
}

pub fn mood_color(mood: PetMood) -> Color {
    match mood {
        PetMood::Idle => Color::Cyan,
        PetMood::Eating => Color::LightGreen,
        PetMood::Sleeping | PetMood::Sleepy => Color::LightBlue,
        PetMood::Happy | PetMood::Playful | PetMood::Loved => Color::LightMagenta,
        PetMood::Excited | PetMood::Proud | PetMood::Training => Color::LightYellow,
        PetMood::Lonely | PetMood::Overstuffed => Color::Yellow,
        PetMood::Angry | PetMood::Sick => Color::LightRed,
        PetMood::PassedOut => Color::Gray,
    }
}

pub fn stage_color(stage: EvolutionStage) -> Color {
    match stage {
        EvolutionStage::Egg => Color::White,
        EvolutionStage::Baby => Color::LightGreen,
        EvolutionStage::Teen => Color::LightCyan,
        EvolutionStage::Adult => Color::LightMagenta,
        EvolutionStage::CyberElder => Color::LightYellow,
    }
}

pub fn theme_color(theme: ShellTheme) -> Color {
    match theme {
        ShellTheme::StrawberryPink => Color::Rgb(255, 130, 175),
        ShellTheme::MatchaGreen => Color::Rgb(130, 215, 155),
        ShellTheme::SkyBlue => Color::Rgb(115, 195, 255),
        ShellTheme::LavenderDream => Color::Rgb(190, 150, 255),
        ShellTheme::SunsetGold => Color::Rgb(255, 195, 95),
    }
}

pub fn sky_scene(tick: usize) -> (&'static str, &'static str) {
    match (tick / 90) % 4 {
        0 => ("🌞 DAY", "Sunny"),
        1 => ("🌸 BREEZE", "Petal Wind"),
        2 => ("🌧️ RAIN", "Cozy Drops"),
        _ => ("🌠 NIGHT", "Starfall"),
    }
}
