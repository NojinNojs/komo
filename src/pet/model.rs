use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pet {
    pub name: String,
    pub caretaker_title: String,
    pub species: PetSpecies,
    pub theme: ShellTheme,
    pub stage: EvolutionStage,
    pub mood: PetMood,
    pub is_alive: bool,
    pub cause_of_death: Option<String>,
    pub revival_count: u8,
    pub vigil_steps_completed: u8,
    pub buried: bool,
    pub incubation: u8,
    pub hunger: u8,
    pub happiness: u8,
    pub energy: u8,
    pub health: u8,
    pub cleanliness: u8,
    pub discipline: u8,
    pub weight_grams: u16,
    pub snacks_in_a_row: u8,
    pub has_toothache: bool,
    pub light_on: bool,
    pub attention_call: AttentionCall,
    pub attention_since: Option<DateTime<Utc>>,
    pub poop_count: u8,
    pub meals_since_poop: u8,
    pub bond: u8,
    pub xp: u32,
    pub level: u32,
    pub coins: u32,
    pub care_mistakes: u32,
    pub is_sleeping: bool,
    pub is_sick: bool,
    pub last_action_kind: Option<PetAction>,
    pub spam_streak: u8,
    pub last_pet_speech: Option<String>,
    pub last_pet_thought: Option<String>,
    pub reaction_ticks_left: u8,
    pub last_updated: DateTime<Utc>,
    pub last_warmed_at: Option<DateTime<Utc>>,
    pub last_fed_at: Option<DateTime<Utc>>,
    pub last_played_at: Option<DateTime<Utc>>,
    pub last_trained_at: Option<DateTime<Utc>>,
    pub last_petted_at: Option<DateTime<Utc>>,
    pub age_seconds: u64,
    pub action_log: Vec<String>,
}

impl Default for Pet {
    fn default() -> Self {
        Self::new("Mochi")
    }
}

impl Pet {
    pub fn new(name: &str) -> Self {
        Self::new_custom(name, PetSpecies::MochiCat, ShellTheme::StrawberryPink)
    }

    pub fn new_custom(name: &str, species: PetSpecies, theme: ShellTheme) -> Self {
        let trimmed = name.trim();
        let final_name = if trimmed.is_empty() {
            species.default_name().to_string()
        } else {
            trimmed.to_string()
        };

        Self {
            name: final_name,
            caretaker_title: "Keeper".to_string(),
            species,
            theme,
            stage: EvolutionStage::Egg,
            mood: PetMood::Idle,
            is_alive: true,
            cause_of_death: None,
            revival_count: 0,
            vigil_steps_completed: 0,
            buried: false,
            incubation: 0,
            hunger: 65,
            happiness: 80,
            energy: 90,
            health: 100,
            cleanliness: 100,
            discipline: 25,
            weight_grams: EvolutionStage::Egg.base_weight_grams(),
            snacks_in_a_row: 0,
            has_toothache: false,
            light_on: true,
            attention_call: AttentionCall::None,
            attention_since: None,
            poop_count: 0,
            meals_since_poop: 0,
            bond: 10,
            xp: 0,
            level: 1,
            coins: 0,
            care_mistakes: 0,
            is_sleeping: false,
            is_sick: false,
            last_action_kind: None,
            spam_streak: 0,
            last_pet_speech: None,
            last_pet_thought: None,
            reaction_ticks_left: 0,
            last_updated: Utc::now(),
            last_warmed_at: None,
            last_fed_at: None,
            last_played_at: None,
            last_trained_at: None,
            last_petted_at: None,
            age_seconds: 0,
            action_log: vec![format!(
                "🥚 A mysterious {} egg arrived in your sanctuary!",
                species.display_name()
            )],
        }
    }
}
