use super::*;
use chrono::{DateTime, Utc};

impl Pet {
    pub fn rename(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        if !trimmed.is_empty() {
            self.name = trimmed.to_string();
            self.log_action(format!("🔖 Renamed to {}!", self.name));
        }
    }

    pub fn log_action(&mut self, msg: impl Into<String>) {
        self.action_log.push(msg.into());
        while self.action_log.len() > 25 {
            self.action_log.remove(0);
        }
    }

    pub fn clear_cooldowns(&mut self) {
        self.last_warmed_at = None;
        self.last_fed_at = None;
        self.last_played_at = None;
        self.last_trained_at = None;
        self.last_petted_at = None;
    }

    pub(super) fn remaining_secs(
        last: Option<DateTime<Utc>>,
        window: i64,
        now: DateTime<Utc>,
    ) -> u64 {
        match last {
            Some(t) => {
                let elapsed = now.signed_duration_since(t).num_seconds();
                if elapsed < window {
                    (window - elapsed) as u64
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    pub fn warm_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_warmed_at, WARM_COOLDOWN_SECS, now)
    }

    pub fn feed_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_fed_at, FEED_COOLDOWN_SECS, now)
    }

    pub fn play_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_played_at, PLAY_COOLDOWN_SECS, now)
    }

    pub fn train_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_trained_at, TRAIN_COOLDOWN_SECS, now)
    }

    pub fn pet_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_petted_at, PET_COOLDOWN_SECS, now)
    }

    pub fn refresh_attention(&mut self) {
        if self.stage == EvolutionStage::Egg {
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
            return;
        }

        if !self.is_alive {
            self.attention_call = AttentionCall::NeedsCare;
            return;
        }

        let urgent_need = self.is_sick
            || self.has_toothache
            || self.hungry_hearts() == 0
            || self.happy_hearts() == 0
            || self.poop_count >= 2
            || (self.is_sleeping && self.light_on);

        if urgent_need {
            if self.attention_call != AttentionCall::NeedsCare {
                self.attention_call = AttentionCall::NeedsCare;
                self.attention_since = Some(Utc::now());
            }
        } else if self.attention_call == AttentionCall::NeedsCare {
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
        }
    }

    pub fn refresh_mood(&mut self) {
        if !self.is_alive || self.health == 0 {
            self.is_alive = false;
            self.health = 0;
            self.mood = PetMood::PassedOut;
            self.refresh_attention();
            return;
        }

        let _ = self.check_evolution();
        if self.weight_grams < self.stage.base_weight_grams() {
            self.weight_grams = self.stage.base_weight_grams();
        }

        // Keep temporary direct action reactions visible while their timer is active!
        if self.reaction_ticks_left > 0
            && matches!(
                self.mood,
                PetMood::Overstuffed
                    | PetMood::Angry
                    | PetMood::Lonely
                    | PetMood::Sleepy
                    | PetMood::Loved
                    | PetMood::Excited
                    | PetMood::Playful
                    | PetMood::Proud
                    | PetMood::Eating
                    | PetMood::Training
            )
            && !self.is_sick
            && !self.has_toothache
        {
            self.refresh_attention();
            return;
        }

        if self.is_sick || self.has_toothache || self.health <= 35 {
            if self.health <= 35 {
                self.is_sick = true;
            }
            self.mood = PetMood::Sick;
        } else if self.is_sleeping {
            self.mood = PetMood::Sleeping;
        } else if self.hunger <= 15 || self.happiness <= 20 {
            self.mood = PetMood::Lonely;
        } else if self.energy <= 25 {
            self.mood = PetMood::Sleepy;
        } else if self.happiness >= 92 && self.hunger >= 75 {
            self.mood = PetMood::Excited;
        } else if self.happiness >= 75 && self.hunger >= 50 {
            self.mood = PetMood::Happy;
        } else {
            self.mood = PetMood::Idle;
        }
        self.refresh_attention();
    }

    pub fn update_mood(&mut self) {
        self.refresh_mood();
    }
}
