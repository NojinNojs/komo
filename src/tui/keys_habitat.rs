use super::*;

impl App {
    pub(super) fn handle_habitat_key(&mut self, code: KeyCode) {
        let icon_count = LcdIcon::selectable().len();
        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.save_now();
                self.should_quit = true;
            }
            KeyCode::Left | KeyCode::Char(',') => {
                self.sound.play(SoundEffect::BeepMove);
                self.selected_icon_idx = (self.selected_icon_idx + icon_count - 1) % icon_count;
            }
            KeyCode::Right | KeyCode::Char('.') | KeyCode::Char('a') | KeyCode::Char('A') => {
                self.sound.play(SoundEffect::BeepMove);
                self.selected_icon_idx = (self.selected_icon_idx + icon_count + 1) % icon_count;
            }
            KeyCode::Enter => {
                self.activate_selected_lcd_icon();
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if !self.pet.is_alive {
                    let msg = self.pet.hold_vigil_step();
                    self.sanctuary.record_memorial_if_buried(&self.pet);
                    self.play_feedback_for_message(&msg, false);
                    self.save_now();
                } else {
                    self.activate_selected_lcd_icon();
                }
            }
            KeyCode::Char('?') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_help = true;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_meter = true;
                self.meter_page = 0;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if self.pet.pending_hosting.is_some() {
                    let _ = self.pet.respond_welcome_hosting(1);
                    self.save_now();
                }
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = true;
                self.talk_selected_idx = 1;
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.wardrobe_open = true;
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.photo_booth_open = true;
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                let msg = self.pet.toggle_light();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                let msg = self.pet.discipline_scold();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char(' ') => {
                let stage_before = self.pet.stage;
                let msg = if self.pet.stage == EvolutionStage::Egg {
                    self.pet.warm_egg()
                } else {
                    self.pet.pet_love()
                };
                let hatched = self.pet.stage != stage_before;
                self.play_feedback_for_message(&msg, hatched);
                self.save_now();
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('1') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.feed();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.pet.stage == EvolutionStage::Egg {
                    let msg = self.pet.feed();
                    self.play_feedback_for_message(&msg, false);
                } else {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.pantry_open = true;
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char('2') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.play();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('3') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.pet_love();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('4') => {
                let stage_before = self.pet.stage;
                let msg = self.pet.train();
                self.play_feedback_for_message(&msg, self.pet.stage != stage_before);
                self.save_now();
            }
            KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('5') => {
                let msg = self.pet.toggle_sleep();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('6') => {
                let msg = self.pet.clean();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('7') => {
                let msg = self.pet.heal();
                self.play_feedback_for_message(&msg, false);
                self.save_now();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.launch_minigame(GameMode::SnackCatcher);
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.sanctuary.sync_active(&self.pet);
                self.roster_selected_idx = self.sanctuary.active_index;
                self.screen = AppScreen::SanctuaryRoster;
            }
            KeyCode::Char('[') => self.cycle_pet(false),
            KeyCode::Char(']') => self.cycle_pet(true),
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_changelog = true;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.rename_buffer = Some(self.pet.name.clone());
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                self.caretaker_buffer = Some(self.pet.caretaker_label().to_string());
            }
            KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.pet.theme = self.pet.theme.next();
                self.save_now();
            }
            _ => {}
        }
    }
}
