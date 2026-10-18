use super::*;
use chrono::Utc;

impl Pet {
    pub fn feed_choice(&mut self, choice: FoodChoice) -> String {
        if let Some(msg) = self.dead_guard() {
            return self.refuse(msg);
        }
        if let Some(msg) = self.egg_guard() {
            return self.refuse(msg);
        }
        let caretaker = self.caretaker_label().to_string();
        if self.is_sleeping {
            self.set_pet_reaction(
                PetAction::Feed,
                format!("💬 \"*mumbles* Still asleep, {caretaker}... wake me with [S] first...\""),
                "💭 (Smell: Catches the scent of food, but too drowsy to chew.)".to_string(),
                Some(PetMood::Sleeping),
            );
            return self.refuse(format!(
                "💤 {} is fast asleep! Wake them up with [S] first.",
                self.name
            ));
        }
        if self.has_toothache {
            self.set_pet_reaction(
                PetAction::Feed,
                format!(
                    "💬 \"Winces and paws at its jaw. 'My tooth hurts too much to chew, {caretaker}. Need [H].'\""
                ),
                "💭 (Feeling: Sharp molar pain blocks every bite.)".to_string(),
                Some(PetMood::Sick),
            );
            return self.refuse(format!(
                "🦷 {} has a Toothache and refuses to eat! Give Medicine [H] first!",
                self.name
            ));
        }

        let is_snack = choice.is_sweet_snack();
        let repeated_feed = matches!(
            self.last_action_kind,
            Some(PetAction::Feed | PetAction::FeedSnack)
        );

        if !is_snack && self.hunger >= 95 {
            if repeated_feed {
                self.happiness = self.happiness.saturating_sub(6);
                self.health = self.health.saturating_sub(6);
                self.bond = self.bond.saturating_sub(3);
                self.care_mistakes = self.care_mistakes.saturating_add(1);
                self.set_pet_reaction(
                    PetAction::Feed,
                    format!(
                        "💬 \"UGH! Stop forcing food on me, {caretaker}! My stomach feels sick and cramped!\""
                    ),
                    "💭 (Feeling: Nauseous from being force-fed while already 4/4 full.)".to_string(),
                    Some(PetMood::Overstuffed),
                );
            } else {
                self.set_pet_reaction(
                    PetAction::Feed,
                    format!(
                        "💬 \"Pats its round belly. 'I'm completely full (4/4 ♥), {caretaker}. Let's play [P] instead.'\""
                    ),
                    "💭 (Feeling: Stomach is warm and full to capacity.)".to_string(),
                    Some(PetMood::Overstuffed),
                );
            }
            return self.refuse(format!(
                "🍱 {} shakes their head: stomach is already completely full (4/4 ♥)!",
                self.name
            ));
        }

        let now = Utc::now();
        let remaining = self.feed_cooldown_remaining(now);
        if !is_snack && remaining > 0 && self.hunger >= 60 {
            self.set_pet_reaction(
                PetAction::Feed,
                format!(
                    "💬 \"Still digesting the last portion, {caretaker}. Give me {} more.\"",
                    format_duration_human(remaining)
                ),
                "💭 (Feeling: Stomach is still settling after the last meal.)".to_string(),
                Some(PetMood::Overstuffed),
            );
            return self.refuse(format!(
                "⏳ {} is still digesting their last meal! Wait {}.",
                self.name,
                format_duration_human(remaining)
            ));
        }
        let cost = choice.cost();
        if self.coins < cost {
            self.set_pet_reaction(
                PetAction::Feed,
                format!(
                    "💬 \"We're short on coins for {} (need {} 💰), {caretaker}. A free meal works too.\"",
                    choice.title(self.species),
                    cost
                ),
                "💭 (Sight: Looking at the light coin pouch; Arcade [G] pays well.)".to_string(),
                None,
            );
            return self.refuse(format!(
                "💰 Not enough coins for {} (need {} coins, have {}). Try a Free Meal/Snack!",
                choice.title(self.species),
                cost,
                self.coins
            ));
        }
        self.coins -= cost;

        let (hunger_gain, happy_gain, energy_gain, health_gain, bond_gain, xp_gain, weight_gain) =
            match choice {
                FoodChoice::FavoriteSnack => (25, 6, 0, 2, 3, 10, 35),
                FoodChoice::HeartyBento => (40, 8, 5, 4, 3, 14, 70),
                FoodChoice::WarmMilkTea => (5, 25, 8, 0, 4, 10, 45),
                FoodChoice::HerbalSoup => {
                    self.is_sick = false;
                    (25, 5, 4, 20, 3, 14, 30)
                }
                FoodChoice::StarlightDelight => (15, 35, 10, 5, 8, 25, 80),
            };

        self.hunger = (self.hunger + hunger_gain).min(100);
        self.happiness = (self.happiness + happy_gain).min(100);
        self.energy = (self.energy + energy_gain).min(100);
        self.health = (self.health + health_gain).min(100);
        self.bond = (self.bond + bond_gain).min(100);
        self.weight_grams = (self.weight_grams + weight_gain).min(9_999);

        if is_snack {
            self.snacks_in_a_row = self.snacks_in_a_row.saturating_add(1);
        } else {
            self.snacks_in_a_row = 0;
        }

        self.meals_since_poop = self.meals_since_poop.saturating_add(1);
        if self.meals_since_poop >= 2 {
            self.meals_since_poop = 0;
            self.poop_count = (self.poop_count + 1).min(5);
            self.cleanliness = self.cleanliness.saturating_sub(15);
            if self.poop_count >= 4 {
                self.is_sick = true;
            }
        }

        self.last_fed_at = Some(now);
        self.last_updated = now;
        self.add_xp(xp_gain);

        let food_name = choice.title(self.species);
        let weight_str = self.format_weight();
        let mut msg = format!(
            "🍡 Fed {} {}! (+{}g weight -> {}, +{} XP)",
            self.name, food_name, weight_gain, weight_str, xp_gain
        );

        if self.snacks_in_a_row >= 3 {
            self.has_toothache = true;
            self.care_mistakes = self.care_mistakes.saturating_add(1);
            self.set_pet_reaction(
                PetAction::FeedSnack,
                format!("💬 \"OUCH! Three sweets in a row gave me a sharp toothache, {caretaker}! Need medicine [H]!\""),
                "💭 (Feeling: Throbbing molar ache from three consecutive sugary treats.)".to_string(),
                Some(PetMood::Sick),
            );
            msg.push_str(
                " — 🦷 OH NO! Too many sweets caused a Toothache! Press [H] for Medicine!",
            );
        } else if is_snack && self.snacks_in_a_row == 2 {
            self.set_pet_reaction(
                PetAction::FeedSnack,
                format!(
                    "💬 \"Sweet and warm, {caretaker}... that's two treats in a row though; one more will hurt my teeth.\""
                ),
                "💭 (Feeling: Sugar rush—better eat a real meal or brush teeth next.)".to_string(),
                Some(PetMood::Eating),
            );
        } else {
            self.set_pet_reaction(
                PetAction::Feed,
                format!(
                    "💬 \"Finishes the {} happily. 'That hit the spot, {caretaker}. Thank you.'\"",
                    food_name
                ),
                "💭 (Feeling: Warm food settling comfortably into the stomach.)".to_string(),
                Some(PetMood::Eating),
            );
        }

        self.refresh_attention();
        self.log_action(msg.clone());
        msg
    }
}
