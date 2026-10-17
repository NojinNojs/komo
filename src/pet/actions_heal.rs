use super::*;
use chrono::Utc;

impl Pet {
    pub fn heal(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();

        // Sacred Phoenix Tear Revival if the companion has died — strictly ONCE per lifetime!
        if !self.is_alive {
            if self.buried {
                return self.refuse(format!(
                    "🪦 {} has already been laid to rest in the Memorial Hall and cannot be revived.",
                    self.name
                ));
            }
            if self.revival_count >= 1 {
                self.set_pet_reaction(
                    PetAction::Heal,
                    format!(
                        "💬 \"The Phoenix Tear has already been spent once in {}'s life, {caretaker}. Hold a Vigil [B] to lay them to rest.\"",
                        self.name
                    ),
                    "💭 (Permanent Consequence: A companion can only be revived once per lifetime.)"
                        .to_string(),
                    Some(PetMood::PassedOut),
                );
                return self.refuse(format!(
                    "🕯 {} already used their one-time Phoenix Revival! Death is now permanent — press [B] to hold the Burial Vigil ({}/3).",
                    self.name, self.vigil_steps_completed
                ));
            }

            self.is_alive = true;
            self.revival_count = self.revival_count.saturating_add(1);
            self.cause_of_death = None;
            self.is_sick = false;
            self.has_toothache = false;
            self.snacks_in_a_row = 0;
            self.poop_count = 0;
            self.health = 80;
            self.hunger = 60;
            self.happiness = 65;
            self.energy = 75;
            self.cleanliness = 100;
            self.coins = self.coins.saturating_sub(50);
            self.bond = self.bond.saturating_sub(5).max(5);
            self.care_mistakes = self.care_mistakes.saturating_add(3);
            self.last_updated = Utc::now();
            self.set_pet_reaction(
                PetAction::Heal,
                format!(
                    "💬 \"Draws a shaky breath as the Phoenix Tear takes hold. 'Don't leave me to fade again, {caretaker}...'\""
                ),
                "💭 (Reborn Once: Survived on the single Phoenix Tear; a second death will be permanent.)"
                    .to_string(),
                Some(PetMood::Loved),
            );
            self.refresh_mood();
            let msg = format!(
                "🔥 PHOENIX REVIVAL! {} returned to life with 80% Health (+3 Care Mistakes, 0 revivals left)!",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }

        if !self.is_sick && !self.has_toothache && self.health >= 95 {
            self.set_pet_reaction(
                PetAction::Heal,
                format!(
                    "💬 \"Makes a wry face at the bitter bottle. 'I'm already healthy (100% 💚), {caretaker}.'\""
                ),
                "💭 (Feeling: Already healthy; no need for bitter medicine.)".to_string(),
                Some(PetMood::Playful),
            );
            return self.refuse(format!("💚 {} is already in tip-top health!", self.name));
        }

        let was_toothache = self.has_toothache;
        self.is_sick = false;
        self.has_toothache = false;
        self.snacks_in_a_row = 0;
        self.health = 100;
        self.happiness = (self.happiness + 10).min(100);
        self.bond = (self.bond + 3).min(100);
        self.last_updated = Utc::now();
        self.add_xp(10);
        self.set_pet_reaction(
            PetAction::Heal,
            if was_toothache {
                format!(
                    "💬 \"Sighs in relief as the clove remedy numbs the ache. 'My tooth finally stopped throbbing, {caretaker}.'\""
                )
            } else {
                format!(
                    "💬 \"Breathing steadies as the fever breaks. 'Thank you for the medicine, {caretaker}.'\""
                )
            },
            "💭 (Feeling: Fever and pain subsiding quickly.)".to_string(),
            Some(PetMood::Happy),
        );
        self.refresh_mood();

        let msg = if was_toothache {
            format!(
                "💉 Treated {}'s Toothache and restored full health! (+10 XP)",
                self.name
            )
        } else {
            format!(
                "💉 Gave {} herbal medicine. Sickness cured! (+10 XP)",
                self.name
            )
        };
        self.log_action(msg.clone());
        msg
    }

    pub fn apply_minigame_reward(
        &mut self,
        score: u32,
        coins_earned: u32,
        xp_earned: u32,
    ) -> String {
        let caretaker = self.caretaker_label().to_string();
        self.coins = self.coins.saturating_add(coins_earned);
        self.happiness = (self.happiness + 12).min(100);
        self.bond = (self.bond + 3).min(100);
        self.energy = self.energy.saturating_sub(5);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(1).max(base_w);
        self.add_xp(xp_earned);
        self.set_pet_reaction(
            PetAction::Play,
            format!(
                "💬 \"Great run in the Arcade, {caretaker}! Scored {score} and brought back +{coins_earned} coins.\""
            ),
            "💭 (Feeling: Proud of the high score and coin haul.)".to_string(),
            Some(PetMood::Excited),
        );
        self.refresh_mood();

        let msg = format!(
            "🎮 Arcade scored {}! Earned +{} coins, +{} XP & burned -1g weight!",
            score, coins_earned, xp_earned
        );
        self.log_action(msg.clone());
        msg
    }
}
