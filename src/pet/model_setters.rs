use super::*;

impl Pet {
    /// Record a direct first-person reaction from your companion so the LCD
    /// immediately shows how they feel about what you just did!
    pub fn set_pet_reaction(
        &mut self,
        action: PetAction,
        speech: impl Into<String>,
        thought: impl Into<String>,
        mood: Option<PetMood>,
    ) {
        if self.last_action_kind == Some(action) {
            self.spam_streak = self.spam_streak.saturating_add(1);
        } else {
            self.last_action_kind = Some(action);
            self.spam_streak = 1;
        }
        self.last_pet_speech = Some(speech.into());
        self.last_pet_thought = Some(thought.into());
        self.reaction_ticks_left = 4; // Stays visible across ~4 decay ticks (~24s) or until next action
        if let Some(m) = mood {
            self.mood = m;
        }
    }

    pub fn hungry_hearts(&self) -> u8 {
        match self.hunger {
            0..=14 => 0,
            15..=39 => 1,
            40..=64 => 2,
            65..=84 => 3,
            _ => 4,
        }
    }

    pub fn happy_hearts(&self) -> u8 {
        match self.happiness {
            0..=14 => 0,
            15..=39 => 1,
            40..=64 => 2,
            65..=84 => 3,
            _ => 4,
        }
    }

    pub fn discipline_blocks(&self) -> u8 {
        (self.discipline / 25).min(4)
    }

    pub fn is_overweight(&self) -> bool {
        self.weight_grams >= self.stage.base_weight_grams() + 500
    }

    pub fn format_weight(&self) -> String {
        if self.weight_grams >= 1_000 {
            format!("{:.2}kg", f64::from(self.weight_grams) / 1000.0)
        } else {
            format!("{}g", self.weight_grams)
        }
    }

    pub fn care_rank(&self) -> CareRank {
        if self.care_mistakes == 0 && self.discipline >= 75 {
            CareRank::PerfectKeeper
        } else if self.care_mistakes <= 2 && self.discipline >= 50 {
            CareRank::StarGuardian
        } else if self.care_mistakes <= 5 {
            CareRank::CozyFriend
        } else {
            CareRank::MischievousRascal
        }
    }

    pub fn caretaker_label(&self) -> &str {
        let trimmed = self.caretaker_title.trim();
        if trimmed.is_empty() {
            "Keeper"
        } else {
            trimmed
        }
    }

    pub fn set_caretaker_title(&mut self, title: &str) {
        let trimmed = title.trim();
        if !trimmed.is_empty() {
            self.caretaker_title = trimmed.to_string();
            self.last_pet_speech = None;
            self.last_pet_thought = None;
            self.reaction_ticks_left = 0;
            self.log_action(format!(
                "🔖 {} will now call you '{}'.",
                self.name, self.caretaker_title
            ));
        }
    }
}
