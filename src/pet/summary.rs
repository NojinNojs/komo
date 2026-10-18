use super::*;
use chrono::{DateTime, Utc};

impl Pet {
    pub fn apply_offline_decay(&mut self, now: DateTime<Utc>) -> u64 {
        self.apply_offline_decay_with_report(now).0
    }

    pub fn status_summary(&self) -> String {
        format!(
            "{} — {} {} | Stage: {} | Mood: {} | Lv.{} ({} XP) | Rank: {} | Weight: {} | Hungry: {} ({}%) • Happy: {} ({}%) • Discipline: {}% • Energy: {}% • Health: {}% • Clean: {}% • Bond: {}% | Mistakes: {} | Coins: {}",
            self.name,
            self.species.display_name(),
            self.species.rarity(),
            self.stage.badge(),
            self.mood.badge(),
            self.level,
            self.xp,
            self.care_rank().badge(),
            self.format_weight(),
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
