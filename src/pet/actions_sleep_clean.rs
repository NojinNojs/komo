use super::*;
use chrono::Utc;

impl Pet {
    pub fn toggle_sleep(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        self.is_sleeping = !self.is_sleeping;
        self.last_updated = Utc::now();
        let caretaker = self.caretaker_label().to_string();

        if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::ToggleSleep,
                format!(
                    "💬 \"Curls onto the woven mat. 'Goodnight, {caretaker}... please switch off the lamp [K].'\""
                ),
                "💭 (Feeling: Tucking paws in and drifting to sleep.)".to_string(),
                Some(PetMood::Sleeping),
            );
            self.refresh_attention();
            let light_hint = if self.light_on {
                " (Don't forget to turn the Light OFF with [K]!)"
            } else {
                ""
            };
            let msg = format!(
                "🌙 Tucked {} onto the sleeping mat. Goodnight!{}",
                self.name, light_hint
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.light_on = true;
            self.energy = (self.energy + 15).min(100);
            self.set_pet_reaction(
                PetAction::ToggleSleep,
                format!(
                    "💬 \"Stretches all four limbs and yawns. 'Morning, {caretaker}. Ready for the day.'\""
                ),
                "💭 (Feeling: Rested and clear-headed after sleep.)".to_string(),
                Some(PetMood::Happy),
            );
            self.refresh_mood();
            let msg = format!(
                "🌞 {} woke up refreshed and turned the room light ON! (+15 Energy)",
                self.name
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn clean(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.cleanliness >= 100 && self.poop_count == 0 {
            self.set_pet_reaction(
                PetAction::Clean,
                format!(
                    "💬 \"Looks around the spotless floor. 'Everything is already clean, {caretaker}.'\""
                ),
                "💭 (Sight: Floor and coat are already clean and dry.)".to_string(),
                Some(PetMood::Proud),
            );
            return self.refuse("🌟 The habitat floor is already sparkling clean!".to_string());
        }

        self.poop_count = 0;
        self.meals_since_poop = 0;
        self.snacks_in_a_row = 0;
        self.cleanliness = 100;
        self.happiness = (self.happiness + 6).min(100);
        self.health = (self.health + 5).min(100);
        self.bond = (self.bond + 2).min(100);
        self.last_updated = Utc::now();
        self.add_xp(8);
        self.set_pet_reaction(
            PetAction::Clean,
            format!(
                "💬 \"Shakes off warm water droplets. 'That feels much better, {caretaker}. Thanks for the wash.'\""
            ),
            "💭 (Feeling: Coat is clean, dry, and fresh.)".to_string(),
            Some(PetMood::Happy),
        );
        self.refresh_mood();

        let msg = format!(
            "🚽 Flushed the mess, brushed teeth & gave {} a warm bath! (+8 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }
}
