use super::*;

impl App {
    pub(super) fn handle_roster_key(&mut self, code: KeyCode) {
        let count = self.sanctuary.pets.len().max(1);
        match code {
            KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                self.sound.play(SoundEffect::BeepMove);
                self.roster_selected_idx = (self.roster_selected_idx + count - 1) % count;
            }
            KeyCode::Down | KeyCode::Right | KeyCode::Char('j') | KeyCode::Tab => {
                self.sound.play(SoundEffect::BeepMove);
                self.roster_selected_idx = (self.roster_selected_idx + 1) % count;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.switch_to_roster_pet();
                self.screen = AppScreen::Habitat;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => self.start_adopting(),
            KeyCode::Esc | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Char('q') => {
                self.sound.play(SoundEffect::BeepMove);
                self.screen = AppScreen::Habitat;
            }
            _ => {}
        }
    }

    pub(super) fn handle_minigame_key(&mut self, code: KeyCode) {
        let mut sound_to_play: Option<SoundEffect> = None;
        let finished = {
            let Some(game) = self.minigame.as_mut() else {
                self.screen = AppScreen::Habitat;
                return;
            };
            match code {
                KeyCode::Char('1') => {
                    game.switch_mode(GameMode::SnackCatcher);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Char('2') => {
                    game.switch_mode(GameMode::MindReader);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Char('3') => {
                    game.switch_mode(GameMode::DashRunner);
                    sound_to_play = Some(SoundEffect::BeepConfirm);
                }
                KeyCode::Tab => {
                    let next = game.mode.next();
                    game.switch_mode(next);
                    sound_to_play = Some(SoundEffect::BeepMove);
                }
                KeyCode::Up | KeyCode::Char(' ') | KeyCode::Char('w') | KeyCode::Char('W') => {
                    game.jump();
                    sound_to_play = Some(SoundEffect::BeepMove);
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('h') => {
                    let wins_before = game.mr_wins;
                    game.move_left();
                    if game.mode == GameMode::MindReader {
                        sound_to_play = Some(if game.mr_wins > wins_before {
                            SoundEffect::PlayWin
                        } else {
                            SoundEffect::PlayHit
                        });
                    }
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('l') => {
                    let wins_before = game.mr_wins;
                    game.move_right();
                    if game.mode == GameMode::MindReader {
                        sound_to_play = Some(if game.mr_wins > wins_before {
                            SoundEffect::PlayWin
                        } else {
                            SoundEffect::PlayHit
                        });
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    game.ticks_remaining = 0;
                }
                _ => {}
            }
            game.is_finished()
        };
        if let Some(sfx) = sound_to_play {
            self.sound.play(sfx);
        }
        if finished {
            self.finish_minigame();
        }
    }

    pub(super) fn finish_minigame(&mut self) {
        if let Some(game) = self.minigame.take() {
            if game.score > 0 || game.coins_earned > 0 || game.xp_earned > 0 {
                let stage_before = self.pet.stage;
                let _ =
                    self.pet
                        .apply_minigame_reward(game.score, game.coins_earned, game.xp_earned);
                if self.pet.stage != stage_before {
                    self.sound.play(SoundEffect::LevelUpFanfare);
                } else {
                    self.sound.play(SoundEffect::PlayWin);
                }
            }
        }
        self.screen = AppScreen::Habitat;
        self.save_now();
    }
}
