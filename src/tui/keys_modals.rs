use super::*;

impl App {
    pub(super) fn handle_pantry_key(&mut self, code: KeyCode) {
        let choices = FoodChoice::all();
        let count = choices.len();
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_selected_idx = (self.pantry_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_selected_idx = (self.pantry_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('f') | KeyCode::Char('F') => {
                let choice = choices[self.pantry_selected_idx];
                let msg = self.pet.feed_choice(choice);
                self.play_feedback_for_message(&msg, false);
                self.pantry_open = false;
                self.save_now();
            }
            KeyCode::Char(c @ '1'..='5') => {
                let idx = (c as usize) - ('1' as usize);
                if let Some(&choice) = choices.get(idx) {
                    let msg = self.pet.feed_choice(choice);
                    self.play_feedback_for_message(&msg, false);
                    self.pantry_open = false;
                    self.save_now();
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.pantry_open = false;
            }
            _ => {}
        }
    }

    pub(super) fn handle_rename_key(&mut self, code: KeyCode) {
        let mut buffer = self.rename_buffer.take().unwrap_or_default();
        match code {
            KeyCode::Enter => {
                self.pet.rename(&buffer);
                self.sound.play(SoundEffect::BeepConfirm);
                self.save_now();
            }
            KeyCode::Esc => {}
            KeyCode::Backspace => {
                buffer.pop();
                self.rename_buffer = Some(buffer);
            }
            KeyCode::Char(c) if buffer.chars().count() < 18 => {
                buffer.push(c);
                self.rename_buffer = Some(buffer);
            }
            _ => self.rename_buffer = Some(buffer),
        }
    }

    pub(super) fn handle_caretaker_key(&mut self, code: KeyCode) {
        let mut buffer = self.caretaker_buffer.take().unwrap_or_default();
        match code {
            KeyCode::Enter => {
                self.pet.set_caretaker_title(&buffer);
                self.sanctuary.set_caretaker_title_all(&buffer);
                self.sound.play(SoundEffect::BeepConfirm);
                self.save_now();
            }
            KeyCode::Esc => {}
            KeyCode::Backspace => {
                buffer.pop();
                self.caretaker_buffer = Some(buffer);
            }
            KeyCode::Char(c) if buffer.chars().count() < 18 => {
                buffer.push(c);
                self.caretaker_buffer = Some(buffer);
            }
            _ => self.caretaker_buffer = Some(buffer),
        }
    }
}
