use super::*;
use chrono::{DateTime, Utc};

impl Pet {
    pub fn apply_offline_decay_with_report(
        &mut self,
        now: DateTime<Utc>,
    ) -> (u64, Option<OfflineReport>) {
        let elapsed = now
            .signed_duration_since(self.last_updated)
            .num_seconds()
            .max(0) as u64;
        if elapsed == 0 {
            return (0, None);
        }

        self.last_updated = now;
        self.age_seconds = self.age_seconds.saturating_add(elapsed);

        let mut events = Vec::new();
        let mut coins_found = 0u32;

        if !self.is_alive {
            return (elapsed, None);
        }

        if self.stage == EvolutionStage::Egg {
            let before = self.incubation;
            let passive = ((elapsed / 15) as u8).min(99);
            self.incubation = self.incubation.saturating_add(passive).min(99);
            let gained = self.incubation.saturating_sub(before);
            if gained > 0 {
                events.push(format!(
                    "🥚 Your egg absorbed ambient sanctuary warmth (+{}% -> {}% incubation).",
                    gained, self.incubation
                ));
            }
            if self.incubation >= 99 {
                events.push(
                    "🐣 The shell is trembling! Press [W] once to hatch your companion!"
                        .to_string(),
                );
            }
            let report = if elapsed >= 120 {
                Some(OfflineReport {
                    elapsed_seconds: elapsed,
                    pet_name: self.name.clone(),
                    events,
                    coins_found: 0,
                })
            } else {
                None
            };
            return (elapsed, report);
        }

        let minutes = elapsed / 60;
        if minutes == 0 {
            return (elapsed, None);
        }

        let (hunger_drop, happy_drop, energy_delta, clean_drop) = if minutes <= 30 {
            let m = minutes as u8;
            if self.is_sleeping {
                (m, m / 2, m.saturating_mul(5), m / 2)
            } else {
                (m.saturating_mul(2), m, m, m)
            }
        } else {
            let ten_min_blocks = (minutes / 10).min(144) as u8;
            if self.is_sleeping {
                (
                    ten_min_blocks.min(70),
                    (ten_min_blocks / 2).min(50),
                    ten_min_blocks.saturating_mul(6).min(100),
                    (ten_min_blocks / 2).min(55),
                )
            } else {
                (
                    ten_min_blocks.saturating_add(ten_min_blocks / 5).min(95),
                    ten_min_blocks.min(75),
                    ten_min_blocks.min(80),
                    ten_min_blocks.min(75),
                )
            }
        };

        let old_hunger = self.hunger;
        let old_energy = self.energy;

        if self.is_sleeping {
            if self.light_on && minutes >= 30 {
                self.care_mistakes = self.care_mistakes.saturating_add(1);
                events.push(format!(
                    "💡 The room light was left ON while {} slept (+1 Care Mistake)!",
                    self.name
                ));
            }
            self.energy = self.energy.saturating_add(energy_delta).min(100);
            self.hunger = self.hunger.saturating_sub(hunger_drop);
            self.happiness = self.happiness.saturating_sub(happy_drop).max(15);
            self.cleanliness = self.cleanliness.saturating_sub(clean_drop).max(20);
            if self.energy >= 95 && minutes >= 45 {
                self.is_sleeping = false;
                self.light_on = true;
                events.push(format!(
                    "🌞 {} slept soundly while you were away and woke up at {}% Energy!",
                    self.name, self.energy
                ));
            } else {
                events.push(format!(
                    "💤 {} rested peacefully (Energy {}% -> {}%).",
                    self.name, old_energy, self.energy
                ));
            }
        } else {
            self.hunger = self.hunger.saturating_sub(hunger_drop);
            self.happiness = self.happiness.saturating_sub(happy_drop).max(10);
            self.energy = self.energy.saturating_sub(energy_delta).max(10);
            self.cleanliness = self.cleanliness.saturating_sub(clean_drop).max(15);

            events.push(format!(
                "🍙 Appetite grew over {} (Fullness {}% -> {}% / {}).",
                format_duration_human(elapsed),
                old_hunger,
                self.hunger,
                format_hearts(self.hungry_hearts())
            ));

            if self.energy <= 20 && minutes >= 120 {
                self.is_sleeping = true;
                self.light_on = false;
                self.energy = 55;
                events.push(format!(
                    "🌙 {} got sleepy waiting for you, turned off the light, and curled up for a nap.",
                    self.name
                ));
            }
        }

        let offline_poops = ((minutes / 180) as u8).min(3);
        if offline_poops > 0 {
            self.poop_count = (self.poop_count + offline_poops).min(5);
            events.push(format!(
                "🚽 {} left {} mess(es) on the floor — press [C] to Flush & Bath!",
                self.name, offline_poops
            ));
        }

        if !self.is_sick && old_hunger >= 50 && minutes >= 15 {
            coins_found = ((minutes / 15) as u32 * 3).clamp(3, 35);
            self.coins = self.coins.saturating_add(coins_found);
            self.add_xp(coins_found);
            events.push(format!(
                "🎁 While exploring the sanctuary garden, {} found +{} coins & +{} XP for you!",
                self.name, coins_found, coins_found
            ));
        }

        if self.hunger <= 15 || self.cleanliness <= 20 || self.poop_count >= 4 {
            let dmg = ((minutes / 30) as u8).clamp(5, 85);
            self.health = self.health.saturating_sub(dmg);
            self.care_mistakes = self.care_mistakes.saturating_add(1);
            if self.health == 0 {
                self.is_alive = false;
                self.mood = PetMood::PassedOut;
                self.cause_of_death = Some("Extended Offline Starvation & Illness".to_string());
                events.push(format!(
                    "👻 {} collapsed from prolonged starvation & illness! Press [H] for Phoenix Tear Revival!",
                    self.name
                ));
            } else if self.health <= 35 || self.poop_count >= 4 {
                self.is_sick = true;
                events.push(format!(
                    "🤒 {} caught a chill from low hunger/cleanliness and needs Medicine [H]!",
                    self.name
                ));
            }
        }

        self.check_evolution();
        self.refresh_mood();

        let report = if elapsed >= 120 {
            Some(OfflineReport {
                elapsed_seconds: elapsed,
                pet_name: self.name.clone(),
                events,
                coins_found,
            })
        } else {
            None
        };

        (elapsed, report)
    }
}
