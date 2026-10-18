use super::*;

/// Compute the companion's current gaze (`Left`, `Center` at the player, or `Right`)
/// and horizontal stroll offset (`-2..=2`).
pub fn compute_posture_and_gaze(pet: &Pet, tick: usize, recent_key_ticks: u8) -> (isize, PetGaze) {
    // Arriving home triggers a "welcome host" stance: run to the front of the glass
    // and look straight at the keeper until they answer the greeting.
    if pet.pending_hosting.is_some()
        && pet.is_alive
        && pet.stage != EvolutionStage::Egg
        && pet.mood != PetMood::Angry
    {
        return (0, PetGaze::TowardKeeper);
    }

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
