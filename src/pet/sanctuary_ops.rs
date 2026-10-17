use super::*;
use chrono::{DateTime, Utc};

impl Sanctuary {
    pub fn switch_by_query(&mut self, query: &str) -> Option<usize> {
        let trimmed = query.trim();
        if let Ok(one_based) = trimmed.parse::<usize>() {
            if one_based >= 1 && one_based <= self.pets.len() {
                self.active_index = one_based - 1;
                return Some(self.active_index);
            }
        }
        let lower = trimmed.to_lowercase();
        if let Some(pos) = self
            .pets
            .iter()
            .position(|p| p.name.to_lowercase() == lower)
        {
            self.active_index = pos;
            return Some(pos);
        }
        None
    }

    pub fn next_pet(&mut self) {
        if !self.pets.is_empty() {
            self.active_index = (self.active_index + 1) % self.pets.len();
        }
    }

    pub fn prev_pet(&mut self) {
        if !self.pets.is_empty() {
            self.active_index = (self.active_index + self.pets.len() - 1) % self.pets.len();
        }
    }

    pub fn sync_active(&mut self, pet: &Pet) {
        if self.pets.is_empty() {
            self.pets.push(pet.clone());
            self.active_index = 0;
        } else {
            if self.active_index >= self.pets.len() {
                self.active_index = self.pets.len() - 1;
            }
            self.pets[self.active_index] = pet.clone();
        }
        self.refresh_level();
    }

    pub fn apply_offline_decay_all(&mut self, now: DateTime<Utc>) -> u64 {
        self.apply_offline_decay_all_with_report(now).0
    }

    pub fn apply_offline_decay_all_with_report(
        &mut self,
        now: DateTime<Utc>,
    ) -> (u64, Option<OfflineReport>) {
        let mut max_elapsed = 0u64;
        let mut active_report = None;
        let active_idx = self.active_index.min(self.pets.len().saturating_sub(1));

        for (idx, pet) in self.pets.iter_mut().enumerate() {
            let mistakes_before = pet.care_mistakes;
            let (elapsed, report) = pet.apply_offline_decay_with_report(now);
            if elapsed > max_elapsed {
                max_elapsed = elapsed;
            }
            if idx == active_idx {
                active_report = report;
            } else {
                pet.care_mistakes = mistakes_before;
                pet.hunger = pet.hunger.max(30);
                pet.cleanliness = pet.cleanliness.max(40);
                pet.health = pet.health.max(60);
                pet.is_alive = true;
                pet.is_sick = false;
                pet.refresh_mood();
            }
        }
        (max_elapsed, active_report)
    }

    pub fn tick_decay_all(&mut self) {
        let active_idx = self.active_index.min(self.pets.len().saturating_sub(1));
        for (idx, pet) in self.pets.iter_mut().enumerate() {
            if idx == active_idx {
                pet.tick_decay();
            } else {
                pet.tick_standby();
            }
        }
    }

    pub fn hatched_count(&self) -> usize {
        self.pets
            .iter()
            .filter(|p| p.stage != EvolutionStage::Egg)
            .count()
    }
}
