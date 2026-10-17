use super::*;
use chrono::{DateTime, Utc};

impl Pet {
    pub fn advisor_tip(&self, now: DateTime<Utc>) -> String {
        let caretaker = self.caretaker_label();
        if !self.is_alive {
            if self.buried {
                return format!(
                    "🪦 {} rests in the Memorial Hall. Press [N] to adopt a new companion.",
                    self.name
                );
            }
            if self.revival_count >= 1 {
                return format!(
                    "🕯 {} has fallen a second time (Phoenix Tear spent). Hold Vigil [B] ({}/3) to lay to rest.",
                    self.name, self.vigil_steps_completed
                );
            }
            return format!(
                "👻 {} has fallen! Press [B] for Vigil Rite ({}/3) or [H] Phoenix Tear (1× per life).",
                self.name, self.vigil_steps_completed
            );
        }
        if self.stage == EvolutionStage::Egg {
            let rem = self.warm_cooldown_remaining(now);
            if self.incubation >= 99 {
                return "🥚 Shell is cracking! Press [W] or [Space] to hatch.".to_string();
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
            return format!("🦷 {} has a Toothache! Press [H] for Medicine.", self.name);
        }
        if self.is_sick {
            return format!(
                "🤒 {} is running a fever (Health {}%)! Press [H] Medicine now.",
                self.name, self.health
            );
        }
        if self.hunger == 0 {
            return format!(
                "🚨 {} is starving (Health {}%)! Feed a meal [F] immediately!",
                self.name, self.health
            );
        }
        if self.is_sleeping && self.light_on {
            return format!(
                "💡 {} is asleep with the lamp ON! Press [K] to switch it OFF.",
                self.name
            );
        }
        if self.attention_call == AttentionCall::Misbehaving {
            return format!("📢 {} is acting spoiled! Press [D] to Scold.", self.name);
        }
        if self.attention_call == AttentionCall::Crying {
            return format!(
                "🥺 {} is crying for {caretaker}! Press [T] to Comfort.",
                self.name
            );
        }
        if self.poop_count > 0 || self.cleanliness < 45 {
            return format!(
                "🚽 Floor needs washing (Mess {})! Press [C] for a Bath.",
                self.poop_count
            );
        }
        if self.is_sleeping {
            if self.energy >= 95 {
                return format!("🌞 {} is fully rested! Press [S] to wake up.", self.name);
            }
            return format!(
                "🌙 {} is curled up asleep ({}% Energy).",
                self.name, self.energy
            );
        }
        if self.hungry_hearts() <= 1 {
            return "🍙 Hungry Hearts low! Press [F] Meal or [M] Pantry.".to_string();
        }
        if self.is_overweight() {
            return format!(
                "🏋 {} is overweight ({}g)! Exercise with [P] or Arcade [G].",
                self.name, self.weight_grams
            );
        }
        if self.energy <= 25 {
            return "🥱 Getting drowsy! Press [S] Sleep, then [K] Light OFF.".to_string();
        }
        if self.happy_hearts() <= 1 {
            return "🎾 Happy Hearts low! Press [P] Play or [Y] Talk.".to_string();
        }
        format!(
            "🌟 Calm in the sanctuary, {caretaker}. [Y] Talk • [G] Arcade • [O] Title • [?] Keys"
        )
    }
}
