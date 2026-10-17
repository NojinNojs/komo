use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EvolutionStage {
    Egg,
    Baby,
    Teen,
    Adult,
    CyberElder,
}

impl EvolutionStage {
    pub fn all() -> &'static [EvolutionStage] {
        &[
            EvolutionStage::Egg,
            EvolutionStage::Baby,
            EvolutionStage::Teen,
            EvolutionStage::Adult,
            EvolutionStage::CyberElder,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            EvolutionStage::Egg => "Egg",
            EvolutionStage::Baby => "Baby",
            EvolutionStage::Teen => "Teen",
            EvolutionStage::Adult => "Adult",
            EvolutionStage::CyberElder => "CyberElder",
        }
    }

    pub fn min_xp(&self) -> u32 {
        match self {
            EvolutionStage::Egg => 0,
            EvolutionStage::Baby => 50,
            EvolutionStage::Teen => 150,
            EvolutionStage::Adult => 350,
            EvolutionStage::CyberElder => 700,
        }
    }

    pub fn next_stage(&self) -> Option<EvolutionStage> {
        match self {
            EvolutionStage::Egg => Some(EvolutionStage::Baby),
            EvolutionStage::Baby => Some(EvolutionStage::Teen),
            EvolutionStage::Teen => Some(EvolutionStage::Adult),
            EvolutionStage::Adult => Some(EvolutionStage::CyberElder),
            EvolutionStage::CyberElder => None,
        }
    }

    pub fn from_xp(xp: u32) -> EvolutionStage {
        if xp >= 700 {
            EvolutionStage::CyberElder
        } else if xp >= 350 {
            EvolutionStage::Adult
        } else if xp >= 150 {
            EvolutionStage::Teen
        } else if xp >= 50 {
            EvolutionStage::Baby
        } else {
            EvolutionStage::Egg
        }
    }

    pub fn max_stat_bonus(&self) -> u8 {
        match self {
            EvolutionStage::Egg => 0,
            EvolutionStage::Baby => 5,
            EvolutionStage::Teen => 10,
            EvolutionStage::Adult => 15,
            EvolutionStage::CyberElder => 25,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetMood {
    Idle,
    Eating,
    Sleeping,
    Happy,
    Sick,
    Coding,
}

pub type PetState = PetMood;

impl PetMood {
    pub fn all() -> &'static [PetMood] {
        &[
            PetMood::Idle,
            PetMood::Eating,
            PetMood::Sleeping,
            PetMood::Happy,
            PetMood::Sick,
            PetMood::Coding,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            PetMood::Idle => "Idle",
            PetMood::Eating => "Eating",
            PetMood::Sleeping => "Sleeping",
            PetMood::Happy => "Happy",
            PetMood::Sick => "Sick",
            PetMood::Coding => "Coding",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetAction {
    Feed,
    Play,
    ToggleSleep,
    Clean,
    CodeSession,
    Heal,
    GitCommit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
    pub stage: EvolutionStage,
    pub mood: PetMood,
    pub hunger: u8,
    pub happiness: u8,
    pub energy: u8,
    pub health: u8,
    pub cleanliness: u8,
    pub xp: u32,
    pub level: u32,
    pub commits_fed: u32,
    pub coins: u32,
    pub is_sleeping: bool,
    pub is_sick: bool,
    pub last_updated: DateTime<Utc>,
    pub age_seconds: u64,
    pub action_log: Vec<String>,
    pub seen_commits: Vec<String>,
}

impl Default for Pet {
    fn default() -> Self {
        Self::new("Ferris")
    }
}

impl Pet {
    pub fn new(name: &str) -> Self {
        let trimmed = name.trim();
        let final_name = if trimmed.is_empty() {
            "Ferris".to_string()
        } else {
            trimmed.to_string()
        };

        Self {
            action_log: vec![format!("{final_name} hatched into the terminal!")],
            name: final_name,
            stage: EvolutionStage::Egg,
            mood: PetMood::Idle,
            hunger: 80,
            happiness: 80,
            energy: 80,
            health: 100,
            cleanliness: 90,
            xp: 0,
            level: 1,
            commits_fed: 0,
            coins: 10,
            is_sleeping: false,
            is_sick: false,
            last_updated: Utc::now(),
            age_seconds: 0,
            seen_commits: Vec::new(),
        }
    }

    pub fn rename(&mut self, new_name: &str) {
        let trimmed = new_name.trim();
        if !trimmed.is_empty() {
            self.name = trimmed.to_string();
            self.log_action(format!("Renamed pet to {}", self.name));
        }
    }

    pub fn log_action(&mut self, msg: impl Into<String>) {
        self.action_log.push(msg.into());
        while self.action_log.len() > 20 {
            self.action_log.remove(0);
        }
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp = self.xp.saturating_add(amount);
        self.level = 1 + (self.xp / 50);
        self.check_evolution()
    }

    pub fn check_evolution(&mut self) -> bool {
        let new_stage = EvolutionStage::from_xp(self.xp);
        if new_stage > self.stage {
            self.stage = new_stage;
            self.log_action(format!(
                "{} evolved into {}!",
                self.name,
                self.stage.name()
            ));
            true
        } else {
            false
        }
    }

    pub fn update_mood(&mut self) {
        if self.is_sick {
            self.mood = PetMood::Sick;
        } else if self.is_sleeping {
            self.mood = PetMood::Sleeping;
        } else if self.happiness >= 85 && self.hunger >= 60 {
            self.mood = PetMood::Happy;
        } else {
            self.mood = PetMood::Idle;
        }
    }

    pub fn apply_offline_decay(&mut self, now: DateTime<Utc>) -> u64 {
        if now <= self.last_updated {
            return 0;
        }

        let elapsed = (now - self.last_updated).num_seconds() as u64;
        self.age_seconds = self.age_seconds.saturating_add(elapsed);
        self.last_updated = now;

        let raw_steps = (elapsed / 300).min(100) as u8;
        let steps = raw_steps.max(u8::from(elapsed >= 60)).min(100);
        let decay = steps.saturating_mul(2);

        self.hunger = self.hunger.saturating_sub(decay);
        self.happiness = self.happiness.saturating_sub(decay);
        self.cleanliness = self.cleanliness.saturating_sub(decay);

        if self.is_sleeping {
            self.energy = self.energy.saturating_add(steps.saturating_mul(3)).min(100);
        } else {
            self.energy = self.energy.saturating_sub(decay);
        }

        if self.hunger <= 15 || self.cleanliness <= 15 {
            self.is_sick = true;
            self.health = self.health.saturating_sub(15);
        }

        self.update_mood();
        elapsed
    }

    pub fn tick_decay(&mut self) {
        self.age_seconds = self.age_seconds.saturating_add(1);
        self.last_updated = Utc::now();
        self.hunger = self.hunger.saturating_sub(1);
        self.cleanliness = self.cleanliness.saturating_sub(1);

        if self.is_sleeping {
            self.energy = (self.energy + 3).min(100);
        } else {
            self.energy = self.energy.saturating_sub(1);
            self.happiness = self.happiness.saturating_sub(1);
        }

        if self.hunger <= 15 || self.cleanliness <= 15 {
            self.is_sick = true;
            self.health = self.health.saturating_sub(2);
        }

        self.update_mood();
    }

    pub fn feed(&mut self) -> String {
        self.is_sleeping = false;
        self.hunger = (self.hunger + 25).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.cleanliness = self.cleanliness.saturating_sub(3);
        self.add_xp(10);
        self.mood = PetMood::Eating;

        let msg = format!(
            "Fed {} a delicious crate snack! (+25 hunger, +10 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn play(&mut self) -> String {
        self.is_sleeping = false;
        self.happiness = (self.happiness + 20).min(100);
        self.energy = self.energy.saturating_sub(10);
        self.hunger = self.hunger.saturating_sub(8);
        self.coins = self.coins.saturating_add(5);
        self.add_xp(15);
        self.mood = PetMood::Happy;

        let msg = format!(
            "Played borrow-checker fetch with {}! (+20 happiness, +15 XP, +5 coins)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn toggle_sleep(&mut self) -> String {
        self.is_sleeping = !self.is_sleeping;
        let msg = if self.is_sleeping {
            self.mood = PetMood::Sleeping;
            self.energy = (self.energy + 10).min(100);
            format!("{} curled up for a nap. Zzz...", self.name)
        } else {
            self.update_mood();
            format!("{} woke up refreshed!", self.name)
        };
        self.log_action(msg.clone());
        msg
    }

    pub fn clean(&mut self) -> String {
        self.cleanliness = 100;
        self.health = (self.health + 10).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.add_xp(10);
        self.update_mood();

        let msg = format!(
            "Ran `cargo clean` on {}! Cleanliness restored to 100%.",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn code_session(&mut self) -> String {
        self.is_sleeping = false;
        self.add_xp(25);
        self.coins = self.coins.saturating_add(15);
        self.happiness = (self.happiness + 10).min(100);
        self.energy = self.energy.saturating_sub(15);
        self.hunger = self.hunger.saturating_sub(10);
        self.mood = PetMood::Coding;

        let msg = format!(
            "{} paired on a Rust coding session! (+25 XP, +15 coins)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn heal(&mut self) -> String {
        self.is_sick = false;
        self.health = 100;
        self.happiness = (self.happiness + 10).min(100);
        self.add_xp(10);
        self.update_mood();

        let msg = format!(
            "Patched {}'s memory leaks! Health restored to 100%.",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn reward_git_commit(&mut self, commit_id_or_msg: &str) -> bool {
        let trimmed = commit_id_or_msg.trim();
        if trimmed.is_empty() || self.seen_commits.iter().any(|c| c == trimmed) {
            return false;
        }

        self.seen_commits.push(trimmed.to_string());
        self.commits_fed = self.commits_fed.saturating_add(1);
        self.hunger = (self.hunger + 15).min(100);
        self.happiness = (self.happiness + 15).min(100);
        self.coins = self.coins.saturating_add(20);
        self.add_xp(40);
        self.mood = PetMood::Coding;

        self.log_action(format!("Fed git commit '{trimmed}' (+40 XP, +20 coins)"));
        true
    }

    pub fn apply_minigame_reward(&mut self, score: u32, coins_earned: u32, xp_earned: u32) -> String {
        self.coins = self.coins.saturating_add(coins_earned);
        self.add_xp(xp_earned);
        self.happiness = (self.happiness + 15).min(100);
        self.mood = PetMood::Happy;

        let msg = format!(
            "Minigame finished! Score: {score} (+{xp_earned} XP, +{coins_earned} coins)"
        );
        self.log_action(msg.clone());
        msg
    }

    pub fn xp_progress_ratio(&self) -> f64 {
        if let Some(next) = self.stage.next_stage() {
            let start = self.stage.min_xp();
            let end = next.min_xp();
            if end <= start {
                1.0
            } else {
                (self.xp.saturating_sub(start) as f64 / (end - start) as f64).clamp(0.0, 1.0)
            }
        } else {
            1.0
        }
    }

    pub fn status_summary(&self) -> String {
        format!(
            "Pet: {} | Stage: {} (Lv.{}, {} XP) | Mood: {} | Hunger: {}% | Happiness: {}% | Energy: {}% | Health: {}% | Cleanliness: {}% | Coins: {} | Commits Fed: {}",
            self.name,
            self.stage.name(),
            self.level,
            self.xp,
            self.mood.label(),
            self.hunger,
            self.happiness,
            self.energy,
            self.health,
            self.cleanliness,
            self.coins,
            self.commits_fed
        )
    }
}
