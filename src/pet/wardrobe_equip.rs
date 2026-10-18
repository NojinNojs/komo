use super::*;

impl Pet {
    pub fn is_coat_unlocked(&self, coat: CoatPreset) -> bool {
        coat == CoatPreset::Classic
            || self.level >= coat.unlock_level()
            || self.unlocked_coats.contains(&coat)
    }

    pub fn is_accessory_unlocked(&self, acc: Accessory) -> bool {
        acc == Accessory::None
            || self.level >= acc.unlock_level()
            || self.unlocked_accessories.contains(&acc)
    }

    pub fn equip_or_buy_coat(&mut self, coat: CoatPreset) -> String {
        if self.is_coat_unlocked(coat) {
            self.coat = coat;
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"The {}? I love it! This fits just right.\"",
                    coat.display_name()
                ),
                "💭 (Feeling: Proud of the new look, showing off for you.)".to_string(),
                Some(PetMood::Proud),
            );
            let msg = format!("👗 Equipped the {} coat!", coat.display_name());
            self.log_action(msg.clone());
            return msg;
        }
        if self.coins >= coat.unlock_cost() {
            self.coins -= coat.unlock_cost();
            self.unlocked_coats.push(coat);
            self.coat = coat;
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"A brand-new {}! Thank you, this is amazing!\"",
                    coat.display_name()
                ),
                "💭 (Feeling: Thrilled with the new coat.)".to_string(),
                Some(PetMood::Loved),
            );
            let msg = format!(
                "🛒 Bought & equipped the {} coat for {} coins!",
                coat.display_name(),
                coat.unlock_cost()
            );
            self.log_action(msg.clone());
            return msg;
        }
        if self.level < coat.unlock_level() {
            format!(
                "🔒 {} requires Lv.{} (you are Lv.{}).",
                coat.display_name(),
                coat.unlock_level(),
                self.level
            )
        } else {
            format!(
                "🔒 {} costs {} coins (you have {}).",
                coat.display_name(),
                coat.unlock_cost(),
                self.coins
            )
        }
    }

    pub fn equip_or_buy_accessory(&mut self, acc: Accessory) -> String {
        if self.is_accessory_unlocked(acc) {
            self.accessory = acc;
            self.set_pet_reaction(
                PetAction::PetLove,
                format!("💬 \"The {}? It suits me perfectly!\"", acc.display_name()),
                "💭 (Feeling: Delighted with the new accessory.)".to_string(),
                Some(PetMood::Proud),
            );
            let msg = format!("🎀 Equipped the {} accessory!", acc.display_name());
            self.log_action(msg.clone());
            return msg;
        }
        if self.coins >= acc.unlock_cost() {
            self.coins -= acc.unlock_cost();
            self.unlocked_accessories.push(acc);
            self.accessory = acc;
            self.set_pet_reaction(
                PetAction::PetLove,
                format!(
                    "💬 \"A brand-new {}! This is wonderful, thank you!\"",
                    acc.display_name()
                ),
                "💭 (Feeling: Ecstatic about the new accessory.)".to_string(),
                Some(PetMood::Loved),
            );
            let msg = format!(
                "🛒 Bought & equipped the {} accessory for {} coins!",
                acc.display_name(),
                acc.unlock_cost()
            );
            self.log_action(msg.clone());
            return msg;
        }
        if self.level < acc.unlock_level() {
            format!(
                "🔒 {} requires Lv.{} (you are Lv.{}).",
                acc.display_name(),
                acc.unlock_level(),
                self.level
            )
        } else {
            format!(
                "🔒 {} costs {} coins (you have {}).",
                acc.display_name(),
                acc.unlock_cost(),
                self.coins
            )
        }
    }
}
