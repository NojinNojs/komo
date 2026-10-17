use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum EvolutionStage {
    #[default]
    Egg,
    Baby,
    Teen,
    Adult,
    CyberElder,
}

impl EvolutionStage {
    pub fn all() -> &'static [EvolutionStage] {
        &[
            Self::Egg,
            Self::Baby,
            Self::Teen,
            Self::Adult,
            Self::CyberElder,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Egg => "Egg",
            Self::Baby => "Baby",
            Self::Teen => "Teen",
            Self::Adult => "Adult",
            Self::CyberElder => "Starlight Elder",
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.name()
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::Egg => "🥚 Egg",
            Self::Baby => "🍼 Baby",
            Self::Teen => "🌱 Teen",
            Self::Adult => "🌸 Adult",
            Self::CyberElder => "🌟 Elder",
        }
    }

    pub fn base_weight_grams(&self) -> u16 {
        match self {
            Self::Egg => 5,
            Self::Baby => 5,
            Self::Teen => 15,
            Self::Adult => 30,
            Self::CyberElder => 45,
        }
    }

    pub fn min_xp(&self) -> u32 {
        match self {
            Self::Egg => 0,
            Self::Baby => 100,
            Self::Teen => 250,
            Self::Adult => 600,
            Self::CyberElder => 1_200,
        }
    }

    pub fn min_bond(&self) -> u8 {
        match self {
            Self::Egg => 0,
            Self::Baby => 12,
            Self::Teen => 18,
            Self::Adult => 32,
            Self::CyberElder => 50,
        }
    }

    pub fn min_age_seconds(&self) -> u64 {
        0
    }

    pub fn next_stage(&self) -> Option<EvolutionStage> {
        match self {
            Self::Egg => Some(Self::Baby),
            Self::Baby => Some(Self::Teen),
            Self::Teen => Some(Self::Adult),
            Self::Adult => Some(Self::CyberElder),
            Self::CyberElder => None,
        }
    }

    pub fn from_xp(xp: u32) -> EvolutionStage {
        if xp >= 1_200 {
            Self::CyberElder
        } else if xp >= 600 {
            Self::Adult
        } else if xp >= 250 {
            Self::Teen
        } else if xp >= 100 {
            Self::Baby
        } else {
            Self::Egg
        }
    }

    pub fn max_stat_bonus(&self) -> u8 {
        match self {
            Self::Egg => 0,
            Self::Baby => 5,
            Self::Teen => 10,
            Self::Adult => 15,
            Self::CyberElder => 25,
        }
    }
}
