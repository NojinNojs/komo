use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoodChoice {
    FavoriteSnack,
    HeartyBento,
    WarmMilkTea,
    HerbalSoup,
    StarlightDelight,
}

impl FoodChoice {
    pub fn all() -> &'static [FoodChoice] {
        &[
            Self::FavoriteSnack,
            Self::HeartyBento,
            Self::WarmMilkTea,
            Self::HerbalSoup,
            Self::StarlightDelight,
        ]
    }

    pub fn is_sweet_snack(&self) -> bool {
        matches!(self, Self::WarmMilkTea | Self::StarlightDelight)
    }

    pub fn title(&self, species: PetSpecies) -> String {
        match self {
            Self::FavoriteSnack => format!("🍙 {} [Meal]", species.favorite_snack()),
            Self::HeartyBento => "🍱 Hearty Bento Box [Meal]".to_string(),
            Self::WarmMilkTea => "🧋 Sweet Boba Milk Tea [Snack]".to_string(),
            Self::HerbalSoup => "🍲 Soothing Herbal Soup [Meal]".to_string(),
            Self::StarlightDelight => "🍮 Starlight Candy Tart [Snack]".to_string(),
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Self::FavoriteSnack => "Free • +1 Hungry ♥, +1g, resets sweet streak",
            Self::HeartyBento => "8 coins • +2 Hungry ♥, +2g weight, +5 Energy",
            Self::WarmMilkTea => "Free • +1 Happy ♥, +2g (3 sweets = Toothache!)",
            Self::HerbalSoup => "10 coins • +1 Hungry ♥, +20 Health, cures chill",
            Self::StarlightDelight => "18 coins • +2 Happy ♥, +8 Bond, +25 XP, +3g",
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Self::FavoriteSnack | Self::WarmMilkTea => 0,
            Self::HeartyBento => 8,
            Self::HerbalSoup => 10,
            Self::StarlightDelight => 18,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetAction {
    WarmEgg,
    Feed,
    FeedSnack,
    Play,
    PetLove,
    Talk,
    Train,
    ToggleSleep,
    ToggleLight,
    DisciplineScold,
    Clean,
    Heal,
}

impl PetAction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::WarmEgg => "Warm / Cuddle",
            Self::Feed => "Feed Meal",
            Self::FeedSnack => "Feed Snack",
            Self::Play => "Play Game",
            Self::PetLove => "Praise / Pat",
            Self::Talk => "Talk / Chat",
            Self::Train => "Train",
            Self::ToggleSleep => "Sleep",
            Self::ToggleLight => "Light Switch",
            Self::DisciplineScold => "Discipline (Scold)",
            Self::Clean => "Clean / Flush",
            Self::Heal => "Medicine / Revive",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OfflineReport {
    pub elapsed_seconds: u64,
    pub pet_name: String,
    pub events: Vec<String>,
    pub coins_found: u32,
}

impl OfflineReport {
    pub fn formatted_duration(&self) -> String {
        format_duration_human(self.elapsed_seconds)
    }
}

pub fn format_duration_human(secs: u64) -> String {
    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let mins = (secs % 3_600) / 60;
    let rem_s = secs % 60;
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else if mins > 0 {
        format!("{mins}m {rem_s}s")
    } else {
        format!("{rem_s}s")
    }
}

pub fn format_hearts(filled: u8) -> String {
    let clamped = filled.min(4);
    let mut out = String::with_capacity(8);
    for i in 0..4 {
        if i < clamped {
            out.push('♥');
        } else {
            out.push('·');
        }
    }
    format!("{out} {clamped}/4")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemorialRecord {
    pub pet_name: String,
    pub species: PetSpecies,
    pub stage: EvolutionStage,
    pub age_seconds: u64,
    pub cause_of_death: String,
    pub epitaph: String,
}
