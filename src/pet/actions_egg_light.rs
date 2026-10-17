use super::*;
use chrono::Utc;

impl Pet {
    pub fn perform_action(&mut self, action: PetAction) -> String {
        match action {
            PetAction::WarmEgg => self.warm_egg(),
            PetAction::Feed => self.feed(),
            PetAction::FeedSnack => self.feed_choice(FoodChoice::WarmMilkTea),
            PetAction::Play => self.play(),
            PetAction::PetLove => self.pet_love(),
            PetAction::Talk => self.answer_dialogue(1),
            PetAction::Train => self.train(),
            PetAction::ToggleSleep => self.toggle_sleep(),
            PetAction::ToggleLight => self.toggle_light(),
            PetAction::DisciplineScold => self.discipline_scold(),
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

    pub(super) fn dead_guard(&mut self) -> Option<String> {
        if !self.is_alive {
            let caretaker = self.caretaker_label().to_string();
            self.set_pet_reaction(
                PetAction::Heal,
                format!(
                    "💬 \"👻 {} has passed out, {caretaker}... Press [H] for Phoenix Revival or [B] for Vigil.\"",
                    self.name
                ),
                "💭 (Spirit: Waiting for a Phoenix Tear [H] or the 3-step Burial Vigil [B]...)".to_string(),
                Some(PetMood::PassedOut),
            );
            Some(format!(
                "👻 {} has passed out! Press [H] to revive them with a Phoenix Tear or [B] to hold a Vigil!",
                self.name
            ))
        } else {
            None
        }
    }

    pub(super) fn egg_guard(&self) -> Option<String> {
        if self.stage == EvolutionStage::Egg {
            Some("🥚 The egg is still unhatched! Press [W] to warm & incubate it.".to_string())
        } else {
            None
        }
    }

    pub(super) fn refuse(&mut self, msg: String) -> String {
        self.log_action(msg.clone());
        msg
    }

    pub fn warm_egg(&mut self) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if self.stage != EvolutionStage::Egg {
            return self.pet_love();
        }

        let now = Utc::now();
        let caretaker = self.caretaker_label().to_string();
        let remaining = self.warm_cooldown_remaining(now);
        if remaining > 0 {
            self.set_pet_reaction(
                PetAction::WarmEgg,
                format!(
                    "💬 \"*tap-tap* The shell is still holding your warmth, {caretaker}. Wait {}.\"",
                    format_duration_human(remaining)
                ),
                "💭 (Inside Egg: Resting in the steady heat of the cushion.)".to_string(),
                None,
            );
            return self.refuse(format!(
                "⏳ The egg is still absorbing your warmth! Wait {} (passive incubation continues).",
                format_duration_human(remaining)
            ));
        }

        self.last_warmed_at = Some(now);
        self.last_updated = now;
        self.incubation = (self.incubation + 20).min(100);
        self.bond = (self.bond + 4).min(100);

        if self.incubation >= 100 {
            self.stage = EvolutionStage::Baby;
            self.weight_grams = EvolutionStage::Baby.base_weight_grams();
            self.xp = self.xp.max(EvolutionStage::Baby.min_xp());
            self.level = 2;
            self.hunger = 70;
            self.happiness = 88;
            self.energy = 92;
            self.coins = self.coins.saturating_add(20);
            self.set_pet_reaction(
                PetAction::WarmEgg,
                format!(
                    "💬 \"*CRACK!* {} blinks up at you for the first time, {caretaker}.\"",
                    self.name
                ),
                format!("💭 (Sight: Locking eyes with {caretaker} right outside the shell.)"),
                Some(PetMood::Excited),
            );
            let msg = format!(
                "🐣 CRACK! {} hatched into a precious {} ({}, {}g)! (+20 coins)",
                self.name,
                self.species.display_name(),
                self.stage.badge(),
                self.weight_grams
            );
            self.log_action(msg.clone());
            msg
        } else {
            self.set_pet_reaction(
                PetAction::WarmEgg,
                format!(
                    "💬 \"*thump-thump* Warmth settles into the shell, {caretaker} ({}% incubated).\"",
                    self.incubation
                ),
                "💭 (Feeling: The inner membrane is growing warm and strong.)".to_string(),
                Some(PetMood::Loved),
            );
            let msg = format!(
                "🤲 You gently warm {}'s egg... Incubation at {}% (+4 Bond)",
                self.name, self.incubation
            );
            self.log_action(msg.clone());
            msg
        }
    }
}
