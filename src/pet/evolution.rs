use super::*;
impl Pet {
    pub fn level_up_threshold(&self) -> u32 {
        100
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp = self.xp.saturating_add(amount);
        self.level = 1 + (self.xp / self.level_up_threshold());
        self.check_evolution()
    }

    pub fn check_evolution(&mut self) -> bool {
        if !self.is_alive {
            return false;
        }
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
                "Next {} • XP {}/{} • Bond {}/{}",
                next.badge(),
                self.xp,
                next.min_xp(),
                self.bond,
                next.min_bond(),
            ),
            None => "Max evolution reached — Apex Sovereign Elder 🌟".to_string(),
        }
    }
}
