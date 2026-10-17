//! Core pet model for **Mochigotchi** — collectible species, lifecycle stages,
//! realistic incubation/care cooldowns, and the multi-pet `Sanctuary` roster.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Seconds you must wait between two manual egg-warming sessions.
pub const WARM_COOLDOWN_SECS: i64 = 12;
/// Seconds you must wait between two training sessions.
pub const TRAIN_COOLDOWN_SECS: i64 = 15;
/// Seconds you must wait between two head-pat (cuddle) sessions.
pub const PET_COOLDOWN_SECS: i64 = 8;

/// Life stages of every companion. Eggs hatch, then grow through three
/// care-gated stages before reaching the Starlight Elder form.
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
            Self::CyberElder => "🌟 Starlight Elder",
        }
    }

    /// Total XP required to *reach* this stage.
    pub fn min_xp(&self) -> u32 {
        match self {
            Self::Egg => 0,
            Self::Baby => 80,
            Self::Teen => 250,
            Self::Adult => 600,
            Self::CyberElder => 1200,
        }
    }

    /// Bond required to *reach* this stage.
    pub fn min_bond(&self) -> u8 {
        match self {
            Self::Egg => 0,
            Self::Baby => 10,
            Self::Teen => 25,
            Self::Adult => 45,
            Self::CyberElder => 70,
        }
    }

    /// Time you must have spent together (in seconds) to *reach* this stage.
    pub fn min_age_seconds(&self) -> u64 {
        match self {
            Self::Egg => 0,
            Self::Baby => 0,
            Self::Teen => 60,
            Self::Adult => 180,
            Self::CyberElder => 420,
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
        if xp >= 1200 {
            Self::CyberElder
        } else if xp >= 600 {
            Self::Adult
        } else if xp >= 250 {
            Self::Teen
        } else if xp >= 80 {
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
            Self::PhoenixChick => "🐦‍🔥 Sol Phoenix",
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

    pub fn rarity(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun => "Common ★",
            Self::StarPengu | Self::SlimeKing => "Rare ★★",
            Self::EmberFox | Self::CrystalUni => "Epic ★★★",
            Self::AzureWyrm => "Mythic ★★★★",
            Self::IgnisDragon | Self::PhoenixChick => "Legendary ★★★★★",
        }
    }

    pub fn element_badge(&self) -> &'static str {
        match self {
            Self::MochiCat => "🍡 Sweet",
            Self::BobaPup => "🧋 Tapioca",
            Self::CloudBun => "☁️ Sky",
            Self::StarPengu => "❄️ Aurora",
            Self::IgnisDragon => "🔥 Dragonfire",
            Self::AzureWyrm => "🌊 Celestial Tide",
            Self::EmberFox => "🦊 Spirit Foxfire",
            Self::PhoenixChick => "☀️ Sunburst",
            Self::CrystalUni => "✨ Starlight",
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
            Self::IgnisDragon => "Flame Chili 🌶️",
            Self::AzureWyrm => "Moon Pearl 🫧",
            Self::EmberFox => "Inari Tofu 🦊",
            Self::PhoenixChick => "Sunfire Berry ☀️",
            Self::CrystalUni => "Starlight Sugar ✨",
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
            Self::SkyBlue => "☁️ Sky Blue",
            Self::LavenderDream => "💜 Lavender Dream",
            Self::SunsetGold => "✨ Sunset Gold",
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
            Self::Idle => "🌤️",
            Self::Eating => "🍡",
            Self::Sleeping => "💤",
            Self::Happy => "💖",
            Self::Sick => "🤒",
            Self::Training => "💪",
        }
    }
}

/// Every care action the player can trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetAction {
    WarmEgg,
    Feed,
    Play,
    PetLove,
    Train,
    ToggleSleep,
    Clean,
    Heal,
}

impl PetAction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::WarmEgg => "Warm / Cuddle",
            Self::Feed => "Feed",
            Self::Play => "Play",
            Self::PetLove => "Headpat",
            Self::Train => "Train",
            Self::ToggleSleep => "Sleep",
            Self::Clean => "Clean",
            Self::Heal => "Heal",
        }
    }
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
    pub poop_count: u8,
    pub meals_since_poop: u8,
    pub bond: u8,
    pub xp: u32,
    pub level: u32,
    pub coins: u32,
    pub is_sleeping: bool,
    pub is_sick: bool,
    pub last_updated: DateTime<Utc>,
    pub last_warmed_at: Option<DateTime<Utc>>,
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
            poop_count: 0,
            meals_since_poop: 0,
            bond: 10,
            xp: 0,
            level: 1,
            coins: 0,
            is_sleeping: false,
            is_sick: false,
            last_updated: Utc::now(),
            last_warmed_at: None,
            last_trained_at: None,
            last_petted_at: None,
            age_seconds: 0,
            action_log: vec![format!(
                "🥚 A mysterious {} egg arrived in your sanctuary!",
                species.display_name()
            )],
        }
    }

    pub fn rename(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        if !trimmed.is_empty() {
            self.name = trimmed.to_string();
            self.log_action(format!("🏷️ Renamed to {}!", self.name));
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

    pub fn train_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_trained_at, TRAIN_COOLDOWN_SECS, now)
    }

    pub fn pet_cooldown_remaining(&self, now: DateTime<Utc>) -> u64 {
        Self::remaining_secs(self.last_petted_at, PET_COOLDOWN_SECS, now)
    }

    pub fn refresh_mood(&mut self) {
        if self.is_sick || self.health <= 35 {
            self.is_sick = true;
            self.mood = PetMood::Sick;
        } else if self.is_sleeping {
            self.mood = PetMood::Sleeping;
        } else if self.happiness >= 80 && self.hunger >= 50 {
            self.mood = PetMood::Happy;
        } else {
            self.mood = PetMood::Idle;
        }
    }

    pub fn update_mood(&mut self) {
        self.refresh_mood();
    }

    pub fn level_up_threshold(&self) -> u32 {
        80
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
                self.coins = self.coins.saturating_add(20);
                self.happiness = (self.happiness + 10).min(100);
                self.health = (self.health + 10).min(100);
                self.log_action(format!(
                    "✨ Evolution! {} became a {}! (+20 coins)",
                    self.name,
                    self.stage.badge()
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
                "Hatch at 100% Incubation (now {}% • warm [W] or wait)",
                self.incubation
            );
        }
        match self.stage.next_stage() {
            Some(next) => format!(
                "Next: {} • XP {}/{} • Bond {}/{} • Age {}s/{}s",
                next.badge(),
                self.xp,
                next.min_xp(),
                self.bond,
                next.min_bond(),
                self.age_seconds,
                next.min_age_seconds()
            ),
            None => "Max evolution reached — Starlight Elder 🌟".to_string(),
        }
    }

    pub fn perform_action(&mut self, action: PetAction) -> String {
        match action {
            PetAction::WarmEgg => self.warm_egg(),
            PetAction::Feed => self.feed(),
            PetAction::Play => self.play(),
            PetAction::PetLove => self.pet_love(),
            PetAction::Train => self.train(),
            PetAction::ToggleSleep => self.toggle_sleep(),
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
                "⏳ The egg is still absorbing your warmth! Wait {}s (passive incubation continues).",
                remaining
            ));
        }

        self.last_warmed_at = Some(now);
        self.last_updated = now;
        self.incubation = (self.incubation + 20).min(100);
        self.bond = (self.bond + 4).min(100);

        if self.incubation >= 100 {
            self.stage = EvolutionStage::Baby;
            self.xp = self.xp.max(EvolutionStage::Baby.min_xp());
            self.level = 2;
            self.hunger = 65;
            self.happiness = 85;
            self.energy = 90;
            self.mood = PetMood::Happy;
            self.coins = self.coins.saturating_add(15);
            let msg = format!(
                "🐣 CRACK! {} hatched into a precious {} ({})! (+15 coins)",
                self.name,
                self.species.display_name(),
                self.stage.badge()
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

    pub fn feed(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.is_sleeping {
            return self.refuse(format!(
                "💤 {} is fast asleep! Wake them up with [S] first.",
                self.name
            ));
        }
        if self.hunger >= 95 {
            return self.refuse(format!(
                "🍽️ {}'s tummy is already completely full!",
                self.name
            ));
        }

        self.hunger = (self.hunger + 22).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.bond = (self.bond + 2).min(100);
        self.meals_since_poop = self.meals_since_poop.saturating_add(1);
        if self.meals_since_poop >= 2 {
            self.meals_since_poop = 0;
            self.poop_count = (self.poop_count + 1).min(5);
            self.cleanliness = self.cleanliness.saturating_sub(15);
        }
        self.mood = PetMood::Eating;
        self.last_updated = Utc::now();
        self.add_xp(10);

        let msg = format!(
            "🍡 Fed {} their favorite {}! (+22 Fullness, +10 XP)",
            self.name,
            self.species.favorite_snack()
        );
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
        if self.is_sick {
            return self.refuse(format!(
                "🤒 {} feels sick and needs medicine [H] before playing!",
                self.name
            ));
        }
        if self.energy < 20 {
            return self.refuse(format!("⚡ {} is too tired to play! Let them rest [S].", self.name));
        }
        if self.hunger < 20 {
            return self.refuse(format!("🍙 {} is too hungry to play! Feed them [F] first.", self.name));
        }

        self.happiness = (self.happiness + 18).min(100);
        self.energy = self.energy.saturating_sub(15);
        self.hunger = self.hunger.saturating_sub(10);
        self.cleanliness = self.cleanliness.saturating_sub(5);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(5);
        self.mood = PetMood::Happy;
        self.last_updated = Utc::now();
        self.add_xp(15);

        let msg = format!(
            "🎾 Played joyfully with {}! (+18 Happy, +5 coins, +15 XP)",
            self.name
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
            return self.refuse(format!("💤 {} is asleep and cannot train right now.", self.name));
        }
        if self.is_sick {
            return self.refuse(format!("🤒 {} is unwell! Heal them [H] before training.", self.name));
        }
        if self.energy < 25 {
            return self.refuse(format!("⚡ {} lacks energy to train! Need 25+ Energy.", self.name));
        }
        if self.hunger < 20 {
            return self.refuse(format!("🍽️ {} is too hungry to focus on training!", self.name));
        }

        let now = Utc::now();
        let remaining = self.train_cooldown_remaining(now);
        if remaining > 0 {
            return self.refuse(format!(
                "⏳ {} is catching their breath after training! Wait {}s.",
                self.name, remaining
            ));
        }

        self.last_trained_at = Some(now);
        self.energy = self.energy.saturating_sub(20);
        self.hunger = self.hunger.saturating_sub(12);
        self.happiness = (self.happiness + 8).min(100);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(10);
        self.mood = PetMood::Training;
        self.last_updated = now;
        self.add_xp(22);

        let msg = format!(
            "🥋 {} finished a focus training session! (+22 XP, +10 coins)",
            self.name
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
            let msg = format!("🌙 Tucked {} into a cozy blanket. Goodnight!", self.name);
            self.log_action(msg.clone());
            msg
        } else {
            self.energy = (self.energy + 15).min(100);
            self.refresh_mood();
            let msg = format!("☀️ {} woke up refreshed! (+15 Energy)", self.name);
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn clean(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if self.cleanliness >= 100 && self.poop_count == 0 {
            return self.refuse("✨ The habitat is already sparkling clean!".to_string());
        }

        self.poop_count = 0;
        self.meals_since_poop = 0;
        self.cleanliness = 100;
        self.happiness = (self.happiness + 6).min(100);
        self.health = (self.health + 5).min(100);
        self.bond = (self.bond + 2).min(100);
        self.last_updated = Utc::now();
        self.add_xp(8);
        self.refresh_mood();

        let msg = format!("🫧 Gave {} a warm bubble bath and tidied up! (+8 XP)", self.name);
        self.log_action(msg.clone());
        msg
    }

    pub fn heal(&mut self) -> String {
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        if !self.is_sick && self.health >= 95 {
            return self.refuse(format!("💚 {} is already in tip-top health!", self.name));
        }

        self.is_sick = false;
        self.health = 100;
        self.happiness = (self.happiness + 10).min(100);
        self.bond = (self.bond + 3).min(100);
        self.last_updated = Utc::now();
        self.add_xp(10);
        self.refresh_mood();

        let msg = format!("💊 Gave {} sweet herbal medicine. Fully recovered! (+10 XP)", self.name);
        self.log_action(msg.clone());
        msg
    }

    pub fn apply_minigame_reward(&mut self, score: u32, coins_earned: u32, xp_earned: u32) -> String {
        self.coins = self.coins.saturating_add(coins_earned);
        self.happiness = (self.happiness + 12).min(100);
        self.bond = (self.bond + 3).min(100);
        self.energy = self.energy.saturating_sub(5);
        self.add_xp(xp_earned);
        self.refresh_mood();

        let msg = format!(
            "🎮 Star Catcher scored {}! Earned +{} coins & +{} XP!",
            score, coins_earned, xp_earned
        );
        self.log_action(msg.clone());
        msg
    }

    /// One in-session heartbeat (the TUI ticks every ~5s).
    pub fn tick_decay(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();

        if self.stage == EvolutionStage::Egg {
            self.incubation = (self.incubation + 1).min(99);
            return;
        }

        if self.is_sleeping {
            self.energy = (self.energy + 6).min(100);
            self.hunger = self.hunger.saturating_sub(1);
        } else {
            self.hunger = self.hunger.saturating_sub(2);
            self.happiness = self.happiness.saturating_sub(1);
            self.energy = self.energy.saturating_sub(1);
        }

        if self.poop_count > 0 {
            self.cleanliness = self.cleanliness.saturating_sub(self.poop_count * 2);
        }
        if self.hunger <= 15 || self.cleanliness <= 25 {
            self.health = self.health.saturating_sub(3);
        }
        if self.health <= 35 {
            self.is_sick = true;
        }

        self.check_evolution();
        self.refresh_mood();
    }

    /// Apply everything that happened while you were away.
    /// Eggs only absorb *passive* warmth (up to 99%) and never starve.
    pub fn apply_offline_decay(&mut self, now: DateTime<Utc>) -> u64 {
        let elapsed = now
            .signed_duration_since(self.last_updated)
            .num_seconds()
            .max(0) as u64;
        if elapsed == 0 {
            return 0;
        }

        self.last_updated = now;
        self.age_seconds = self.age_seconds.saturating_add(elapsed);

        if self.stage == EvolutionStage::Egg {
            let passive = ((elapsed / 15) as u8).min(99);
            self.incubation = self.incubation.saturating_add(passive).min(99);
            return elapsed;
        }

        let intervals = (elapsed / 60).min(48) as u8;
        if intervals == 0 {
            return elapsed;
        }

        if self.is_sleeping {
            self.energy = self.energy.saturating_add(intervals.saturating_mul(5)).min(100);
            self.hunger = self.hunger.saturating_sub(intervals);
        } else {
            self.hunger = self.hunger.saturating_sub(intervals.saturating_mul(2));
            self.happiness = self.happiness.saturating_sub(intervals);
            self.energy = self.energy.saturating_sub(intervals);
            self.cleanliness = self.cleanliness.saturating_sub(intervals);
        }

        if self.hunger <= 15 || self.cleanliness <= 20 {
            self.health = self.health.saturating_sub(intervals).max(20);
            if self.health <= 35 {
                self.is_sick = true;
            }
        }

        self.check_evolution();
        self.refresh_mood();
        elapsed
    }

    pub fn status_summary(&self) -> String {
        format!(
            "{} — {} {} | Stage: {} | Lv.{} ({} XP) | Hunger {}% • Happy {}% • Energy {}% • Health {}% • Clean {}% • Bond {}% | Coins {}",
            self.name,
            self.species.display_name(),
            self.species.rarity(),
            self.stage.badge(),
            self.level,
            self.xp,
            self.hunger,
            self.happiness,
            self.energy,
            self.health,
            self.cleanliness,
            self.bond,
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
        self.pets
            .iter_mut()
            .map(|p| p.apply_offline_decay(now))
            .max()
            .unwrap_or(0)
    }

    pub fn tick_decay_all(&mut self) {
        for pet in &mut self.pets {
            pet.tick_decay();
        }
    }

    pub fn hatched_count(&self) -> usize {
        self.pets
            .iter()
            .filter(|p| p.stage != EvolutionStage::Egg)
            .count()
    }
}
