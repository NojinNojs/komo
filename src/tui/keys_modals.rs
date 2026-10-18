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

    pub(super) fn handle_wardrobe_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Tab | KeyCode::Left | KeyCode::Right => {
                self.sound.play(SoundEffect::BeepMove);
                self.wardrobe_tab = 1 - self.wardrobe_tab;
            }
            KeyCode::Char(c @ ('1' | '2')) => {
                self.sound.play(SoundEffect::BeepMove);
                self.wardrobe_tab = (c as u8) - b'1';
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.sound.play(SoundEffect::BeepMove);
                if self.wardrobe_tab == 0 {
                    let n = CoatPreset::all().len();
                    self.wardrobe_coat_idx = (self.wardrobe_coat_idx + n - 1) % n;
                } else {
                    let n = Accessory::all().len();
                    self.wardrobe_acc_idx = (self.wardrobe_acc_idx + n - 1) % n;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.sound.play(SoundEffect::BeepMove);
                if self.wardrobe_tab == 0 {
                    let n = CoatPreset::all().len();
                    self.wardrobe_coat_idx = (self.wardrobe_coat_idx + 1) % n;
                } else {
                    let n = Accessory::all().len();
                    self.wardrobe_acc_idx = (self.wardrobe_acc_idx + 1) % n;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let msg = if self.wardrobe_tab == 0 {
                    self.pet
                        .equip_or_buy_coat(CoatPreset::all()[self.wardrobe_coat_idx])
                } else {
                    self.pet
                        .equip_or_buy_accessory(Accessory::all()[self.wardrobe_acc_idx])
                };
                if msg.contains("🔒") {
                    self.sound.play(SoundEffect::ErrorBuzz);
                } else {
                    self.sound.play(SoundEffect::BeepConfirm);
                }
                self.save_now();
            }
            KeyCode::Esc | KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.wardrobe_open = false;
            }
            _ => {}
        }
    }

    pub(super) fn handle_photo_booth_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left | KeyCode::Up => {
                self.sound.play(SoundEffect::BeepMove);
                self.photo_ratio = self.photo_ratio.prev();
            }
            KeyCode::Right | KeyCode::Down | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.photo_ratio = self.photo_ratio.next();
            }
            KeyCode::Char(c @ '1'..='4') => {
                self.sound.play(SoundEffect::BeepMove);
                self.photo_ratio = ShareRatio::all()[(c as usize) - ('1' as usize)];
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let out = default_share_output_path(&self.pet, self.photo_ratio);
                match save_share_card_png(&self.pet, self.photo_ratio, &out) {
                    Ok(()) => {
                        self.sound.play(SoundEffect::BeepConfirm);
                        self.photo_status = Some(format!("📸 Saved PNG card -> {}", out.display()));
                    }
                    Err(e) => {
                        self.sound.play(SoundEffect::ErrorBuzz);
                        self.photo_status = Some(format!("! Save failed: {e}"));
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.photo_booth_open = false;
            }
            _ => {}
        }
    }
}
