use super::*;

impl App {
    pub fn handle_key(&mut self, code: KeyCode) {
        self.recent_key_ticks = 8;
        self.key_burst = self.key_burst.saturating_add(1).min(12);

        if self.rename_buffer.is_none()
            && self.caretaker_buffer.is_none()
            && !(self.screen == AppScreen::Onboarding
                && self.onboarding_step == OnboardingStep::EnterName)
            && matches!(code, KeyCode::Char('u') | KeyCode::Char('U'))
        {
            self.sound.toggle();
            return;
        }

        if self.show_changelog {
            if matches!(
                code,
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ' | 'q' | 'n' | 'N')
            ) {
                self.sound.play(SoundEffect::BeepConfirm);
                self.show_changelog = false;
                self.sanctuary.last_seen_version = CURRENT_APP_VERSION.to_string();
                self.save_now();
            }
            return;
        }

        if self.offline_report.is_some() {
            self.handle_offline_report_key(code);
            return;
        }

        if self.show_help {
            if matches!(
                code,
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('?' | 'q' | ' ')
            ) {
                self.sound.play(SoundEffect::BeepMove);
                self.show_help = false;
            }
            return;
        }

        if self.show_meter {
            match code {
                KeyCode::Right
                | KeyCode::Down
                | KeyCode::Char(' ')
                | KeyCode::Enter
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.meter_page = (self.meter_page + 1) % 4;
                }
                KeyCode::Left | KeyCode::Up => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.meter_page = (self.meter_page + 3) % 4;
                }
                KeyCode::Esc | KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('q') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.show_meter = false;
                }
                _ => {}
            }
            return;
        }

        if self.rename_buffer.is_some() {
            self.handle_rename_key(code);
            return;
        }

        if self.caretaker_buffer.is_some() {
            self.handle_caretaker_key(code);
            return;
        }

        if self.wardrobe_open {
            self.handle_wardrobe_key(code);
            return;
        }

        if self.photo_booth_open {
            self.handle_photo_booth_key(code);
            return;
        }

        if self.talk_open {
            self.handle_talk_key(code);
            return;
        }

        if self.pantry_open {
            self.handle_pantry_key(code);
            return;
        }
        match self.screen {
            AppScreen::Onboarding => self.handle_onboarding_key(code),
            AppScreen::SanctuaryRoster => self.handle_roster_key(code),
            AppScreen::MiniGame => self.handle_minigame_key(code),
            AppScreen::Habitat => self.handle_habitat_key(code),
        }
    }

    pub(super) fn handle_talk_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.talk_selected_idx = if self.talk_selected_idx == 1 { 2 } else { 1 };
            }
            KeyCode::Char(c @ ('1' | '2')) => {
                let _ = self.pet.answer_dialogue((c as u8) - b'0');
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Enter | KeyCode::Char(' ' | 'y' | 'Y') => {
                let _ = self.pet.answer_dialogue(self.talk_selected_idx);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.talk_open = false;
            }
            _ => {}
        }
    }

    pub(super) fn handle_offline_report_key(&mut self, code: KeyCode) {
        if self.pet.pending_hosting.is_some() {
            match code {
                KeyCode::Up | KeyCode::Left => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.hosting_selected_idx = if self.hosting_selected_idx <= 1 {
                        3
                    } else {
                        self.hosting_selected_idx - 1
                    };
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.hosting_selected_idx = (self.hosting_selected_idx % 3) + 1;
                }
                KeyCode::Char(c @ ('1' | '2' | '3')) => {
                    let _ = self.pet.respond_welcome_hosting((c as u8) - b'0');
                    self.sound.play(SoundEffect::ChatChirp);
                    self.offline_report = None;
                    self.save_now();
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let _ = self.pet.respond_welcome_hosting(self.hosting_selected_idx);
                    self.sound.play(SoundEffect::ChatChirp);
                    self.offline_report = None;
                    self.save_now();
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.pet.pending_hosting = None;
                    self.offline_report = None;
                }
                _ => {}
            }
        } else if matches!(
            code,
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ' | 'q')
        ) {
            self.sound.play(SoundEffect::BeepConfirm);
            self.offline_report = None;
        }
    }
}
