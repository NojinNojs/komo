use chrono::{DateTime, Utc};
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

    pub fn min_xp(&self) -> u32 {
        match self {
            Self::Egg => 0,
            Self::Baby => 50,
            Self::Teen => 150,
            Self::Adult => 350,
            Self::CyberElder => 700,
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
        if xp >= 700 {
            Self::CyberElder
        } else if xp >= 350 {
            Self::Adult
        } else if xp >= 150 {
            Self::Teen
        } else if xp >= 50 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PetSpecies {
    #[default]
    MochiCat,
    BobaPup,
    CloudBun,
    StarPengu,
}

impl PetSpecies {
    pub fn all() -> &'static [PetSpecies] {
        &[
            Self::MochiCat,
            Self::BobaPup,
            Self::CloudBun,
            Self::StarPengu,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "🐱 Neko Mochi",
            Self::BobaPup => "🐶 Boba Pup",
            Self::CloudBun => "🐰 Cloud Bun",
            Self::StarPengu => "🐧 Star Pengu",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::MochiCat => "A squishy rice-cake kitty who loves warm naps and crispy taiyaki.",
            Self::BobaPup => "A bouncy tapioca puppy overflowing with sweet milk-tea energy.",
            Self::CloudBun => "A weightless cotton-candy bunny that floats on daydreams.",
            Self::StarPengu => "A cosmic little penguin hatched from stardust and aurora ice.",
        }
    }

    pub fn default_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "Mochi",
            Self::BobaPup => "Boba",
            Self::CloudBun => "Cloudy",
            Self::StarPengu => "Pebble",
        }
    }

    pub fn favorite_snack(&self) -> &'static str {
        match self {
            Self::MochiCat => "Taiyaki Fish 🐟",
            Self::BobaPup => "Honey Biscuit 🦴",
            Self::CloudBun => "Sweet Carrot 🥕",
            Self::StarPengu => "Snowberry 🫐",
        }
    }
}

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
}

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
            Self::Idle => "Calm & Cozy",
            Self::Eating => "Munching Snack",
            Self::Sleeping => "Fast Asleep",
            Self::Happy => "Super Happy",
            Self::Sick => "Feeling Unwell",
            Self::Training => "Training Hard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            age_seconds: 0,
            action_log: vec![format!(
                "🥚 A mysterious {} egg appeared! Press [W] to warm it.",
                species.display_name()
            )],
        }
    }

    pub fn rename(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        if !trimmed.is_empty() {
            self.name = trimmed.to_string();
            self.log_action(format!("🏷️ Renamed pet to {}!", self.name));
        }
    }

    pub fn log_action(&mut self, msg: impl Into<String>) {
        self.action_log.push(msg.into());
        while self.action_log.len() > 20 {
            self.action_log.remove(0);
        }
    }

    pub fn refresh_mood(&mut self) {
        if self.is_sleeping {
            self.mood = PetMood::Sleeping;
        } else if self.is_sick || self.health < 35 {
            self.mood = PetMood::Sick;
        } else if self.happiness >= 80 && self.hunger >= 50 {
            self.mood = PetMood::Happy;
        } else {
            self.mood = PetMood::Idle;
        }
    }

    pub fn update_mood(&mut self) {
        self.refresh_mood();
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp = self.xp.saturating_add(amount);
        self.level = 1 + (self.xp / 50);
        self.check_evolution()
    }

    pub fn check_evolution(&mut self) -> bool {
        let target = EvolutionStage::from_xp(self.xp);
        if target > self.stage {
            if self.stage == EvolutionStage::Egg {
                self.incubation = 100;
            }
            self.stage = target;
            self.coins = self.coins.saturating_add(15);
            self.happiness = (self.happiness + 15).min(100);
            self.energy = (self.energy + 15).min(100);
            self.health = (self.health + 10).min(100);
            self.log_action(format!(
                "✨ Evolution! {} evolved into {}! (+15 coins)",
                self.name,
                self.stage.display_name()
            ));
            true
        } else {
            false
        }
    }

    pub fn xp_progress_ratio(&self) -> f64 {
        if self.stage == EvolutionStage::Egg {
            (self.incubation as f64 / 100.0).clamp(0.0, 1.0)
        } else if let Some(next) = self.stage.next_stage() {
            let current_min = self.stage.min_xp();
            let span = (next.min_xp() - current_min).max(1);
            let progress = self.xp.saturating_sub(current_min);
            (progress as f64 / span as f64).clamp(0.0, 1.0)
        } else {
            1.0
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

    fn check_egg_guard(&mut self) -> Option<String> {
        if self.stage == EvolutionStage::Egg {
            let msg =
                "🥚 The egg is still unhatched! Press [W] to warm & incubate it.".to_string();
            self.log_action(msg.clone());
            Some(msg)
        } else {
            None
        }
    }

    pub fn warm_egg(&mut self) -> String {
        if self.stage != EvolutionStage::Egg {
            return self.pet_love();
        }

        self.incubation = (self.incubation + 25).min(100);
        self.bond = (self.bond + 3).min(100);

        if self.incubation >= 100 {
            self.stage = EvolutionStage::Baby;
            self.xp = self.xp.max(50);
            self.level = 2;
            self.hunger = 65;
            self.happiness = 85;
            self.energy = 90;
            self.mood = PetMood::Happy;
            self.coins = self.coins.saturating_add(10);
            let msg = format!(
                "🐣 Crack! {} hatched into an adorable {}! (+10 coins)",
                self.name,
                self.species.display_name()
            );
            self.log_action(msg.clone());
            msg
        } else {
            let msg = format!(
                "🤲 You gently warm the egg... Incubation at {}%!",
                self.incubation
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn feed(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if self.is_sleeping {
            let msg = format!("💤 {} is fast asleep and can't eat right now.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.hunger >= 95 {
            let msg = format!("🍽️ {}'s tummy is already completely full!", self.name);
            self.log_action(msg.clone());
            return msg;
        }

        self.hunger = (self.hunger + 25).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.bond = (self.bond + 2).min(100);
        self.meals_since_poop = self.meals_since_poop.saturating_add(1);
        if self.meals_since_poop >= 2 {
            self.meals_since_poop = 0;
            self.poop_count = (self.poop_count + 1).min(5);
            self.cleanliness = self.cleanliness.saturating_sub(15);
        }
        self.mood = PetMood::Eating;
        self.add_xp(8);

        let msg = format!(
            "🍙 Fed {} a delicious {}! (+25 Fullness, +8 XP)",
            self.name,
            self.species.favorite_snack()
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn play(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if self.is_sleeping {
            let msg = format!("💤 {} is sleeping! Wake them up first.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.is_sick {
            let msg = format!(
                "🤒 {} is feeling sick and needs medicine [H] before playing!",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }
        if self.energy < 20 {
            let msg = format!("🥱 {} is too tired to play! Let them rest [S].", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.hunger < 20 {
            let msg = format!("🥺 {} is too hungry to play! Feed them [F] first.", self.name);
            self.log_action(msg.clone());
            return msg;
        }

        self.happiness = (self.happiness + 20).min(100);
        self.energy = self.energy.saturating_sub(15);
        self.hunger = self.hunger.saturating_sub(10);
        self.cleanliness = self.cleanliness.saturating_sub(5);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(5);
        self.mood = PetMood::Happy;
        self.add_xp(15);

        let msg = format!(
            "🎾 Played joyfully with {}! (+20 Happy, +15 XP, +5 coins)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn pet_love(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if self.is_sleeping {
            let msg = format!("💤 You watch {} sleep peacefully.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.happiness >= 98 {
            self.happiness = 100;
            self.mood = PetMood::Happy;
            let msg = format!(
                "💕 {} purrs happily! (Already at max happiness!)",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }

        self.happiness = (self.happiness + 10).min(100);
        self.bond = (self.bond + 5).min(100);
        self.mood = PetMood::Happy;
        self.add_xp(5);

        let msg = format!(
            "💖 Gave {} warm cuddles and headpats! (+10 Happy, +5 Bond, +5 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn train(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if self.is_sleeping {
            let msg = format!("💤 {} is asleep and cannot train right now.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.is_sick {
            let msg = format!("🤒 {} is unwell! Heal them [H] before training.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.energy < 25 {
            let msg = format!("🥱 {} lacks the energy to train! Need >=25 Energy.", self.name);
            self.log_action(msg.clone());
            return msg;
        }
        if self.hunger < 20 {
            let msg = format!("🍽️ {} is too hungry to focus on training!", self.name);
            self.log_action(msg.clone());
            return msg;
        }

        self.energy = self.energy.saturating_sub(20);
        self.hunger = self.hunger.saturating_sub(12);
        self.happiness = (self.happiness + 8).min(100);
        self.bond = (self.bond + 4).min(100);
        self.coins = self.coins.saturating_add(10);
        self.mood = PetMood::Training;
        self.add_xp(25);

        let msg = format!(
            "🥋 {} completed a focus training session! (+25 XP, +10 coins)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn toggle_sleep(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        self.is_sleeping = !self.is_sleeping;
        if self.is_sleeping {
            self.mood = PetMood::Sleeping;
            let msg = format!("🌙 Tucked {} into a cozy blanket. Goodnight!", self.name);
            self.log_action(msg.clone());
            msg
        } else {
            self.energy = (self.energy + 15).min(100);
            self.refresh_mood();
            let msg = format!("☀️ {} woke up feeling refreshed! (+15 Energy)", self.name);
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn clean(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if self.cleanliness == 100 && self.poop_count == 0 {
            let msg = format!("✨ {}'s room is already sparkling clean!", self.name);
            self.log_action(msg.clone());
            return msg;
        }

        self.cleanliness = 100;
        self.poop_count = 0;
        self.meals_since_poop = 0;
        self.happiness = (self.happiness + 10).min(100);
        self.health = (self.health + 5).min(100);
        self.add_xp(10);
        self.refresh_mood();

        let msg = format!(
            "🫧 Gave {} a warm bubble bath and tidied up! (+10 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn heal(&mut self) -> String {
        if let Some(msg) = self.check_egg_guard() {
            return msg;
        }
        if !self.is_sick && self.health >= 95 {
            let msg = format!("💚 {} is already in tip-top health!", self.name);
            self.log_action(msg.clone());
            return msg;
        }

        self.is_sick = false;
        self.health = 100;
        self.happiness = (self.happiness + 5).min(100);
        self.add_xp(10);
        self.refresh_mood();

        let msg = format!(
            "💊 Gave {} sweet herbal medicine! Fully recovered! (+10 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn apply_minigame_reward(&mut self, score: u32, coins_earned: u32, xp_earned: u32) -> String {
        if self.stage == EvolutionStage::Egg {
            self.coins = self.coins.saturating_add(coins_earned);
            let warm_msg = self.warm_egg();
            let msg = format!(
                "🎮 Star Catcher score {}! (+{} coins) {}",
                score, coins_earned, warm_msg
            );
            self.log_action(msg.clone());
            return msg;
        }

        self.coins = self.coins.saturating_add(coins_earned);
        self.happiness = (self.happiness + 10).min(100);
        self.bond = (self.bond + 3).min(100);
        self.add_xp(xp_earned);
        self.refresh_mood();

        let msg = format!(
            "🎮 Star Catcher score {}! Earned +{} coins & +{} XP!",
            score, coins_earned, xp_earned
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn apply_offline_decay(&mut self, now: DateTime<Utc>) -> u64 {
        let elapsed = now
            .signed_duration_since(self.last_updated)
            .num_seconds()
            .max(0) as u64;
        self.age_seconds = self.age_seconds.saturating_add(elapsed);
        self.last_updated = now;

        if self.stage == EvolutionStage::Egg || elapsed < 60 {
            return elapsed;
        }

        let minutes = (elapsed / 60).min(720) as u8;
        let hunger_drop = (minutes / 3).max(1);
        let happy_drop = (minutes / 4).max(1);
        let clean_drop = (minutes / 5).max(1);

        self.hunger = self.hunger.saturating_sub(hunger_drop);
        self.happiness = self.happiness.saturating_sub(happy_drop);
        self.cleanliness = self.cleanliness.saturating_sub(clean_drop);

        if self.is_sleeping {
            let energy_gain = (minutes / 2).max(2);
            self.energy = (self.energy.saturating_add(energy_gain)).min(100);
        } else {
            let energy_drop = (minutes / 4).max(1);
            self.energy = self.energy.saturating_sub(energy_drop);
        }

        if self.hunger == 0 || self.cleanliness < 20 {
            let health_drop = (minutes / 6).max(5);
            self.health = self.health.saturating_sub(health_drop);
            if self.health < 40 {
                self.is_sick = true;
            }
        }

        self.refresh_mood();
        elapsed
    }

    pub fn tick_decay(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();

        if self.stage == EvolutionStage::Egg {
            return;
        }

        self.hunger = self.hunger.saturating_sub(2);
        self.happiness = self.happiness.saturating_sub(1);
        self.cleanliness = self.cleanliness.saturating_sub(1);

        if self.is_sleeping {
            self.energy = (self.energy + 4).min(100);
        } else {
            self.energy = self.energy.saturating_sub(1);
        }

        if self.hunger < 15 || self.cleanliness < 20 || self.poop_count >= 3 {
            self.health = self.health.saturating_sub(2);
            if self.health < 40 {
                self.is_sick = true;
            }
        }

        self.refresh_mood();
    }

    pub fn status_summary(&self) -> String {
        format!(
            "{} ({}) | Stage: {} | Lv.{} ({} XP) | Hunger: {}% | Happy: {}% | Energy: {}% | Health: {}% | Coins: {}",
            self.name,
            self.species.display_name(),
            self.stage.display_name(),
            self.level,
            self.xp,
            self.hunger,
            self.happiness,
            self.energy,
            self.health,
            self.coins
        )
    }
}
