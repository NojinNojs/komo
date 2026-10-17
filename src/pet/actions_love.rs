use super::*;
use chrono::Utc;

impl Pet {
    pub fn pet_love(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::PetLove,
                format!("💬 \"*purrs softly in sleep* ...warm hand, {caretaker}...\""),
                "💭 (Touch: Feels a gentle hand resting on its head while asleep.)".to_string(),
                Some(PetMood::Sleeping),
            );
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
            self.add_xp(10);
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"Leans into your hand and stops sniffling. 'Thanks for staying close, {caretaker}.'\""
                ),
                format!("💭 (Touch: Calmed down as soon as {caretaker} reached out.)"),
                Some(PetMood::Loved),
            );
            let msg = format!(
                "💖 Praised & comforted crying {}! (+15% Discipline, +14 Happy, +10 XP)",
                self.name
            );
            self.log_action(msg.clone());
            return msg;
        }

        let remaining = self.pet_cooldown_remaining(now);
        if remaining > 0 {
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"Haha, that tickles, {caretaker}! My fur is getting ruffled (wait {}s).\"",
                    remaining
                ),
                "💭 (Touch: Ticklish from rapid headpats!)".to_string(),
                Some(PetMood::Playful),
            );
            return self.refuse(format!(
                "💕 {} is still basking in your headpats! Wait {}s.",
                self.name, remaining
            ));
        }
        if self.happiness >= 98 {
            self.happiness = 100;
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"Closes both eyes and leans against your palm, {caretaker}. Couldn't be happier.\""
                ),
                "💭 (Feeling: Completely content and safe.)".to_string(),
                Some(PetMood::Loved),
            );
            return self.refuse(format!(
                "💖 {} purrs happily! Happiness is already maxed out.",
                self.name
            ));
        }

        self.last_petted_at = Some(now);
        self.last_updated = now;
        self.happiness = (self.happiness + 8).min(100);
        self.bond = (self.bond + 5).min(100);
        self.add_xp(6);
        self.set_pet_reaction(
            PetAction::PetLove,
            format!(
                "💬 \"Nuzzles your palm warmly. 'Your hand is always warm, {caretaker}.' (+5 Bond)\""
            ),
            format!("💭 (Touch:Steady warmth from {caretaker}'s hand.)"),
            Some(PetMood::Loved),
        );

        let msg = format!(
            "💖 Gave {} warm cuddles and headpats! (+8 Happy, +5 Bond, +6 XP)",
            self.name
        );
        self.log_action(msg.clone());
        msg
    }
}
