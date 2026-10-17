use super::*;
use chrono::Utc;

impl Pet {
    pub fn train(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.is_sleeping {
            return self.refuse(format!(
                "💤 {} is asleep and cannot train right now.",
                self.name
            ));
        }
        if self.is_sick || self.has_toothache {
            self.set_pet_reaction(
                PetAction::Train,
                format!(
                    "💬 \"Coughs softly. 'I'm not well enough for drills today, {caretaker}. Need medicine [H].'\""
                ),
                "💭 (Feeling: Too weak and feverish for training.)".to_string(),
                Some(PetMood::Sick),
            );
            return self.refuse(format!(
                "🤒 {} is unwell! Heal them [H] before training.",
                self.name
            ));
        }
        if self.energy < 25 {
            self.set_pet_reaction(
                PetAction::Train,
                format!(
                    "💬 \"Legs are shaking, {caretaker}... let me sleep [S] before another drill.\""
                ),
                "💭 (Feeling: Stamina is nearly depleted.)".to_string(),
                Some(PetMood::Sleepy),
            );
            return self.refuse(format!(
                "⚡ {} lacks energy to train! Need 25+ Energy.",
                self.name
            ));
        }
        if self.hunger < 20 {
            self.set_pet_reaction(
                PetAction::Train,
                format!(
                    "💬 \"Can't focus on an empty stomach, {caretaker}. Let's eat [F] first.\""
                ),
                "💭 (Feeling: Stomach is too empty for heavy drills.)".to_string(),
                Some(PetMood::Lonely),
            );
            return self.refuse(format!(
                "🍱 {} is too hungry to focus on training!",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.train_cooldown_remaining(now);
        if remaining > 0 {
            self.set_pet_reaction(
                PetAction::Train,
                format!(
                    "💬 \"Muscles are still warm from the last set, {caretaker}. Rest {} before the next drill.\"",
                    format_duration_human(remaining)
                ),
                "💭 (Feeling: Pacing recovery between training sets.)".to_string(),
                Some(PetMood::Proud),
            );
            return self.refuse(format!(
                "⏳ {} is catching their breath after training! Wait {}.",
                self.name,
                format_duration_human(remaining)
            ));
        }

        self.last_trained_at = Some(now);
        self.energy = self.energy.saturating_sub(20);
        self.hunger = self.hunger.saturating_sub(12);
        self.happiness = (self.happiness + 8).min(100);
        self.discipline = (self.discipline + 8).min(100);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(10);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(2).max(base_w);
        self.last_updated = now;
        self.add_xp(22);
        self.set_pet_reaction(
            PetAction::Train,
            format!(
                "💬 \"Stands tall after the drill. 'Getting stronger every day, {caretaker} (+22 XP, +10 💰).'\""
            ),
            "💭 (Feeling: Muscles warm, posture sharp and disciplined.)".to_string(),
            Some(PetMood::Training),
        );

        let msg = format!(
            "🥋 {} finished a focus training session! (Weight {}g, +8% Discipline, +22 XP, +10 coins)",
            self.name, self.weight_grams
        );
        self.log_action(msg.clone());
        msg
    }
}
