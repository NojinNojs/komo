use super::*;
use chrono::Utc;

impl Pet {
    /// Advance the 3-step Burial & Vigil Rite (`[B]`) when a companion has died.
    pub fn hold_vigil_step(&mut self) -> String {
        let caretaker = self.caretaker_label().to_string();
        if self.is_alive {
            return self.refuse(format!(
                "🕯 {} is alive and well! Vigil rites are only for fallen companions.",
                self.name
            ));
        }
        if self.buried {
            return self.refuse(format!(
                "🪦 {} has already been laid to rest in the Memorial Hall. Press [N] to adopt.",
                self.name
            ));
        }

        self.vigil_steps_completed = (self.vigil_steps_completed + 1).min(3);
        self.last_updated = Utc::now();

        let msg = match self.vigil_steps_completed {
            1 => {
                self.poop_count = 0;
                self.cleanliness = 100;
                self.set_pet_reaction(
                    PetAction::Clean,
                    format!(
                        "💬 \"[ Vigil 1/3 ] You wash the habitat floor and fold {}'s woven mat.\"",
                        self.name
                    ),
                    "💭 (Vigil Step 1/3: Cleaning the room in quiet remembrance. Press [B] again.)"
                        .to_string(),
                    Some(PetMood::PassedOut),
                );
                format!(
                    "🕯 Vigil Step 1/3: Cleaned the habitat and folded {}'s mat. Press [B] for Step 2.",
                    self.name
                )
            }
            2 => {
                self.set_pet_reaction(
                    PetAction::Talk,
                    format!(
                        "💬 \"[ Vigil 2/3 ] You carve {}'s name and {} lived onto a memorial stone.\"",
                        self.name,
                        format_duration_human(self.age_seconds)
                    ),
                    "💭 (Vigil Step 2/3: Inscribing the memorial stone. Press [B] once more to finish.)"
                        .to_string(),
                    Some(PetMood::PassedOut),
                );
                format!(
                    "🪦 Vigil Step 2/3: Inscribed {}'s memorial stone. Press [B] for the final vow.",
                    self.name
                )
            }
            _ => {
                self.buried = true;
                self.set_pet_reaction(
                    PetAction::Talk,
                    format!(
                        "💬 \"[ Epitaph ] Here rests {}, cherished by {}. You may now adopt again [N].\"",
                        self.name, caretaker
                    ),
                    "💭 (Laid to Rest: Permanently recorded in the Sanctuary Memorial Hall.)"
                        .to_string(),
                    Some(PetMood::PassedOut),
                );
                format!(
                    "🪦 Vigil Complete (3/3): {} has been laid to rest in the Memorial Hall. You may now press [N] to adopt.",
                    self.name
                )
            }
        };
        self.log_action(msg.clone());
        msg
    }
}
