//! Core pet model for **Mochigotchi v1.1.0** — reverse-engineered authentic
//! Bandai Tamagotchi mechanics (8 LCD bezel icons including `💬Talk`, 4 Hungry/
//! Happy Hearts, Meal vs. Sweet Snack weight & toothache system, Room Light
//! switch, Attention Calls, Discipline scold/praise, Care Mistakes, Care Ranks,
//! Interactive Pet Dialogue & Inner Thoughts, and the multi-pet [`Sanctuary`]).
//!
//! All emoji icons use single-codepoint East_Asian_Width=Wide glyphs (zero ZWJ
//! or U+FE0F variation selectors) so terminal column widths match 100% across
//! Ghostty, Kitty, WezTerm, Alacritty, and Ratatui.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Seconds you must wait between two manual egg-warming sessions (3 minutes).
pub const WARM_COOLDOWN_SECS: i64 = 180;
/// Seconds you must wait between two meals (2 minutes).
pub const FEED_COOLDOWN_SECS: i64 = 120;
/// Seconds you must wait between two play sessions (2.5 minutes).
pub const PLAY_COOLDOWN_SECS: i64 = 150;
/// Seconds you must wait between two training sessions (4 minutes).
pub const TRAIN_COOLDOWN_SECS: i64 = 240;
/// Seconds you must wait between two headpat/chat sessions (45 seconds).
pub const PET_COOLDOWN_SECS: i64 = 45;

/// The interactive LCD bezel icons of an authentic Bandai Tamagotchi + `💬Talk`.
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
            Self::Lights => "💡Light",
            Self::Play => "🎮Game",
            Self::Medicine => "💉Meds",
            Self::Bath => "🚽Bath",
            Self::Meter => "📏Meter",
            Self::Discipline => "📢Scold",
            Self::Talk => "💬Talk",
            Self::Attention => "🔔Call",
        }
    }
}

/// Interactive conversation prompt when you open the **Heart-to-Heart Talk** screen (`[Y]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialoguePrompt {
    pub inner_thought: String,
    pub pet_line: String,
    pub option_1: String,
    pub option_2: String,
}

/// Authentic Tamagotchi Attention Call state (`🔔`).
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

/// Tamagotchi Care Rank earned from your Care Mistakes and Discipline score.
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

/// Life stages of every companion.
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
            Self::CyberElder => 40,
        }
    }

    pub fn min_xp(&self) -> u32 {
        match self {
            Self::Egg => 0,
            Self::Baby => 100,
            Self::Teen => 450,
            Self::Adult => 1_400,
            Self::CyberElder => 3_500,
        }
    }

    pub fn min_bond(&self) -> u8 {
        match self {
            Self::Egg => 0,
            Self::Baby => 12,
            Self::Teen => 35,
            Self::Adult => 65,
            Self::CyberElder => 90,
        }
    }

    pub fn min_age_seconds(&self) -> u64 {
        match self {
            Self::Egg => 0,
            Self::Baby => 0,
            Self::Teen => 6 * 3_600,
            Self::Adult => 24 * 3_600,
            Self::CyberElder => 72 * 3_600,
        }
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
        if xp >= 3_500 {
            Self::CyberElder
        } else if xp >= 1_400 {
            Self::Adult
        } else if xp >= 450 {
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

/// Ten collectible companions: four Cozy Classics and six Mythical &
/// Fantasy creatures (dragons included).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PetSpecies {
    #[default]
    MochiCat,
    BobaPup,
    CloudBun,
    StarPengu,
    IgnisDragon,
    AzureWyrm,
    EmberFox,
    PhoenixChick,
    CrystalUni,
    SlimeKing,
}

impl PetSpecies {
    pub fn all() -> &'static [PetSpecies] {
        &[
            Self::MochiCat,
            Self::BobaPup,
            Self::CloudBun,
            Self::StarPengu,
            Self::IgnisDragon,
            Self::AzureWyrm,
            Self::EmberFox,
            Self::PhoenixChick,
            Self::CrystalUni,
            Self::SlimeKing,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "🐱 Neko Mochi",
            Self::BobaPup => "🐶 Boba Pup",
            Self::CloudBun => "🐰 Cloud Bun",
            Self::StarPengu => "🐧 Star Pengu",
            Self::IgnisDragon => "🐉 Ignis Drake",
            Self::AzureWyrm => "🐲 Azure Wyrm",
            Self::EmberFox => "🦊 Kitsune Nine",
            Self::PhoenixChick => "🦅 Sol Phoenix",
            Self::CrystalUni => "🦄 Astra Unicorn",
            Self::SlimeKing => "👑 Royal Jelly",
        }
    }

    pub fn family(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun | Self::StarPengu => "Cozy Classics",
            _ => "Mythical & Fantasy",
        }
    }

    pub fn short_family(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun | Self::StarPengu => "Cozy",
            _ => "Mythical",
        }
    }

    pub fn rarity(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun => "Common ★",
            Self::StarPengu | Self::SlimeKing => "Rare ★★",
            Self::EmberFox | Self::CrystalUni => "Epic ★★★",
            Self::AzureWyrm => "Mythic ★★★★",
            Self::IgnisDragon | Self::PhoenixChick => "Legendary ★★★★★",
        }
    }

    pub fn star_bar(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun => "★☆☆☆☆",
            Self::StarPengu | Self::SlimeKing => "★★☆☆☆",
            Self::EmberFox | Self::CrystalUni => "★★★☆☆",
            Self::AzureWyrm => "★★★★☆",
            Self::IgnisDragon | Self::PhoenixChick => "★★★★★",
        }
    }

    pub fn element_badge(&self) -> &'static str {
        match self {
            Self::MochiCat => "🍡 Sweet",
            Self::BobaPup => "🧋 Tapioca",
            Self::CloudBun => "🌈 Sky",
            Self::StarPengu => "🌌 Aurora",
            Self::IgnisDragon => "🔥 Dragonfire",
            Self::AzureWyrm => "🌊 Celestial Tide",
            Self::EmberFox => "🦊 Spirit Foxfire",
            Self::PhoenixChick => "🌞 Sunburst",
            Self::CrystalUni => "🌟 Starlight",
            Self::SlimeKing => "💧 Mystic Slime",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::MochiCat => "A squishy sweet rice-cake kitty who loves warm naps and taiyaki.",
            Self::BobaPup => "A loyal milk-tea puppy buzzing with bouncy tapioca energy.",
            Self::CloudBun => "A cotton-soft sky bunny that floats on daydreams and breezes.",
            Self::StarPengu => "A cosmic snow penguin who collects falling aurora stars.",
            Self::IgnisDragon => {
                "A noble crimson whelp whose ember breath warms the whole sanctuary."
            }
            Self::AzureWyrm => "An ancient sky-and-sea dragon guarding a luminous tidal pearl.",
            Self::EmberFox => "A shrine fox wreathed in dancing lanterns of cyan foxfire.",
            Self::PhoenixChick => "An immortal solar firebird whose feathers glow like dawn.",
            Self::CrystalUni => "A prism-horned celestial foal galloping across constellations.",
            Self::SlimeKing => "A bouncy gelatinous monarch wearing a tiny golden crown.",
        }
    }

    pub fn default_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "Mochi",
            Self::BobaPup => "Boba",
            Self::CloudBun => "Cloudy",
            Self::StarPengu => "Pebble",
            Self::IgnisDragon => "Ignis",
            Self::AzureWyrm => "Ao",
            Self::EmberFox => "Kitsune",
            Self::PhoenixChick => "Sol",
            Self::CrystalUni => "Astra",
            Self::SlimeKing => "Puni",
        }
    }

    pub fn favorite_snack(&self) -> &'static str {
        match self {
            Self::MochiCat => "Taiyaki Fish 🐟",
            Self::BobaPup => "Honey Biscuit 🦴",
            Self::CloudBun => "Sweet Carrot 🥕",
            Self::StarPengu => "Snowberry 🫐",
            Self::IgnisDragon => "Flame Chili 🔥",
            Self::AzureWyrm => "Moon Pearl 🫧",
            Self::EmberFox => "Inari Tofu 🦊",
            Self::PhoenixChick => "Sunfire Berry 🌞",
            Self::CrystalUni => "Starlight Sugar 🌟",
            Self::SlimeKing => "Crystal Dew 💧",
        }
    }

    pub fn is_mythical(&self) -> bool {
        self.family() == "Mythical & Fantasy"
    }
}

/// The five LCD shell colorways you can wrap your companion in.
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

/// What your companion is doing right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PetMood {
    #[default]
    Idle,
    Eating,
    Sleeping,
    Happy,
    Sick,
    Training,
}

pub type PetState = PetMood;

impl PetMood {
    pub fn all() -> &'static [PetMood] {
        &[
            Self::Idle,
            Self::Eating,
            Self::Sleeping,
            Self::Happy,
            Self::Sick,
            Self::Training,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Cozy & Curious",
            Self::Eating => "Nomming Happily",
            Self::Sleeping => "Dreaming Zzz",
            Self::Happy => "Sparkling Joy",
            Self::Sick => "Feeling Unwell",
            Self::Training => "Leveling Up!",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Idle => "🌈",
            Self::Eating => "🍡",
            Self::Sleeping => "💤",
            Self::Happy => "💖",
            Self::Sick => "🤒",
            Self::Training => "💪",
        }
    }
}

/// Five foods available in the Pantry (`[M]`): Meals vs. Sweet Snacks!
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

/// Every care action the player can trigger.
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
            Self::Heal => "Medicine",
        }
    }
}

/// Summary of what happened to your companions while the terminal was closed.
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

/// Render 0..=4 classic Tamagotchi hearts (`♥♥♥· 3/4`).
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

/// A single companion: everything about one creature in your Sanctuary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pet {
    pub name: String,
    pub species: PetSpecies,
    pub theme: ShellTheme,
    pub stage: EvolutionStage,
    pub mood: PetMood,
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
            species,
            theme,
            stage: EvolutionStage::Egg,
            mood: PetMood::Idle,
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

    pub fn hungry_hearts(&self) -> u8 {
        match self.hunger {
            0..=14 => 0,
            15..=39 => 1,
            40..=64 => 2,
            65..=84 => 3,
            _ => 4,
        }
    }

    pub fn happy_hearts(&self) -> u8 {
        match self.happiness {
            0..=14 => 0,
            15..=39 => 1,
            40..=64 => 2,
            65..=84 => 3,
            _ => 4,
        }
    }

    pub fn discipline_blocks(&self) -> u8 {
        (self.discipline / 25).min(4)
    }

    pub fn is_overweight(&self) -> bool {
        self.weight_grams >= self.stage.base_weight_grams() + 18
    }

    pub fn care_rank(&self) -> CareRank {
        if self.care_mistakes == 0 && self.discipline >= 75 {
            CareRank::PerfectKeeper
        } else if self.care_mistakes <= 2 && self.discipline >= 50 {
            CareRank::StarGuardian
        } else if self.care_mistakes <= 5 {
            CareRank::CozyFriend
        } else {
            CareRank::MischievousRascal
        }
    }

    /// Generate a rich, species-aware interactive conversation prompt (`[Y]` / `💬Talk`)
    /// so you can see what your companion is thinking and respond to them!
    pub fn dialogue_prompt(&self, tick: usize) -> DialoguePrompt {
        if self.stage == EvolutionStage::Egg {
            return DialoguePrompt {
                inner_thought:
                    "💭 (Inside the shell, a tiny silhouette curls toward the sound of your voice.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"*tap... tap...* {}'s egg vibrates softly in response to you!\"",
                    self.name
                ),
                option_1: "1. Hum a cozy lullaby to the egg (+Bond)".to_string(),
                option_2: "2. Promise to share delicious snacks once hatched (+Warmth)".to_string(),
            };
        }

        if self.is_sleeping {
            return DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} is sleep-talking about chasing floating {} in the clouds...)",
                    self.name,
                    self.species.favorite_snack()
                ),
                pet_line: format!(
                    "💬 \"*mumble* ...save the biggest {} for my keeper... Zzz...\"",
                    self.species.favorite_snack()
                ),
                option_1: "1. Whisper 'Sweet dreams!' and tuck the blanket (+Bond)".to_string(),
                option_2: "2. Leave a midnight treat by their pillow (+Happy)".to_string(),
            };
        }

        let topic = (tick / 5 + self.xp as usize) % 3;
        match (self.species, topic) {
            (PetSpecies::IgnisDragon, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (Ignis is puffing tiny golden smoke rings and eyeing the sky eagerly.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"Hey keeper! My dragon wings feel stronger today! What should {} practice next?\"",
                    self.name
                ),
                option_1: "1. Soar above the clouds together (+12 XP, +4 Bond)".to_string(),
                option_2: "2. Roast sweet marshmallows with dragonfire (+12 Happy, +5 coins)"
                    .to_string(),
            },
            (PetSpecies::AzureWyrm, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (The tidal pearl in Azure Wyrm's claws shimmers with a calm ocean glow.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"Keeper, listen closely... can you hear the ancient ocean tide inside {}'s pearl?\"",
                    self.name
                ),
                option_1: "1. Meditate by the starlit tide together (+6 Bond, +10 XP)".to_string(),
                option_2: "2. Polish the moon pearl until it sparkles (+12 Happy, +5 coins)"
                    .to_string(),
            },
            (PetSpecies::EmberFox, 0) => DialoguePrompt {
                inner_thought:
                    "💭 (Kitsune's ears twitch as cyan foxfire lanterns dance in a circle.)"
                        .to_string(),
                pet_line: format!(
                    "💬 \"Hehe~ {} learned a new shrine illusion! Want to see a foxfire firework show?\"",
                    self.name
                ),
                option_1: "1. Watch the cyan foxfire fireworks (+14 Happy, +4 Bond)".to_string(),
                option_2: "2. Practice shrine riddles together (+12 XP, +5% Discipline)"
                    .to_string(),
            },
            (_, 1) => DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} is secretly wondering what your favorite part of the day was...)",
                    self.name
                ),
                pet_line: "💬 \"Whenever you open the terminal, my whole habitat lights up! How are you feeling today?\""
                    .to_string(),
                option_1: "1. 'Ready for an adventure together!' (+12 XP, +4 Bond)".to_string(),
                option_2: "2. 'Just relaxing with my favorite companion~' (+14 Happy, +5 Bond)"
                    .to_string(),
            },
            _ => DialoguePrompt {
                inner_thought: format!(
                    "💭 ({} is daydreaming about {} and shiny sanctuary treasures!)",
                    self.name,
                    self.species.favorite_snack()
                ),
                pet_line: "💬 \"Look what I found while exploring the garden! Should we celebrate with a game or a story?\""
                    .to_string(),
                option_1: "1. Share a heroic legend of the Sanctuary (+12 XP, +5 coins)".to_string(),
                option_2: "2. Do a happy victory dance together (+12 Happy, +5 Bond)".to_string(),
            },
        }
    }

    /// Respond to your companion's dialogue prompt (`choice == 1` or `2`).
    pub fn answer_dialogue(&mut self, choice: u8) -> String {
        let now = Utc::now();
        self.last_updated = now;

        if self.stage == EvolutionStage::Egg {
            self.bond = (self.bond + 3).min(100);
            self.incubation = (self.incubation + 2).min(99);
            let msg = format!(
                "💬 You spoke warmly to {}'s egg! It wiggled happily (+2% warmth, +3 Bond).",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }

        if choice == 1 {
            self.bond = (self.bond + 5).min(100);
            self.happiness = (self.happiness + 8).min(100);
            self.discipline = (self.discipline + 4).min(100);
            self.add_xp(12);
            self.mood = if self.is_sleeping {
                PetMood::Sleeping
            } else {
                PetMood::Happy
            };
            let msg = format!(
                "💬 Had a deep heart-to-heart chat with {}! (+12 XP, +5 Bond, +8 Happy)",
                self.name
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.bond = (self.bond + 5).min(100);
            self.happiness = (self.happiness + 14).min(100);
            self.coins = self.coins.saturating_add(5);
            self.add_xp(8);
            self.mood = if self.is_sleeping {
                PetMood::Sleeping
            } else {
                PetMood::Happy
            };
            let msg = format!(
                "💬 Shared a joyful moment with {}! They gifted you +5 coins! (+14 Happy, +5 Bond)",
                self.name
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn rename(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        if !trimmed.is_empty() {
            self.name = trimmed.to_string();
            self.log_action(format!("🔖 Renamed to {}!", self.name));
        }
    }

    pub fn log_action(&mut self, msg: impl Into<String>) {
        self.action_log.push(msg.into());
        while self.action_log.len() > 25 {
            self.action_log.remove(0);
        }
    }

    pub fn clear_cooldowns(&mut self) {
        self.last_warmed_at = None;
        self.last_fed_at = None;
        self.last_played_at = None;
        self.last_trained_at = None;
        self.last_petted_at = None;
    }

    fn remaining_secs(last: Option<DateTime<Utc>>, window: i64, now: DateTime<Utc>) -> u64 {
        match last {
            Some(t) => {
                let elapsed = now.signed_duration_since(t).num_seconds();
                if elapsed < window {
                    (window - elapsed) as u64
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    pub fn warm_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_warmed_at, WARM_COOLDOWN_SECS, now)
    }

    pub fn feed_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_fed_at, FEED_COOLDOWN_SECS, now)
    }

    pub fn play_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_played_at, PLAY_COOLDOWN_SECS, now)
    }

    pub fn train_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_trained_at, TRAIN_COOLDOWN_SECS, now)
    }

    pub fn pet_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_petted_at, PET_COOLDOWN_SECS, now)
    }

    pub fn refresh_attention(&mut self) {
        if self.stage == EvolutionStage::Egg {
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
            return;
        }

        let urgent_need = self.is_sick
            || self.has_toothache
            || self.hungry_hearts() == 0
            || self.happy_hearts() == 0
            || self.poop_count >= 2
            || (self.is_sleeping && self.light_on);

        if urgent_need {
            if self.attention_call != AttentionCall::NeedsCare {
                self.attention_call = AttentionCall::NeedsCare;
                self.attention_since = Some(Utc::now());
            }
        } else if self.attention_call == AttentionCall::NeedsCare {
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
        }
    }

    pub fn refresh_mood(&mut self) {
        if self.weight_grams < self.stage.base_weight_grams() {
            self.weight_grams = self.stage.base_weight_grams();
        }
        if self.is_sick || self.has_toothache || self.health <= 35 {
            if self.health <= 35 {
                self.is_sick = true;
            }
            self.mood = PetMood::Sick;
        } else if self.is_sleeping {
            self.mood = PetMood::Sleeping;
        } else if self.happiness >= 80 && self.hunger >= 50 {
            self.mood = PetMood::Happy;
        } else {
            self.mood = PetMood::Idle;
        }
        self.refresh_attention();
    }

    pub fn update_mood(&mut self) {
        self.refresh_mood();
    }

    pub fn level_up_threshold(&self) -> u32 {
        100
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp = self.xp.saturating_add(amount);
        self.level = 1 + (self.xp / self.level_up_threshold());
        self.check_evolution()
    }

    pub fn check_evolution(&mut self) -> bool {
        if self.stage == EvolutionStage::Egg {
            if self.incubation >= 100 {
                self.stage = EvolutionStage::Baby;
                self.weight_grams = self
                    .weight_grams
                    .max(EvolutionStage::Baby.base_weight_grams());
                self.xp = self.xp.max(EvolutionStage::Baby.min_xp());
                self.level = self.level.max(2);
                return true;
            }
            return false;
        }

        let mut evolved = false;
        while let Some(next) = self.stage.next_stage() {
            if self.xp >= next.min_xp()
                && self.bond >= next.min_bond()
                && self.age_seconds >= next.min_age_seconds()
            {
                self.stage = next;
                self.weight_grams = self.weight_grams.max(next.base_weight_grams());
                let bonus = u32::from(next.max_stat_bonus());
                let rank_bonus = if self.care_mistakes <= 2 && self.discipline >= 50 {
                    bonus
                } else {
                    0
                };
                let reward = 25 + rank_bonus;
                self.coins = self.coins.saturating_add(reward);
                self.happiness = (self.happiness + 12).min(100);
                self.health = (self.health + 10).min(100);
                self.log_action(format!(
                    "🌟 Evolution! {} grew into {} ({}, +{} coins)!",
                    self.name,
                    self.stage.badge(),
                    self.care_rank().badge(),
                    reward
                ));
                evolved = true;
            } else {
                break;
            }
        }
        evolved
    }

    pub fn xp_progress_ratio(&self) -> f64 {
        if self.stage == EvolutionStage::Egg {
            return (self.incubation as f64 / 100.0).clamp(0.0, 1.0);
        }
        match self.stage.next_stage() {
            Some(next) => {
                let cur = self.stage.min_xp();
                let nxt = next.min_xp();
                if nxt <= cur {
                    1.0
                } else {
                    ((self.xp.saturating_sub(cur)) as f64 / (nxt - cur) as f64).clamp(0.0, 1.0)
                }
            }
            None => 1.0,
        }
    }

    pub fn next_evolution_requirements(&self) -> String {
        if self.stage == EvolutionStage::Egg {
            return format!(
                "Goal: Hatch at 100% Incubation (now {}% • [W] +20% / 3m)",
                self.incubation
            );
        }
        match self.stage.next_stage() {
            Some(next) => format!(
                "Next {} • XP {}/{} • Bond {}/{} • Age {}/{}",
                next.badge(),
                self.xp,
                next.min_xp(),
                self.bond,
                next.min_bond(),
                format_duration_human(self.age_seconds),
                format_duration_human(next.min_age_seconds()),
            ),
            None => "Max evolution reached — Starlight Elder 🌟".to_string(),
        }
    }

    pub fn advisor_tip(&self, now: DateTime<Utc>) -> String {
        if self.stage == EvolutionStage::Egg {
            let rem = self.warm_cooldown_remaining(now);
            if self.incubation >= 99 {
                return "🥚 Ready to hatch! Press [W] or [Space] now!".to_string();
            } else if rem == 0 {
                return "🤲 Press [W] or [Space] to warm the egg (+20%).".to_string();
            } else {
                return format!(
                    "🔥 Egg warming ({}%). Next [W] in {} (or [Y] Talk).",
                    self.incubation,
                    format_duration_human(rem)
                );
            }
        }

        if self.has_toothache {
            return format!("🦷 {} has a Toothache! Press [H] for Medicine!", self.name);
        }
        if self.is_sick {
            return format!("🤒 {} feels sick! Press [H] for Medicine!", self.name);
        }
        if self.is_sleeping && self.light_on {
            return format!(
                "💡 {} is asleep with Light ON! Press [K] to turn OFF!",
                self.name
            );
        }
        if self.attention_call == AttentionCall::Misbehaving {
            return format!("📢 {} is acting spoiled! Press [D] to Scold!", self.name);
        }
        if self.attention_call == AttentionCall::Crying {
            return format!("🥺 {} is crying! Press [T] to Praise & Pat!", self.name);
        }
        if self.poop_count > 0 || self.cleanliness < 45 {
            return format!(
                "🚽 Mess on floor (Poop {})! Press [C] for a Bath!",
                self.poop_count
            );
        }
        if self.is_sleeping {
            if self.energy >= 95 {
                return format!("🌞 {} is fully rested! Press [S] to wake up.", self.name);
            }
            return format!("🌙 {} is sleeping peacefully ({}% Energy).", self.name, self.energy);
        }
        if self.hungry_hearts() <= 1 {
            return "🍙 Hungry Hearts low! Press [F] Meal or [M] Pantry.".to_string();
        }
        if self.is_overweight() {
            return format!(
                "🏋 {} is chubby ({}g)! Play [P] or Arcade [G]!",
                self.name, self.weight_grams
            );
        }
        if self.energy <= 25 {
            return "🥱 Getting sleepy! Press [S] to sleep, then [K] Light OFF.".to_string();
        }
        if self.happy_hearts() <= 1 {
            return "🎾 Happy Hearts low! Press [P] Play or [Y] Talk.".to_string();
        }
        "🌟 Cozy! Press [Y] to Talk, [G] for 3-in-1 Arcade, [I] Meter, or [?] Keys.".to_string()
    }

    pub fn perform_action(&mut self, action: PetAction) -> String {
        match action {
            PetAction::WarmEgg => self.warm_egg(),
            PetAction::Feed => self.feed(),
            PetAction::FeedSnack => self.feed_choice(FoodChoice::WarmMilkTea),
            PetAction::Play => self.play(),
            PetAction::PetLove => self.pet_love(),
            PetAction::Talk => self.answer_dialogue(1),
            PetAction::Train => self.train(),
            PetAction::ToggleSleep => self.toggle_sleep(),
            PetAction::ToggleLight => self.toggle_light(),
            PetAction::DisciplineScold => self.discipline_scold(),
            PetAction::Clean => self.clean(),
            PetAction::Heal => self.heal(),
        }
    }

    pub fn perform(&mut self, action: PetAction) -> String {
        self.perform_action(action)
    }

    pub fn apply_action(&mut self, action: PetAction) -> String {
        self.perform_action(action)
    }

    fn egg_guard(&self) -> Option<String> {
        if self.stage == EvolutionStage::Egg {
            Some("🥚 The egg is still unhatched! Press [W] to warm & incubate it.".to_string())
        } else {
            None
        }
    }

    fn refuse(&mut self, msg: String) -> String {
        self.log_action(msg.clone());
        msg
    }

    pub fn warm_egg(&mut self) -> String {
        if self.stage != EvolutionStage::Egg {
            return self.pet_love();
        }

        let now = Utc::now();
        let remaining = self.warm_cooldown_remaining(now);
        if remaining > 0 {
            return self.refuse(format!(
                "⏳ The egg is still absorbing your warmth! Wait {} (passive incubation continues).",
                format_duration_human(remaining)
            ));
        }

        self.last_warmed_at = Some(now);
        self.last_updated = now;
        self.incubation = (self.incubation + 20).min(100);
        self.bond = (self.bond + 4).min(100);

        if self.incubation >= 100 {
            self.stage = EvolutionStage::Baby;
            self.weight_grams = EvolutionStage::Baby.base_weight_grams();
            self.xp = self.xp.max(EvolutionStage::Baby.min_xp());
            self.level = 2;
            self.hunger = 70;
            self.happiness = 88;
            self.energy = 92;
            self.mood = PetMood::Happy;
            self.coins = self.coins.saturating_add(20);
            let msg = format!(
                "🐣 CRACK! {} hatched into a precious {} ({}, {}g)! (+20 coins)",
                self.name,
                self.species.display_name(),
                self.stage.badge(),
                self.weight_grams
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.mood = PetMood::Happy;
            let msg = format!(
                "🤲 You gently warm {}'s egg... Incubation at {}% (+4 Bond)",
                self.name, self.incubation
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn toggle_light(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        self.light_on = !self.light_on;
        self.last_updated = Utc::now();

        let msg = if !self.light_on {
            if self.is_sleeping {
                self.bond = (self.bond + 3).min(100);
                self.add_xp(5);
                format!(
                    "🌙 Switched room light OFF. {} sleeps soundly in the cozy dark! (+3 Bond, +5 XP)",
                    self.name
                )
            } else {
                format!(
                    "🌙 Switched room light OFF while {} is awake (dim sanctuary mode).",
                    self.name
                )
            }
        } else {
            "💡 Switched room light ON! Bright and sunny in the habitat.".to_string()
        };
        self.refresh_mood();
        self.log_action(msg.clone());
        msg
    }

    pub fn discipline_scold(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!("💤 {} is sleeping — don't scold them in bed!", self.name));
        }

        self.last_updated = Utc::now();
        if self.attention_call == AttentionCall::Misbehaving {
            self.discipline = (self.discipline + 25).min(100);
            self.bond = (self.bond + 3).min(100);
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
            self.add_xp(12);
            self.refresh_mood();
            let msg = format!(
                "📢 Scolded {} for the fake attention call! Discipline rose to {}% (+12 XP)!",
                self.name, self.discipline
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.happiness = self.happiness.saturating_sub(8);
            self.bond = self.bond.saturating_sub(2);
            self.refresh_mood();
            let msg = format!(
                "🥺 {} wasn't misbehaving! Unfair scold lowered Happiness (-8%). Only scold during a Spoiled Call!",
                self.name
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn feed(&mut self) -> String {
        self.feed_choice(FoodChoice::FavoriteSnack)
    }

    pub fn feed_choice(&mut self, choice: FoodChoice) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!(
                "💤 {} is fast asleep! Wake them up with [S] first.",
                self.name
            ));
        }
        if self.has_toothache {
            return self.refuse(format!(
                "🦷 {} has a Toothache and refuses to eat! Give Medicine [H] first!",
                self.name
            ));
        }

        let is_snack = choice.is_sweet_snack();
        if !is_snack && self.hunger >= 95 {
            return self.refuse(format!(
                "🍱 {} shakes their head: tummy is already completely full (4/4 ♥)!",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.feed_cooldown_remaining(now);
        if !is_snack && remaining > 0 && self.hunger >= 60 {
            return self.refuse(format!(
                "⏳ {} is still digesting their last meal! Wait {}.",
                self.name,
                format_duration_human(remaining)
            ));
        }

        let cost = choice.cost();
        if self.coins < cost {
            return self.refuse(format!(
                "💰 Not enough coins for {} (need {} coins, have {}). Try a Free Meal/Snack!",
                choice.title(self.species),
                cost,
                self.coins
            ));
        }
        self.coins -= cost;

        let (hunger_gain, happy_gain, energy_gain, health_gain, bond_gain, xp_gain, weight_gain) =
            match choice {
                FoodChoice::FavoriteSnack => (25, 6, 0, 0, 3, 10, 1),
                FoodChoice::HeartyBento => (40, 8, 5, 2, 3, 14, 2),
                FoodChoice::WarmMilkTea => (5, 25, 8, 0, 4, 10, 2),
                FoodChoice::HerbalSoup => {
                    self.is_sick = false;
                    (25, 5, 4, 20, 3, 14, 1)
                }
                FoodChoice::StarlightDelight => (15, 35, 10, 5, 8, 25, 3),
            };

        self.hunger = (self.hunger + hunger_gain).min(100);
        self.happiness = (self.happiness + happy_gain).min(100);
        self.energy = (self.energy + energy_gain).min(100);
        self.health = (self.health + health_gain).min(100);
        self.bond = (self.bond + bond_gain).min(100);
        self.weight_grams = (self.weight_grams + weight_gain).min(99);

        if is_snack {
            self.snacks_in_a_row = self.snacks_in_a_row.saturating_add(1);
        } else {
            self.snacks_in_a_row = 0;
        }

        self.meals_since_poop = self.meals_since_poop.saturating_add(1);
        if self.meals_since_poop >= 2 {
            self.meals_since_poop = 0;
            self.poop_count = (self.poop_count + 1).min(5);
            self.cleanliness = self.cleanliness.saturating_sub(15);
            if self.poop_count >= 4 {
                self.is_sick = true;
            }
        }

        self.mood = PetMood::Eating;
        self.last_fed_at = Some(now);
        self.last_updated = now;
        self.add_xp(xp_gain);

        let food_name = choice.title(self.species);
        let mut msg = format!(
            "🍡 Fed {} {}! (+{}g weight -> {}g, +{} XP)",
            self.name, food_name, weight_gain, self.weight_grams, xp_gain
        );

        if self.snacks_in_a_row >= 3 {
            self.has_toothache = true;
            self.mood = PetMood::Sick;
            msg.push_str(
                " — 🦷 OH NO! Too many sweets caused a Toothache! Press [H] for Medicine!",
            );
        }

        self.refresh_attention();
        self.log_action(msg.clone());
        msg
    }

    pub fn play(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!("💤 {} is sleeping! Wake them first.", self.name));
        }
        if self.is_sick || self.has_toothache {
            return self.refuse(format!(
                "🤒 {} feels sick and needs Medicine [H] before playing!",
                self.name
            ));
        }
        if self.energy < 20 {
            return self.refuse(format!(
                "⚡ {} is too tired to play! Let them rest [S].",
                self.name
            ));
        }
        if self.hunger < 20 {
            return self.refuse(format!(
                "🍙 {} is too hungry to play! Feed them [F] first.",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.play_cooldown_remaining(now);
        if remaining > 0 && self.happiness >= 75 {
            return self.refuse(format!(
                "⏳ {} just finished playing! Let them catch their breath ({}).",
                self.name,
                format_duration_human(remaining)
            ));
        }

        self.happiness = (self.happiness + 18).min(100);
        self.energy = self.energy.saturating_sub(14);
        self.hunger = self.hunger.saturating_sub(9);
        self.cleanliness = self.cleanliness.saturating_sub(5);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(5);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(2).max(base_w);
        self.mood = PetMood::Happy;
        self.last_played_at = Some(now);
        self.last_updated = now;
        self.add_xp(15);
        self.refresh_attention();

        let msg = format!(
            "🎾 Played Left/Right Game with {}! (+18 Happy, Weight {}g, +5 coins, +15 XP)",
            self.name, self.weight_grams
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn pet_love(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!("💤 You watch {} sleep peacefully.", self.name));
        }

        let now = Utc::now();
        if self.attention_call == AttentionCall::Crying {
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
            self.discipline = (self.discipline + 15).min(100);
            self.happiness = (self.happiness + 14).min(100);
            self.bond = (self.bond + 6).min(100);
            self.last_petted_at = Some(now);
            self.last_updated = now;
            self.mood = PetMood::Happy;
            self.add_xp(10);
            let msg = format!(
                "💖 Praised & comforted crying {}! (+15% Discipline, +14 Happy, +10 XP)",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }

        let remaining = self.pet_cooldown_remaining(now);
        if remaining > 0 {
            return self.refuse(format!(
                "💕 {} is still basking in your headpats! Wait {}s.",
                self.name, remaining
            ));
        }
        if self.happiness >= 98 {
            self.happiness = 100;
            self.mood = PetMood::Happy;
            return self.refuse(format!(
                "💖 {} purrs happily! Happiness is already maxed out.",
                self.name
            ));
        }

        self.last_petted_at = Some(now);
        self.last_updated = now;
        self.happiness = (self.happiness + 8).min(100);
        self.bond = (self.bond + 5).min(100);
        self.mood = PetMood::Happy;
        self.add_xp(6);

        let msg = format!(
            "💖 Gave {} warm cuddles and headpats! (+8 Happy, +5 Bond, +6 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn train(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!(
                "💤 {} is asleep and cannot train right now.",
                self.name
            ));
        }
        if self.is_sick || self.has_toothache {
            return self.refuse(format!(
                "🤒 {} is unwell! Heal them [H] before training.",
                self.name
            ));
        }
        if self.energy < 25 {
            return self.refuse(format!(
                "⚡ {} lacks energy to train! Need 25+ Energy.",
                self.name
            ));
        }
        if self.hunger < 20 {
            return self.refuse(format!(
                "🍱 {} is too hungry to focus on training!",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.train_cooldown_remaining(now);
        if remaining > 0 {
            return self.refuse(format!(
                "⏳ {} is catching their breath after training! Wait {}.",
                self.name,
                format_duration_human(remaining)
            ));
        }

        self.last_trained_at = Some(now);
        self.energy = self.energy.saturating_sub(20);
        self.hunger = self.hunger.saturating_sub(12);
        self.happiness = (self.happiness + 8).min(100);
        self.discipline = (self.discipline + 8).min(100);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(10);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(2).max(base_w);
        self.mood = PetMood::Training;
        self.last_updated = now;
        self.add_xp(22);

        let msg = format!(
            "🥋 {} finished a focus training session! (Weight {}g, +8% Discipline, +22 XP, +10 coins)",
            self.name, self.weight_grams
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn toggle_sleep(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        self.is_sleeping = !self.is_sleeping;
        self.last_updated = Utc::now();

        if self.is_sleeping {
            self.mood = PetMood::Sleeping;
            self.refresh_attention();
            let light_hint = if self.light_on {
                " (Don't forget to turn the Light OFF with [K]!)"
            } else {
                ""
            };
            let msg = format!(
                "🌙 Tucked {} into a cozy blanket. Goodnight!{}",
                self.name, light_hint
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.light_on = true;
            self.energy = (self.energy + 15).min(100);
            self.refresh_mood();
            let msg = format!(
                "🌞 {} woke up refreshed and turned the room light ON! (+15 Energy)",
                self.name
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn clean(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.cleanliness >= 100 && self.poop_count == 0 {
            return self.refuse("🌟 The habitat floor is already sparkling clean!".to_string());
        }

        self.poop_count = 0;
        self.meals_since_poop = 0;
        self.snacks_in_a_row = 0;
        self.cleanliness = 100;
        self.happiness = (self.happiness + 6).min(100);
        self.health = (self.health + 5).min(100);
        self.bond = (self.bond + 2).min(100);
        self.last_updated = Utc::now();
        self.add_xp(8);
        self.refresh_mood();

        let msg = format!(
            "🚽 Flushed the mess, brushed teeth & gave {} a warm bubble bath! (+8 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn heal(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if !self.is_sick && !self.has_toothache && self.health >= 95 {
            return self.refuse(format!("💚 {} is already in tip-top health!", self.name));
        }

        let was_toothache = self.has_toothache;
        self.is_sick = false;
        self.has_toothache = false;
        self.snacks_in_a_row = 0;
        self.health = 100;
        self.happiness = (self.happiness + 10).min(100);
        self.bond = (self.bond + 3).min(100);
        self.last_updated = Utc::now();
        self.add_xp(10);
        self.refresh_mood();

        let msg = if was_toothache {
            format!(
                "💉 Treated {}'s Toothache and restored full health! (+10 XP)",
                self.name
            )
        } else {
            format!(
                "💉 Gave {} herbal medicine. Sickness cured! (+10 XP)",
                self.name
            )
        };
        self.log_action(msg.clone());
        msg
    }

    pub fn apply_minigame_reward(&mut self, score: u32, coins_earned: u32, xp_earned: u32) -> String {
        self.coins = self.coins.saturating_add(coins_earned);
        self.happiness = (self.happiness + 12).min(100);
        self.bond = (self.bond + 3).min(100);
        self.energy = self.energy.saturating_sub(5);
        let base_w = self.stage.base_weight_grams();
        self.weight_grams = self.weight_grams.saturating_sub(1).max(base_w);
        self.add_xp(xp_earned);
        self.refresh_mood();

        let msg = format!(
            "🎮 Arcade scored {}! Earned +{} coins, +{} XP & burned -1g weight!",
            score, coins_earned, xp_earned
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn tick_decay(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();

        if self.stage == EvolutionStage::Egg {
            self.incubation = (self.incubation + 1).min(99);
            return;
        }

        if self.is_sleeping {
            if self.light_on {
                self.energy = (self.energy + 2).min(100);
                self.happiness = self.happiness.saturating_sub(1);
                if self.age_seconds.is_multiple_of(30) {
                    self.care_mistakes = self.care_mistakes.saturating_add(1);
                }
            } else {
                self.energy = (self.energy + 5).min(100);
            }
            self.hunger = self.hunger.saturating_sub(1);
        } else {
            let weight_penalty = u8::from(self.is_overweight());
            self.hunger = self.hunger.saturating_sub(2);
            self.happiness = self.happiness.saturating_sub(1);
            self.energy = self.energy.saturating_sub(1 + weight_penalty);

            if self.attention_call == AttentionCall::None
                && self.discipline < 100
                && self.age_seconds.is_multiple_of(40)
            {
                if self.age_seconds.is_multiple_of(80) {
                    self.attention_call = AttentionCall::Misbehaving;
                    self.attention_since = Some(Utc::now());
                    self.log_action(format!(
                        "🔔 {} is beeping for attention just to be mischievous! Press [D] to Scold!",
                        self.name
                    ));
                } else {
                    self.attention_call = AttentionCall::Crying;
                    self.attention_since = Some(Utc::now());
                    self.log_action(format!(
                        "🔔 {} is sniffling and calling for you! Press [T] to Praise & Pat!",
                        self.name
                    ));
                }
            }
        }

        if self.poop_count > 0 {
            self.cleanliness = self.cleanliness.saturating_sub(self.poop_count * 2);
            if self.poop_count >= 4 {
                self.is_sick = true;
            }
        }
        if self.hunger <= 15 || self.cleanliness <= 25 {
            self.health = self.health.saturating_sub(3);
            self.care_mistakes = self.care_mistakes.saturating_add(1);
        }
        if self.health <= 35 {
            self.is_sick = true;
        }

        if let Some(since) = self.attention_since {
            if Utc::now().signed_duration_since(since).num_minutes() >= 15 {
                self.care_mistakes = self.care_mistakes.saturating_add(1);
                self.attention_since = Some(Utc::now());
            }
        }

        self.check_evolution();
        self.refresh_mood();
    }

    pub fn tick_standby(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();
        if self.stage == EvolutionStage::Egg {
            self.incubation = (self.incubation + 1).min(99);
            return;
        }
        if self.age_seconds.is_multiple_of(2) {
            if self.is_sleeping {
                self.energy = (self.energy + 4).min(100);
            } else {
                self.hunger = self.hunger.saturating_sub(1).max(20);
                self.happiness = self.happiness.saturating_sub(1).max(25);
                self.energy = self.energy.saturating_sub(1).max(25);
            }
        }
        self.check_evolution();
        self.refresh_mood();
    }

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
                    "🐣 The shell is trembling! Press [W] once to hatch your companion!".to_string(),
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
                    ten_min_blocks.saturating_add(ten_min_blocks / 5).min(85),
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
            self.hunger = self.hunger.saturating_sub(hunger_drop).max(5);
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
            self.hunger = self.hunger.saturating_sub(hunger_drop).max(5);
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
            let dmg = ((minutes / 30) as u8).clamp(5, 35);
            self.health = self.health.saturating_sub(dmg).max(20);
            self.care_mistakes = self.care_mistakes.saturating_add(1);
            if self.health <= 35 || self.poop_count >= 4 {
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

    pub fn apply_offline_decay(&mut self, now: DateTime<Utc>) -> u64 {
        self.apply_offline_decay_with_report(now).0
    }

    pub fn status_summary(&self) -> String {
        format!(
            "{} — {} {} | Stage: {} | Lv.{} ({} XP) | Rank: {} | Weight: {}g | Hungry: {} ({}%) • Happy: {} ({}%) • Discipline: {}% • Energy: {}% • Health: {}% • Clean: {}% • Bond: {}% | Mistakes: {} | Coins: {}",
            self.name,
            self.species.display_name(),
            self.species.rarity(),
            self.stage.badge(),
            self.level,
            self.xp,
            self.care_rank().badge(),
            self.weight_grams,
            format_hearts(self.hungry_hearts()),
            self.hunger,
            format_hearts(self.happy_hearts()),
            self.happiness,
            self.discipline,
            self.energy,
            self.health,
            self.cleanliness,
            self.bond,
            self.care_mistakes,
            self.coins
        )
    }
}

/// Your whole collection of companions, all living under one roof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sanctuary {
    pub pets: Vec<Pet>,
    pub active_index: usize,
    pub sanctuary_level: u32,
}

impl Default for Sanctuary {
    fn default() -> Self {
        Self::new_with_pet(Pet::default())
    }
}

impl Sanctuary {
    pub fn new_with_pet(pet: Pet) -> Self {
        Self {
            pets: vec![pet],
            active_index: 0,
            sanctuary_level: 1,
        }
    }

    fn refresh_level(&mut self) {
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

    pub fn add_pet(&mut self, pet: Pet) -> usize {
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
