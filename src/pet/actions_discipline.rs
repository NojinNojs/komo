use super::*;
use chrono::Utc;

impl Pet {
    pub fn discipline_scold(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::DisciplineScold,
                format!("💬 \"*flinches in sleep* ...please let me rest, {caretaker}...\""),
                "💭 (Hearing: Startled half-awake by a sharp voice.)".to_string(),
                Some(PetMood::Sleeping),
            );
            return self.refuse(format!(
                "💤 {} is sleeping — don't scold them in bed!",
                self.name
            ));
        }

        self.last_updated = Utc::now();
        if self.attention_call == AttentionCall::Misbehaving {
            self.discipline = (self.discipline + 25).min(100);
            self.bond = (self.bond + 3).min(100);
            self.attention_call = AttentionCall::None;
            self.attention_since = None;
            self.add_xp(12);
            self.set_pet_reaction(
                PetAction::DisciplineScold,
                format!(
                    "💬 \"Ears dip low. 'Sorry, {caretaker}. I'll stop ringing the bell for nothing.'\""
                ),
                "💭 (Feeling: Chastened, but respects your clear boundary.)".to_string(),
                Some(PetMood::Proud),
            );
            self.refresh_mood();
            let msg = format!(
                "📢 Scolded {} for the fake attention call! Discipline rose to {}% (+12 XP)!",
                self.name, self.discipline
            );
            self.log_action(msg.clone());
            msg
        } else {
            let repeated = self.last_action_kind == Some(PetAction::DisciplineScold);
            self.happiness = self.happiness.saturating_sub(if repeated { 14 } else { 8 });
            self.bond = self.bond.saturating_sub(if repeated { 6 } else { 3 });
            if repeated {
                self.care_mistakes = self.care_mistakes.saturating_add(1);
                self.set_pet_reaction(
                    PetAction::DisciplineScold,
                    format!(
                        "💬 \"HMPH! Why do you keep yelling at me when I did nothing wrong, {caretaker}?!\""
                    ),
                    "💭 (Feeling: Hurt and resentful from repeated unfair scolding; turning away.)"
                        .to_string(),
                    Some(PetMood::Angry),
                );
            } else {
                self.set_pet_reaction(
                    PetAction::DisciplineScold,
                    format!(
                        "💬 \"Flinches back. 'What did I do wrong, {caretaker}? I was behaving...'\""
                    ),
                    "💭 (Feeling: Confused and stung by an unprovoked scold.)".to_string(),
                    Some(PetMood::Lonely),
                );
            }
            self.refresh_mood();
            let msg = format!(
                "🥺 {} wasn't misbehaving! Unfair scold damaged trust (-Bond, -Happiness{}).",
                self.name,
                if repeated { ", +1 Care Mistake" } else { "" }
            );
            self.log_action(msg.clone());
            msg
        }
    }

    pub fn feed(&mut self) -> String {
        self.feed_choice(FoodChoice::FavoriteSnack)
    }
}
