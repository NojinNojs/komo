use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ShellTheme {
    #[default]
    StrawberryPink,
    MatchaGreen,
    SkyBlue,
    LavenderDream,
    SunsetGold,
}

impl ShellTheme {
    pub fn all() -> &'static [ShellTheme] {
        &[
            Self::StrawberryPink,
            Self::MatchaGreen,
            Self::SkyBlue,
            Self::LavenderDream,
            Self::SunsetGold,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::StrawberryPink => "🍓 Strawberry Pink",
            Self::MatchaGreen => "🍵 Matcha Green",
            Self::SkyBlue => "💎 Sky Blue",
            Self::LavenderDream => "🔮 Lavender Dream",
            Self::SunsetGold => "🌟 Sunset Gold",
        }
    }

    pub fn next(&self) -> ShellTheme {
        let all = Self::all();
        let idx = all.iter().position(|t| t == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }
}

/// 15 Rich Emotional States so your companion feels genuinely alive!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PetMood {
    #[default]
    Idle,
    Eating,
    Sleeping,
    Sleepy,
    Happy,
    Excited,
    Loved,
    Playful,
    Proud,
    Lonely,
    Angry,
    Overstuffed,
    Sick,
    Training,
    PassedOut,
}

pub type PetState = PetMood;

impl PetMood {
    pub fn all() -> &'static [PetMood] {
        &[
            Self::Idle,
            Self::Eating,
            Self::Sleeping,
            Self::Sleepy,
            Self::Happy,
            Self::Excited,
            Self::Loved,
            Self::Playful,
            Self::Proud,
            Self::Lonely,
            Self::Angry,
            Self::Overstuffed,
            Self::Sick,
            Self::Training,
            Self::PassedOut,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Curious",
            Self::Eating => "Nomming",
            Self::Sleeping => "Dreaming",
            Self::Sleepy => "Drowsy",
            Self::Happy => "Joyful",
            Self::Excited => "Ecstatic",
            Self::Loved => "Affectionate",
            Self::Playful => "Playful",
            Self::Proud => "Proud",
            Self::Lonely => "Lonely",
            Self::Angry => "Grumpy",
            Self::Overstuffed => "Overstuffed",
            Self::Sick => "Sick",
            Self::Training => "Fired Up",
            Self::PassedOut => "Spirit Form",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Idle => "🌈",
            Self::Eating => "🍡",
            Self::Sleeping => "💤",
            Self::Sleepy => "🥱",
            Self::Happy => "💖",
            Self::Excited => "🤩",
            Self::Loved => "🥰",
            Self::Playful => "🎵",
            Self::Proud => "😎",
            Self::Lonely => "🥺",
            Self::Angry => "💢",
            Self::Overstuffed => "🤢",
            Self::Sick => "🤒",
            Self::Training => "💪",
            Self::PassedOut => "👻",
        }
    }

    pub fn badge(&self) -> String {
        format!("{} {}", self.emoji(), self.label())
    }
}
