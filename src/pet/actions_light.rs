use super::*;
use chrono::Utc;

impl Pet {
    pub fn toggle_light(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        self.light_on = !self.light_on;
        self.last_updated = Utc::now();
        let caretaker = self.caretaker_label().to_string();

        let msg = if !self.light_on {
            if self.is_sleeping {
                self.bond = (self.bond + 3).min(100);
                self.add_xp(5);
                self.set_pet_reaction(
                    PetAction::ToggleLight,
                    format!("💬 \"*exhales softly* Thanks for dimming the lamp, {caretaker}...\""),
                    "💭 (Feeling: Deep, uninterrupted sleep in the dark.)".to_string(),
                    Some(PetMood::Sleeping),
                );
                format!(
                    "🌙 Switched room light OFF. {} sleeps soundly in the dark! (+3 Bond, +5 XP)",
                    self.name
                )
            } else {
                self.set_pet_reaction(
                    PetAction::ToggleLight,
                    format!("💬 \"It went dark all of a sudden, {caretaker}. Are we turning in early [K]?\""),
                    "💭 (Sight: Pupils dilating in the dark room.)".to_string(),
                    Some(PetMood::Lonely),
                );
                format!(
                    "🌙 Switched room light OFF while {} is awake (dim sanctuary mode).",
                    self.name
                )
            }
        } else if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::ToggleLight,
                format!("💬 \"*shields eyes* Too bright, {caretaker}... I'm still trying to sleep [K].\""),
                "💭 (Sight: Harsh lamp glare pressing through closed eyelids.)".to_string(),
                Some(PetMood::Angry),
            );
            "💡 Switched room light ON while sleeping! (Turn it OFF with [K] so they can rest!)"
                .to_string()
        } else {
            self.set_pet_reaction(
                PetAction::ToggleLight,
                format!("💬 \"Warm lamp light again. Good to see you clearly, {caretaker}.\""),
                "💭 (Sight: Room is warmly lit and clear.)".to_string(),
                Some(PetMood::Happy),
            );
            "💡 Switched room light ON! Warm light fills the habitat.".to_string()
        };
        self.refresh_mood();
        self.log_action(msg.clone());
        msg
    }
}
