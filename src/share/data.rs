//! Decoupled card payload plus a convenience constructor from a live `Pet`.

use super::font_5x7::has_glyph;
use crate::pet::{Accessory, CareRank, EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};

/// Uppercase and strip every char without a 5x7 glyph, so emoji (including any
/// ZWJ/VS-16 sequences) can never reach the card, then collapse stray spaces.
pub fn sanitize_card_text(s: &str) -> String {
    let mapped: String = s
        .chars()
        .map(|c| {
            let u = c.to_ascii_uppercase();
            if has_glyph(u) {
                u
            } else {
                ' '
            }
        })
        .collect();
    mapped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn rank_info(rank: CareRank) -> (&'static str, u8) {
    match rank {
        CareRank::PerfectKeeper => ("S-RANK", 0),
        CareRank::StarGuardian => ("A-RANK", 1),
        CareRank::CozyFriend => ("B-RANK", 2),
        CareRank::MischievousRascal => ("C-RANK", 3),
    }
}

fn stage_label(stage: EvolutionStage) -> &'static str {
    match stage {
        EvolutionStage::Egg => "EGG",
        EvolutionStage::Baby => "BABY",
        EvolutionStage::Teen => "TEEN",
        EvolutionStage::Adult => "ADULT",
        EvolutionStage::CyberElder => "ELDER",
    }
}

pub type CustomPalette = ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8));

/// Everything the card renderer needs — no storage, cooldowns, or serde types.
pub struct ShareCardData {
    pub name: String,
    pub species_name: String,
    pub keeper_title: String,
    pub stage_label: String,
    pub rank_label: String,
    pub rank_tier: u8,
    pub coat_label: String,
    pub accessory_label: String,
    pub accessory: Accessory,
    pub level: u32,
    pub xp: u32,
    pub coins: u32,
    pub age_seconds: u64,
    pub hunger: u8,
    pub happiness: u8,
    pub health: u8,
    pub energy: u8,
    pub bond: u8,
    pub discipline: u8,
    pub light_on: bool,
    pub species: PetSpecies,
    pub stage: EvolutionStage,
    pub mood: PetMood,
    pub theme: ShellTheme,
    pub custom_palette: Option<CustomPalette>,
}

impl ShareCardData {
    pub fn from_pet(pet: &Pet) -> Self {
        let (rank_label, rank_tier) = rank_info(pet.care_rank());
        let name = sanitize_card_text(&pet.name);
        let keeper = sanitize_card_text(pet.caretaker_label());
        Self {
            name: if name.is_empty() {
                "FRIEND".to_string()
            } else {
                name
            },
            species_name: sanitize_card_text(pet.species.display_name()),
            keeper_title: if keeper.is_empty() {
                "KEEPER".to_string()
            } else {
                keeper
            },
            stage_label: stage_label(pet.stage).to_string(),
            rank_label: rank_label.to_string(),
            rank_tier,
            coat_label: sanitize_card_text(pet.coat.display_name()),
            accessory_label: sanitize_card_text(pet.accessory.display_name()),
            accessory: pet.accessory,
            level: pet.level,
            xp: pet.xp,
            coins: pet.coins,
            age_seconds: pet.age_seconds,
            hunger: pet.hunger,
            happiness: pet.happiness,
            health: pet.health,
            energy: pet.energy,
            bond: pet.bond,
            discipline: pet.discipline,
            light_on: pet.light_on,
            species: pet.species,
            stage: pet.stage,
            mood: pet.mood,
            theme: pet.theme,
            custom_palette: pet.coat.palette_override(),
        }
    }

    pub fn age_label(&self) -> String {
        format!(
            "AGE {}H {}M",
            self.age_seconds / 3600,
            (self.age_seconds % 3600) / 60
        )
    }

    pub fn subtitle(&self) -> String {
        format!(
            "{} \u{2022} KEEPER: {}",
            self.species_name, self.keeper_title
        )
    }

    fn coat_parts(&self) -> (&str, &str) {
        let coat = if self.coat_label.is_empty() {
            "DEFAULT"
        } else {
            self.coat_label.as_str()
        };
        let gear = if self.accessory_label.is_empty() {
            "NONE"
        } else {
            self.accessory_label.as_str()
        };
        (coat, gear)
    }

    pub fn coat_line(&self) -> String {
        let (coat, gear) = self.coat_parts();
        format!("COAT {coat} \u{2022} GEAR {gear}")
    }

    /// Prefix-free fallback for narrow panels so clipping never cuts a word.
    pub fn coat_line_compact(&self) -> String {
        let (coat, gear) = self.coat_parts();
        format!("{coat} \u{2022} {gear}")
    }
}
