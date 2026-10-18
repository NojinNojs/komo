use super::*;
use chrono::Utc;

impl Pet {
    pub fn answer_dialogue(&mut self, choice: u8) -> String {
        let now = Utc::now();
        self.last_updated = now;
        let caretaker = self.caretaker_label().to_string();

        if !self.is_alive {
            if choice == 2 {
                return self.heal();
            }
            return self.hold_vigil_step();
        }

        if self.stage == EvolutionStage::Egg {
            self.bond = (self.bond + 3).min(100);
            self.incubation = (self.incubation + 2).min(99);
            let msg = format!(
                "💬 Spoke softly to {}'s egg (+2% warmth, +3 Bond).",
                self.name
            );
            self.set_pet_reaction(
                PetAction::Talk,
                format!("💬 \"*tap-tap* Heard your voice, {caretaker}. Warm and safe in here.\""),
                format!("💭 (Hearing: {caretaker}'s voice carries clearly through the shell.)"),
                Some(PetMood::Loved),
            );
            self.log_action(msg.clone());
            return msg;
        }

        let rem = self.talk_reward_cooldown_remaining(now);
        if rem > 0 {
            if self.happiness < 90 {
                self.happiness = self.happiness.saturating_add(1).min(100);
            }
            self.set_pet_reaction(
                PetAction::Talk,
                format!(
                    "💬 \"Hehe, my throat's getting dry from talking non-stop, {caretaker}! Let's hang out for {}s.\"",
                    rem
                ),
                format!("💭 (Feeling: Enjoying quiet company with {caretaker} while voice rests.)"),
                Some(if self.is_sleeping {
                    PetMood::Sleeping
                } else {
                    PetMood::Happy
                }),
            );
            let msg = format!(
                "💬 Chatted with {} (throat's getting dry — next XP/coin reward in {}s).",
                self.name, rem
            );
            self.log_action(msg.clone());
            return msg;
        }

        self.last_talk_reward_at = Some(now);
        if choice == 1 {
            self.bond = (self.bond + 5).min(100);
            self.happiness = (self.happiness + 8).min(100);
            self.discipline = (self.discipline + 4).min(100);
            self.add_xp(12);
            self.set_pet_reaction(
                PetAction::Talk,
                format!(
                    "💬 \"Glad we talked, {caretaker}. {} feels focused and ready.\"",
                    self.name
                ),
                format!("💭 (Feeling: Steady trust with {caretaker}.)"),
                Some(if self.is_sleeping {
                    PetMood::Sleeping
                } else {
                    PetMood::Excited
                }),
            );
            let msg = format!(
                "💬 Shared a quiet talk with {} (+12 XP, +5 Bond, +8 Happy).",
                self.name
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.bond = (self.bond + 5).min(100);
            self.happiness = (self.happiness + 14).min(100);
            self.coins = self.coins.saturating_add(5);
            self.add_xp(8);
            self.set_pet_reaction(
                PetAction::Talk,
                format!("💬 \"Thanks, {caretaker}. Dropped +5 coins into our pouch.\""),
                format!("💭 (Feeling: Warm and relaxed beside {caretaker}.)"),
                Some(if self.is_sleeping {
                    PetMood::Sleeping
                } else {
                    PetMood::Loved
                }),
            );
            let msg = format!(
                "💬 Warm moment with {}! Found +5 coins (+14 Happy, +5 Bond).",
                self.name
            );
            self.log_action(msg.clone());
            msg
        }
    }
}
