use super::*;
use crate::senses::PetGaze;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AbsenceTier {
    #[default]
    ShortVisit,
    MediumReturn,
    LateNightLamp,
    LongNeglect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WelcomeHostEvent {
    pub elapsed_seconds: u64,
    pub tier: AbsenceTier,
    pub gaze: PetGaze,
    pub greeting_speech: String,
    pub greeting_thought: String,
    pub found_coins: u32,
    pub gift_note: Option<String>,
    pub option_1: String,
    pub option_2: String,
    pub option_3: String,
}

impl Pet {
    pub fn build_welcome_host_event(
        &self,
        elapsed_seconds: u64,
        found_coins: u32,
        now: DateTime<Utc>,
    ) -> WelcomeHostEvent {
        let caretaker = self.caretaker_label();
        let tier = if !self.is_alive
            || self.hunger <= 25
            || self.health <= 45
            || elapsed_seconds >= 6 * 3600
        {
            AbsenceTier::LongNeglect
        } else if now.hour() >= 22 || now.hour() < 5 {
            AbsenceTier::LateNightLamp
        } else if elapsed_seconds >= 900 {
            AbsenceTier::MediumReturn
        } else {
            AbsenceTier::ShortVisit
        };

        let (speech, thought, gift_note) = match tier {
            AbsenceTier::ShortVisit => (
                format!("💬 \"Oh, you're back already, {caretaker}! I was just watching the clouds outside.\""),
                format!("💭 (Feeling: Gently glad you returned so soon, {caretaker}.)"),
                None,
            ),
            AbsenceTier::MediumReturn => (
                format!("💬 \"Welcome home, {caretaker}! I heard footsteps and ran straight to the glass! Look what I found while you were out...\""),
                format!("💭 (Feeling: Brimming with little discoveries to share with {caretaker}.)"),
                Some(format!("🎁 Found +{} lucky coins in the sanctuary garden!", found_coins.max(3))),
            ),
            AbsenceTier::LateNightLamp => (
                format!("💬 \"Working late, {caretaker}? Come sit by the warm lamp with me for a bit.\""),
                "💭 (Feeling: Quiet, warm company under the soft lamp glow.)".to_string(),
                None,
            ),
            AbsenceTier::LongNeglect => (
                format!("💬 \"*{}'s eyes glisten with relief* You came back, {caretaker}... I missed you so much.\"", self.name),
                format!("💭 (Feeling: Overwhelming relief that {caretaker} is finally home.)"),
                None,
            ),
        };

        WelcomeHostEvent {
            elapsed_seconds,
            tier,
            gaze: PetGaze::TowardKeeper,
            greeting_speech: speech,
            greeting_thought: thought,
            found_coins,
            gift_note,
            option_1: "1. Pat head & say \"I'm home!\" (+Bond, +Happy)".to_string(),
            option_2: "2. Give a warm welcome treat (+Hungry, +Happy)".to_string(),
            option_3: "3. Ask what happened while you were away (+XP, +Coins)".to_string(),
        }
    }

    pub fn prepare_welcome_hosting(
        &mut self,
        elapsed_seconds: u64,
        found_coins: u32,
        now: DateTime<Utc>,
    ) {
        if self.is_alive && self.stage != EvolutionStage::Egg {
            let ev = self.build_welcome_host_event(elapsed_seconds, found_coins, now);
            self.last_pet_speech = Some(ev.greeting_speech.clone());
            self.last_pet_thought = Some(ev.greeting_thought.clone());
            self.reaction_ticks_left = 6;
            self.pending_hosting = Some(ev);
        }
    }

    pub fn respond_welcome_hosting(&mut self, choice: u8) -> String {
        let now = Utc::now();
        self.last_updated = now;
        let caretaker = self.caretaker_label().to_string();
        let Some(ev) = self.pending_hosting.take() else {
            return format!("💬 {} has no pending homecoming to answer.", self.name);
        };

        let (msg, mood) = match choice {
            1 => {
                self.bond = (self.bond + 6).min(100);
                self.happiness = (self.happiness + 10).min(100);
                self.add_xp(8);
                (
                    format!("💬 Patted {} on the head and said \"I'm home!\" (+6 Bond, +10 Happy, +8 XP)", self.name),
                    PetMood::Loved,
                )
            }
            2 => {
                self.hunger = (self.hunger + 18).min(100);
                self.happiness = (self.happiness + 12).min(100);
                self.bond = (self.bond + 4).min(100);
                self.add_xp(8);
                (
                    format!(
                        "🍙 Gave {} a warm welcome treat (+18 Hungry, +12 Happy, +4 Bond, +8 XP)",
                        self.name
                    ),
                    PetMood::Eating,
                )
            }
            _ => {
                self.bond = (self.bond + 5).min(100);
                self.add_xp(10);
                self.coins = self.coins.saturating_add(5);
                if ev.tier == AbsenceTier::MediumReturn && ev.found_coins == 0 {
                    self.coins = self.coins.saturating_add(3);
                }
                (
                    format!(
                        "💬 Asked {} what they did while away (+5 Bond, +10 XP, +5 Coins)",
                        self.name
                    ),
                    PetMood::Excited,
                )
            }
        };

        self.set_pet_reaction(
            PetAction::Talk,
            format!("💬 \"I'm so happy to see you, {caretaker}.\""),
            format!("💭 (Feeling: Warm reunion with {caretaker}.)"),
            Some(mood),
        );
        self.log_action(msg.clone());
        msg
    }
}
