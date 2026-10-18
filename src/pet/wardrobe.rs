use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CoatPreset {
    #[default]
    Classic,
    SakuraMochi,
    MatchaCream,
    CalicoSunset,
    TuxedoMidnight,
    StarlightGold,
}

impl CoatPreset {
    pub fn all() -> &'static [CoatPreset] {
        &[
            Self::Classic,
            Self::SakuraMochi,
            Self::MatchaCream,
            Self::CalicoSunset,
            Self::TuxedoMidnight,
            Self::StarlightGold,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Classic => "🌸 Classic Mochi",
            Self::SakuraMochi => "🌸 Sakura Mochi",
            Self::MatchaCream => "🍵 Matcha Cream",
            Self::CalicoSunset => "🌇 Calico Sunset",
            Self::TuxedoMidnight => "🌃 Tuxedo Midnight",
            Self::StarlightGold => "⭐ Starlight Gold",
        }
    }

    pub fn badge(&self) -> &'static str {
        self.display_name()
    }

    pub fn unlock_cost(&self) -> u32 {
        match self {
            Self::Classic => 0,
            Self::SakuraMochi => 20,
            Self::MatchaCream => 25,
            Self::CalicoSunset => 30,
            Self::TuxedoMidnight => 40,
            Self::StarlightGold => 60,
        }
    }

    pub fn unlock_level(&self) -> u32 {
        match self {
            Self::Classic => 1,
            Self::SakuraMochi => 2,
            Self::MatchaCream => 3,
            Self::CalicoSunset => 4,
            Self::TuxedoMidnight => 5,
            Self::StarlightGold => 7,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|c| c == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    #[allow(clippy::type_complexity)]
    pub fn palette_override(&self) -> Option<((u8, u8, u8), (u8, u8, u8), (u8, u8, u8))> {
        match self {
            Self::Classic => None,
            Self::SakuraMochi => Some(((244, 194, 194), (255, 240, 245), (255, 182, 193))),
            Self::MatchaCream => Some(((188, 214, 160), (240, 245, 235), (205, 228, 180))),
            Self::CalicoSunset => Some(((250, 160, 90), (255, 220, 170), (90, 60, 45))),
            Self::TuxedoMidnight => Some(((40, 42, 54), (235, 235, 245), (255, 255, 255))),
            Self::StarlightGold => Some(((255, 200, 80), (255, 235, 170), (255, 250, 210))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Accessory {
    #[default]
    None,
    SilkRibbon,
    BellCollar,
    CozyScarf,
    TinyCrown,
    WizardHat,
    StarlightHalo,
}

impl Accessory {
    pub fn all() -> &'static [Accessory] {
        &[
            Self::None,
            Self::SilkRibbon,
            Self::BellCollar,
            Self::CozyScarf,
            Self::TinyCrown,
            Self::WizardHat,
            Self::StarlightHalo,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SilkRibbon => "🎀 Silk Ribbon",
            Self::BellCollar => "🔔 Bell Collar",
            Self::CozyScarf => "🧣 Cozy Scarf",
            Self::TinyCrown => "👑 Tiny Crown",
            Self::WizardHat => "🧙 Wizard Hat",
            Self::StarlightHalo => "😇 Starlight Halo",
        }
    }

    pub fn badge(&self) -> &'static str {
        self.display_name()
    }

    pub fn unlock_cost(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::SilkRibbon => 15,
            Self::BellCollar => 25,
            Self::CozyScarf => 35,
            Self::TinyCrown => 50,
            Self::WizardHat => 65,
            Self::StarlightHalo => 80,
        }
    }

    pub fn unlock_level(&self) -> u32 {
        match self {
            Self::None => 1,
            Self::SilkRibbon => 2,
            Self::BellCollar => 3,
            Self::CozyScarf => 4,
            Self::TinyCrown => 5,
            Self::WizardHat => 6,
            Self::StarlightHalo => 8,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|a| a == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }
}
