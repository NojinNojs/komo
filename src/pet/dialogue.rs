use super::*;

impl Pet {
    pub fn dialogue_prompt(&self, tick: usize) -> DialoguePrompt {
        let caretaker = self.caretaker_label();
        if !self.is_alive {
            return DialoguePrompt {
                inner_thought: "💭 (A quiet stillness fills the habitat around the resting spirit.)"
                    .to_string(),
                pet_line: format!(
                    "💬 \"{}, {} has fallen. Will you hold a Vigil [B] or use a Phoenix Tear [H]?\"",
                    caretaker, self.name
                ),
                option_1: "1. Hold a quiet Vigil step and vow better care (+Bond)".to_string(),
                option_2: "2. Use a Sacred Phoenix Tear to revive them now".to_string(),
            };
        }

        if self.stage == EvolutionStage::Egg {
            return DialoguePrompt {
                inner_thought:
                    "💭 (Inside the shell, a small shadow shifts toward the sound of your voice.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"*tap... tap...* {}'s egg rocks gently against the cushion when {} speaks.\"",
                    self.name, caretaker
                ),
                option_1: "1. Hum a steady, low melody beside the nest (+Bond)".to_string(),
                option_2: "2. Tuck the warm blanket closer around the shell (+Warmth)".to_string(),
            };
        }

        if self.is_sleeping {
            return DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} twitches a paw in sleep, dreaming of {}.)",
                    self.name,
                    self.species.favorite_snack()
                ),
                pet_line: format!(
                    "💬 \"*murmurs in sleep* ...save a piece of {} for {}...\"",
                    self.species.favorite_snack(),
                    caretaker
                ),
                option_1: "1. Pull the blanket up to their shoulders quietly (+Bond)".to_string(),
                option_2: "2. Leave a small treat beside the mat for morning (+Happy)".to_string(),
            };
        }

        let topic = (tick + self.xp as usize) % 3;
        match (self.species, topic) {
            (PetSpecies::IgnisDragon, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (Exhales a thin ring of warm smoke and looks straight at you.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"{}, the scales along my wings feel heavier today. What should we practice?\"",
                    caretaker
                ),
                option_1: "1. Practice wind-draft glides across the yard (+12 XP, +5 Bond)".to_string(),
                option_2: "2. Toast sweet chestnuts over a low ember (+14 Happy, +5 coins)".to_string(),
            },
            (PetSpecies::AzureWyrm, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (Rolls the pale sea-pearl between front claws, listening to it hum.)".to_string(),
                pet_line: format!(
                    "💬 \"Listen closely, {}. You can hear the tide inside this pearl.\"",
                    caretaker
                ),
                option_1: "1. Sit together and listen to the low tide (+12 XP, +5 Bond)".to_string(),
                option_2: "2. Polish the sea-pearl with a soft cloth (+14 Happy, +5 coins)".to_string(),
            },
            (PetSpecies::CrystalUni, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (Tilts its horn so a thin band of prism light falls across your keyboard.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"The air feels clear tonight, {}. Want to step outside for a bit?\"",
                    caretaker
                ),
                option_1: "1. Pace the garden path under the stars (+12 XP, +5 Bond)".to_string(),
                option_2: "2. Brush the tangles out of its mane (+14 Happy, +5 coins)"
                    .to_string(),
            },
            (_, 1) => DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} rests its chin near the glass, watching your hands on the keys.)",
                    self.name
                ),
                pet_line: format!(
                    "💬 \"You've been at the terminal a while, {}. How is the work going?\"",
                    caretaker
                ),
                option_1: "1. 'Making steady progress—let's keep going.' (+12 XP, +5 Bond)"
                    .to_string(),
                option_2: "2. 'Taking a five-minute breather with you.' (+14 Happy, +5 Bond)"
                    .to_string(),
            },
            _ => DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} nudges a shiny pebble across the floor toward you.)",
                    self.name
                ),
                pet_line: format!(
                    "💬 \"Found this by the garden border, {}. Keep it for the coin pouch?\"",
                    caretaker
                ),
                option_1: "1. Practice a quick recall drill together (+12 XP, +5 coins)".to_string(),
                option_2: "2. Scratch behind their ears and share a rest (+14 Happy, +5 Bond)".to_string(),
            },
        }
    }
}
