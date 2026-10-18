use super::*;
use chrono::Utc;

impl Pet {
    pub fn tick_decay(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();

        if self.reaction_ticks_left > 0 {
            self.reaction_ticks_left -= 1;
            if self.reaction_ticks_left == 0 {
                self.last_pet_speech = None;
                self.last_pet_thought = None;
            }
        }

        if !self.is_alive {
            self.mood = PetMood::PassedOut;
            return;
        }

        if self.stage == EvolutionStage::Egg {
            self.incubation = (self.incubation + 1).min(99);
            return;
        }

        if self.is_sleeping {
            if self.light_on {
                self.energy = (self.energy + 2).min(100);
                self.happiness = self.happiness.saturating_sub(1);
                if self.age_seconds.is_multiple_of(30) {
                    self.care_mistakes = self.care_mistakes.saturating_add(1);
                }
            } else {
                self.energy = (self.energy + 5).min(100);
            }
            self.hunger = self.hunger.saturating_sub(1);
        } else {
            let weight_penalty = u8::from(self.is_overweight());
            if self.age_seconds.is_multiple_of(2) {
                self.hunger = self.hunger.saturating_sub(1);
                self.happiness = self.happiness.saturating_sub(1);
                self.energy = self.energy.saturating_sub(1 + weight_penalty);
            } else {
                self.hunger = self.hunger.saturating_sub(1);
            }

            if self.attention_call == AttentionCall::None
                && self.discipline < 100
                && self.age_seconds.is_multiple_of(40)
            {
                if self.age_seconds.is_multiple_of(80) {
                    self.attention_call = AttentionCall::Misbehaving;
                    self.attention_since = Some(Utc::now());
                    self.log_action(format!(
                        "🔔 {} is beeping for attention just to be mischievous! Press [D] to Scold!",
                        self.name
                    ));
                } else {
                    self.attention_call = AttentionCall::Crying;
                    self.attention_since = Some(Utc::now());
                    self.log_action(format!(
                        "🔔 {} is sniffling and calling for you! Press [T] to Praise & Pat!",
                        self.name
                    ));
                }
            }
        }

        if self.poop_count > 0 {
            self.cleanliness = self.cleanliness.saturating_sub(self.poop_count * 2);
            if self.poop_count >= 3 {
                self.is_sick = true;
            }
        }

        // Realistic Starvation & Untreated Illness Drain!
        let mut health_drain = 0u8;
        if self.hunger == 0 {
            health_drain = health_drain.saturating_add(6);
            let base_w = self.stage.base_weight_grams();
            self.weight_grams = self.weight_grams.saturating_sub(1).max(base_w);
        } else if self.hunger <= 15 {
            health_drain = health_drain.saturating_add(2);
        }
        if self.cleanliness <= 20 || self.poop_count >= 3 {
            health_drain = health_drain.saturating_add(3);
        }
        if self.is_sick || self.has_toothache {
            health_drain = health_drain.saturating_add(4);
        }

        if health_drain > 0 {
            self.health = self.health.saturating_sub(health_drain);
            if self.age_seconds.is_multiple_of(5) {
                self.care_mistakes = self.care_mistakes.saturating_add(1);
            }
        }

        if self.health == 0 {
            self.is_alive = false;
            self.is_sleeping = false;
            self.mood = PetMood::PassedOut;
            let reason = if self.hunger == 0 {
                "Starvation (0% Hunger)".to_string()
            } else {
                "Untreated Sickness & Neglect".to_string()
            };
            self.cause_of_death = Some(reason.clone());
            self.log_action(format!(
                "👻 {} collapsed from {}! Press [H] for Phoenix Tear Revival!",
                self.name, reason
            ));
            self.refresh_attention();
            return;
        }

        if self.health <= 35 {
            self.is_sick = true;
        }

        if let Some(since) = self.attention_since {
            if Utc::now().signed_duration_since(since).num_minutes() >= 15 {
                self.care_mistakes = self.care_mistakes.saturating_add(1);
                self.attention_since = Some(Utc::now());
            }
        }

        self.check_evolution();
        self.refresh_mood();
    }

    pub fn tick_standby(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();
        if !self.is_alive {
            return;
        }
        if self.stage == EvolutionStage::Egg {
            self.incubation = (self.incubation + 1).min(99);
            return;
        }
        if self.age_seconds.is_multiple_of(2) {
            if self.is_sleeping {
                self.energy = (self.energy + 4).min(100);
            } else {
                self.hunger = self.hunger.saturating_sub(1).max(20);
                self.happiness = self.happiness.saturating_sub(1).max(25);
                self.energy = self.energy.saturating_sub(1).max(25);
            }
        }
        self.check_evolution();
        self.refresh_mood();
    }
}
