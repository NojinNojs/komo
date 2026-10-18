use super::*;

impl App {
    pub(super) fn switch_to_roster_pet(&mut self) {
        self.sanctuary.sync_active(&self.pet);
        if self.sanctuary.switch_pet(self.roster_selected_idx) {
            self.pet = self.sanctuary.active_pet().clone();
            self.sound.play(SoundEffect::BeepConfirm);
            self.save_now();
        }
    }

    pub(super) fn cycle_pet(&mut self, forward: bool) {
        self.sanctuary.sync_active(&self.pet);
        if forward {
            self.sanctuary.next_pet();
        } else {
            self.sanctuary.prev_pet();
        }
        self.pet = self.sanctuary.active_pet().clone();
        self.roster_selected_idx = self.sanctuary.active_index;
        self.sound.play(SoundEffect::BeepMove);
        self.save_now();
    }

    pub(super) fn start_adopting(&mut self) {
        self.sanctuary.record_memorial_if_buried(&self.pet);
        self.sanctuary.sync_active(&self.pet);
        if let Err(reason) = self.sanctuary.can_adopt_new() {
            self.sound.play(SoundEffect::ErrorBuzz);
            self.pet.log_action(reason.clone());
            self.pet.set_pet_reaction(
                crate::pet::PetAction::Heal,
                format!("💬 \"{reason}\""),
                "💭 (Accountability: A fallen companion must be revived [H] or laid to rest [B] before adopting anew.)".to_string(),
                Some(crate::pet::PetMood::PassedOut),
            );
            return;
        }
        self.adopting_new = true;
        self.pantry_open = false;
        self.talk_open = false;
        self.wardrobe_open = false;
        self.photo_booth_open = false;
        self.show_meter = false;
        self.show_help = false;
        self.screen = AppScreen::Onboarding;
        self.onboarding_step = OnboardingStep::SelectSpecies;
        self.selected_species_idx = 0;
        self.selected_theme_idx = 0;
        self.preview_stage_idx = 0;
        self.preview_auto_morph = true;
        self.onboarding_name_input = PetSpecies::curated()[0].default_name().to_string();
        self.onboarding_caretaker_input = self.pet.caretaker_label().to_string();
        self.onboarding_edit_caretaker = false;
        self.sound.play(SoundEffect::BeepConfirm);
    }

    pub(super) fn launch_minigame(&mut self, mode: GameMode) {
        let refusal = if !self.pet.is_alive {
            Some(format!(
                "👻 {} has passed out! Press [H] to revive them first!",
                self.pet.name
            ))
        } else if self.pet.stage == EvolutionStage::Egg {
            Some("🥚 The egg can't play yet — warm it first!".to_string())
        } else if self.pet.is_sleeping {
            Some(format!("💤 {} is asleep — wake them first!", self.pet.name))
        } else if self.pet.has_toothache || self.pet.is_sick {
            Some(format!(
                "🤒 {} needs Medicine [H] before playing games!",
                self.pet.name
            ))
        } else if self.pet.energy < 15 {
            Some(format!("⚡ {} is too tired for the arcade.", self.pet.name))
        } else {
            None
        };

        match refusal {
            Some(msg) => {
                self.sound.play(SoundEffect::ErrorBuzz);
                self.pet.log_action(msg);
            }
            None => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.minigame = Some(MiniGame::new_with_mode(mode, 44, 8, 120));
                self.screen = AppScreen::MiniGame;
            }
        }
    }

    pub(super) fn activate_selected_lcd_icon(&mut self) {
        let icon = LcdIcon::selectable()[self.selected_icon_idx % LcdIcon::selectable().len()];
        match icon {
            LcdIcon::Food => {
                if self.pet.stage == EvolutionStage::Egg {
                    let stage_before = self.pet.stage;
                    let msg = self.pet.warm_egg();
                    let hatched = self.pet.stage != stage_before;
                    self.play_feedback_for_message(&msg, hatched);
                    self.save_now();
                } else {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.pantry_open = true;
                }
            }
            LcdIcon::Lights => {
                let msg = self.pet.toggle_light();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Play => {
                self.launch_minigame(GameMode::MindReader);
            }
            LcdIcon::Medicine => {
                let msg = self.pet.heal();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Bath => {
                let msg = self.pet.clean();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Meter => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_meter = true;
                self.meter_page = 0;
            }
            LcdIcon::Discipline => {
                let msg = if self.pet.attention_call == AttentionCall::Crying {
                    self.pet.pet_love()
                } else {
                    self.pet.discipline_scold()
                };
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            LcdIcon::Talk => {
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = true;
                self.talk_selected_idx = 1;
            }
            LcdIcon::Attention => {}
        }
    }
}
