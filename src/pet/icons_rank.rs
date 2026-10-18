use serde::{Deserialize, Serialize};

pub const WARM_COOLDOWN_SECS: i64 = 180;
pub const FEED_COOLDOWN_SECS: i64 = 120;
pub const PLAY_COOLDOWN_SECS: i64 = 150;
pub const TRAIN_COOLDOWN_SECS: i64 = 240;
pub const PET_COOLDOWN_SECS: i64 = 45;
pub const TALK_REWARD_COOLDOWN_SECS: i64 = 180;
pub const LIGHT_REWARD_COOLDOWN_SECS: i64 = 180;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LcdIcon {
    #[default]
    Food,
    Lights,
    Play,
    Medicine,
    Bath,
    Meter,
    Discipline,
    Talk,
    Attention,
}

impl LcdIcon {
    pub fn selectable() -> &'static [LcdIcon] {
        &[
            Self::Food,
            Self::Lights,
            Self::Play,
            Self::Medicine,
            Self::Bath,
            Self::Meter,
            Self::Discipline,
            Self::Talk,
        ]
    }

    pub fn top_row() -> &'static [LcdIcon] {
        &[Self::Food, Self::Lights, Self::Play, Self::Medicine]
    }

    pub fn bottom_row() -> &'static [LcdIcon] {
        &[Self::Bath, Self::Meter, Self::Discipline, Self::Talk]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Food => "🍙Food",
            Self::Lights => "💡Lit",
            Self::Play => "🎮Game",
            Self::Medicine => "💉Meds",
            Self::Bath => "🚽Bath",
            Self::Meter => "📏Stat",
            Self::Discipline => "📢Scold",
            Self::Talk => "💬Talk",
            Self::Attention => "🔔Call",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialoguePrompt {
    pub inner_thought: String,
    pub pet_line: String,
    pub option_1: String,
    pub option_2: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AttentionCall {
    #[default]
    None,
    NeedsCare,
    Misbehaving,
    Crying,
}

impl AttentionCall {
    pub fn is_active(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::None => "🔔 Quiet",
            Self::NeedsCare => "🔔 CALL: Care!",
            Self::Misbehaving => "🔔 CALL: Scold [D]",
            Self::Crying => "🔔 CALL: Praise [T]",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CareRank {
    PerfectKeeper,
    StarGuardian,
    CozyFriend,
    MischievousRascal,
}

impl CareRank {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::PerfectKeeper => "👑 S-Rank (Perfect Keeper)",
            Self::StarGuardian => "🌟 A-Rank (Star Guardian)",
            Self::CozyFriend => "🌱 B-Rank (Cozy Friend)",
            Self::MischievousRascal => "🩹 C-Rank (Rascal)",
        }
    }
}
