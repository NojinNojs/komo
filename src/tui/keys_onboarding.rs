use super::*;

impl App {
    pub(super) fn handle_onboarding_key(&mut self, code: KeyCode) {
        let species_count = PetSpecies::all().len();
        let theme_count = ShellTheme::all().len();
        let stage_count = EvolutionStage::all().len();

        match self.onboarding_step {
            OnboardingStep::SelectSpecies => match code {
                KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_species_idx =
                        (self.selected_species_idx + species_count - 1) % species_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_species_idx = (self.selected_species_idx + 1) % species_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') | KeyCode::Char(' ') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.preview_auto_morph = false;
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.preview_auto_morph = !self.preview_auto_morph;
                }
                KeyCode::Enter => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.onboarding_step = OnboardingStep::SelectTheme;
                    self.onboarding_name_input = PetSpecies::all()[self.selected_species_idx]
                        .default_name()
                        .to_string();
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    if self.sanctuary.hatched_count() > 0 || self.adopting_new {
                        self.adopting_new = false;
                        self.screen = AppScreen::Habitat;
                    } else {
                        self.should_quit = true;
                    }
                }
                _ => {}
            },
            OnboardingStep::SelectTheme => match code {
                KeyCode::Up | KeyCode::Left | KeyCode::Char('k') | KeyCode::Char('h') => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_theme_idx =
                        (self.selected_theme_idx + theme_count - 1) % theme_count;
                }
                KeyCode::Down
                | KeyCode::Right
                | KeyCode::Char('j')
                | KeyCode::Char('l')
                | KeyCode::Tab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.selected_theme_idx = (self.selected_theme_idx + 1) % theme_count;
                }
                KeyCode::Char('v') | KeyCode::Char('V') => {
                    self.preview_stage_idx = (self.preview_stage_idx + 1) % stage_count;
                }
                KeyCode::BackTab | KeyCode::Esc => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.onboarding_step = OnboardingStep::SelectSpecies;
                }
                KeyCode::Enter => {
                    self.sound.play(SoundEffect::BeepConfirm);
                    self.onboarding_step = OnboardingStep::EnterName;
                }
                _ => {}
            },
            OnboardingStep::EnterName => match code {
                KeyCode::Enter => self.finish_adoption(),
                KeyCode::Tab | KeyCode::Up | KeyCode::Down => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.onboarding_edit_caretaker = !self.onboarding_edit_caretaker;
                }
                KeyCode::Backspace => {
                    if self.onboarding_edit_caretaker {
                        self.onboarding_caretaker_input.pop();
                    } else {
                        self.onboarding_name_input.pop();
                    }
                }
                KeyCode::Esc | KeyCode::BackTab => {
                    self.sound.play(SoundEffect::BeepMove);
                    self.onboarding_step = OnboardingStep::SelectTheme;
                }
                KeyCode::Char(c) => {
                    if self.onboarding_edit_caretaker {
                        if self.onboarding_caretaker_input.chars().count() < 18 {
                            self.onboarding_caretaker_input.push(c);
                        }
                    } else if self.onboarding_name_input.chars().count() < 18 {
                        self.onboarding_name_input.push(c);
                    }
                }
                _ => {}
            },
        }
    }

    pub(super) fn finish_adoption(&mut self) {
        let species = PetSpecies::all()[self.selected_species_idx];
        let theme = ShellTheme::all()[self.selected_theme_idx];
        let name = self.onboarding_name_input.trim().to_string();
        let caretaker = self.onboarding_caretaker_input.trim().to_string();
        let mut new_pet = Pet::new_custom(&name, species, theme);
        if !caretaker.is_empty() {
            new_pet.caretaker_title = caretaker.clone();
        }

        if self.adopting_new {
            self.sanctuary.add_pet(new_pet.clone());
            if !caretaker.is_empty() {
                self.sanctuary.set_caretaker_title_all(&caretaker);
            }
        } else {
            self.sanctuary = Sanctuary::new_with_pet(new_pet.clone());
        }

        self.pet = self.sanctuary.active_pet().clone();
        self.roster_selected_idx = self.sanctuary.active_index;
        self.adopting_new = false;
        self.screen = AppScreen::Habitat;
        self.sound.play(SoundEffect::HatchFanfare);
        self.save_now();
    }
}
