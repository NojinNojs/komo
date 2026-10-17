use super::*;
use chrono::Utc;

impl Pet {
    pub fn play(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::Play,
                format!("💬 \"*curls tighter* Let me sleep a bit longer, {caretaker}...\""),
                "💭 (Feeling: Deeply asleep on the mat.)".to_string(),
                Some(PetMood::Sleeping),
            );
            return self.refuse(format!("💤 {} is sleeping! Wake them first.", self.name));
        }
        if self.is_sick || self.has_toothache {
            self.set_pet_reaction(
                PetAction::Play,
                format!(
                    "💬 \"Shivers weakly. 'I feel too sick to run around, {caretaker}. Need medicine [H] first.'\""
                ),
                "💭 (Feeling: Limbs ache and head feels dizzy.)".to_string(),
                Some(PetMood::Sick),
            );
            return self.refuse(format!(
                "🤒 {} feels sick and needs Medicine [H] before playing!",
                self.name
            ));
        }
        if self.energy < 20 {
            self.set_pet_reaction(
                PetAction::Play,
                format!(
                    "💬 \"Pants softly. 'My legs are worn out, {caretaker}. Let me rest [S] first.'\""
                ),
                "💭 (Feeling: Muscles heavy and low on stamina.)".to_string(),
                Some(PetMood::Sleepy),
            );
            return self.refuse(format!(
                "⚡ {} is too tired to play! Let them rest [S].",
                self.name
            ));
        }
        if self.hunger < 20 {
            self.set_pet_reaction(
                PetAction::Play,
                format!(
                    "💬 \"Stomach growls. 'Too hungry to chase anything right now, {caretaker}. Feed me [F] first?'\""
                ),
                "💭 (Feeling: Empty stomach makes it hard to run.)".to_string(),
                Some(PetMood::Lonely),
            );
            return self.refuse(format!(
                "🍙 {} is too hungry to play! Feed them [F] first.",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.play_cooldown_remaining(now);
        if remaining > 0 && self.happiness >= 75 {
            self.set_pet_reaction(
                PetAction::Play,
                format!(
                    "💬 \"Catching my breath for {}, {caretaker}. Or open the Arcade [G] if you want a match!\"",
                    format_duration_human(remaining)
                ),
                "💭 (Feeling: Heart beating fast after a good sprint.)".to_string(),
                Some(PetMood::Playful),
            );
            return self.refuse(format!(
                "⏳ {} just finished playing! Let them catch their breath ({}).",
                self.name,
                format_duration_human(remaining)
            ));
        }

        self.happiness = (self.happiness + 18).min(100);
        self.energy = self.energy.saturating_sub(14);
        self.hunger = self.hunger.saturating_sub(9);
        self.cleanliness = self.cleanliness.saturating_sub(5);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(5);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(2).max(base_w);
        self.last_played_at = Some(now);
        self.last_updated = now;
        self.add_xp(15);
        self.set_pet_reaction(
            PetAction::Play,
            format!(
                "💬 \"Good match, {caretaker}! Trimmed down to {}g and found +5 coins.\"",
                self.weight_grams
            ),
            format!("💭 (Feeling: Warm and energized after playing with {caretaker}.)"),
            Some(PetMood::Playful),
        );
        self.refresh_attention();

        let msg = format!(
            "🎾 Played Left/Right Game with {}! (+18 Happy, Weight {}g, +5 coins, +15 XP)",
            self.name, self.weight_grams
        );
        self.log_action(msg.clone());
        msg
    }
}
