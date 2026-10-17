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

        if self.offline_report.is_some() {
            if matches!(
                code,
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') | KeyCode::Char('q')
            ) {
                self.sound.play(SoundEffect::BeepConfirm);
                self.offline_report = None;
            }
            return;
        }

        if self.show_help {
            if matches!(
                code,
                KeyCode::Esc
                    | KeyCode::Enter
                    | KeyCode::Char('?')
                    | KeyCode::Char('q')
                    | KeyCode::Char(' ')
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
            KeyCode::Char('1') => {
                let _ = self.pet.answer_dialogue(1);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Char('2') => {
                let _ = self.pet.answer_dialogue(2);
                self.sound.play(SoundEffect::ChatChirp);
                self.talk_open = false;
                self.save_now();
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('y') | KeyCode::Char('Y') => {
                let choice = self.talk_selected_idx;
                let _ = self.pet.answer_dialogue(choice);
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
}
