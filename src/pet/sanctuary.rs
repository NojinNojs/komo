use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sanctuary {
    pub pets: Vec<Pet>,
    pub active_index: usize,
    pub sanctuary_level: u32,
    pub caretaker_title: String,
    pub caretaker_karma: i32,
    pub memorials: Vec<MemorialRecord>,
}

impl Default for Sanctuary {
    fn default() -> Self {
        Self::new_with_pet(Pet::default())
    }
}

impl Sanctuary {
    pub fn new_with_pet(mut pet: Pet) -> Self {
        let title = pet.caretaker_label().to_string();
        pet.caretaker_title = title.clone();
        Self {
            pets: vec![pet],
            active_index: 0,
            sanctuary_level: 1,
            caretaker_title: title,
            caretaker_karma: 0,
            memorials: Vec::new(),
        }
    }

    pub fn set_caretaker_title_all(&mut self, title: &str) {
        let trimmed = title.trim();
        if !trimmed.is_empty() {
            self.caretaker_title = trimmed.to_string();
            for p in &mut self.pets {
                p.caretaker_title = trimmed.to_string();
                p.last_pet_speech = None;
                p.last_pet_thought = None;
                p.reaction_ticks_left = 0;
            }
        }
    }

    pub fn can_adopt_new(&self) -> Result<(), String> {
        let active = self.active_pet();
        if !active.is_alive && !active.buried {
            return Err(format!(
                "🕯 Cannot adopt yet! {} has fallen — you must hold the 3-step Burial Vigil [B] ({}/3) or revive them [H] first.",
                active.name, active.vigil_steps_completed
            ));
        }
        Ok(())
    }

    pub fn record_memorial_if_buried(&mut self, pet: &Pet) {
        if !pet.is_alive && pet.buried {
            let already = self
                .memorials
                .iter()
                .any(|m| m.pet_name == pet.name && m.age_seconds == pet.age_seconds);
            if !already {
                let caretaker = pet.caretaker_label();
                self.memorials.push(MemorialRecord {
                    pet_name: pet.name.clone(),
                    species: pet.species,
                    stage: pet.stage,
                    age_seconds: pet.age_seconds,
                    cause_of_death: pet
                        .cause_of_death
                        .clone()
                        .unwrap_or_else(|| "Neglect".to_string()),
                    epitaph: format!("Here rests {}, remembered by {}.", pet.name, caretaker),
                });
                self.caretaker_karma = self.caretaker_karma.saturating_sub(15);
            }
        }
    }

    pub(super) fn refresh_level(&mut self) {
        self.sanctuary_level = 1 + (self.pets.len() as u32 / 2);
    }

    pub fn active_pet(&self) -> &Pet {
        let idx = self.active_index.min(self.pets.len().saturating_sub(1));
        &self.pets[idx]
    }

    pub fn active_pet_mut(&mut self) -> &mut Pet {
        if self.pets.is_empty() {
            self.pets.push(Pet::default());
        }
        if self.active_index >= self.pets.len() {
            self.active_index = self.pets.len() - 1;
        }
        &mut self.pets[self.active_index]
    }

    pub fn add_pet(&mut self, mut pet: Pet) -> usize {
        if pet.caretaker_title.trim().is_empty() || pet.caretaker_title == "Keeper" {
            if !self.caretaker_title.trim().is_empty() {
                pet.caretaker_title = self.caretaker_title.clone();
            }
        } else {
            self.caretaker_title = pet.caretaker_title.clone();
        }
        // Caretaker Karma consequence: if karma is negative from past neglect deaths,
        // newly adopted pets start cautious with lower initial bond!
        if self.caretaker_karma < 0 {
            pet.bond = 4;
        } else if self.caretaker_karma >= 20 {
            pet.bond = 18;
        }
        self.pets.push(pet);
        self.active_index = self.pets.len() - 1;
        self.refresh_level();
        self.active_index
    }

    pub fn switch_pet(&mut self, index: usize) -> bool {
        if index < self.pets.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }
}
