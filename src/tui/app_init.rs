use super::*;

impl App {
    pub fn new(pet: Pet, paths: StoragePaths) -> Self {
        Self::new_with_sanctuary(Sanctuary::new_with_pet(pet), paths, false)
    }

    pub fn new_with_sanctuary(
        sanctuary: Sanctuary,
        paths: StoragePaths,
        start_onboarding: bool,
    ) -> Self {
        let pet = sanctuary.active_pet().clone();
        let caretaker_default = pet.caretaker_label().to_string();
        let selected_species_idx = PetSpecies::all()
            .iter()
            .position(|s| *s == pet.species)
            .unwrap_or(0);
        let selected_theme_idx = ShellTheme::all()
            .iter()
            .position(|t| *t == pet.theme)
            .unwrap_or(0);
        Self {
            pet,
            sanctuary,
            paths,
            sound: SoundEngine::default(),
            screen: if start_onboarding {
                AppScreen::Onboarding
            } else {
                AppScreen::Habitat
            },
            onboarding_step: OnboardingStep::SelectSpecies,
            selected_species_idx,
            selected_theme_idx,
            preview_stage_idx: 4,
            preview_auto_morph: true,
            roster_selected_idx: 0,
            selected_icon_idx: 0,
            pantry_open: false,
            pantry_selected_idx: 0,
            talk_open: false,
            talk_selected_idx: 1,
            show_meter: false,
            meter_page: 0,
            show_help: false,
            offline_report: None,
            onboarding_name_input: String::new(),
            onboarding_caretaker_input: caretaker_default,
            onboarding_edit_caretaker: false,
            adopting_new: false,
            tick_count: 0,
            recent_key_ticks: 0,
            key_burst: 0,
            should_quit: false,
            minigame: None,
            rename_buffer: None,
            caretaker_buffer: None,
            last_storage_status: "💾 saved".to_string(),
        }
    }

    pub fn new_onboarding(paths: StoragePaths) -> Self {
        let mut app =
            Self::new_with_sanctuary(Sanctuary::new_with_pet(Pet::new("Mochi")), paths, true);
        app.preview_stage_idx = 0;
        app.onboarding_name_input = "Mochi".to_string();
        app.onboarding_caretaker_input = "Keeper".to_string();
        app.onboarding_edit_caretaker = false;
        app
    }

    pub fn save_now(&mut self) {
        self.sanctuary.record_memorial_if_buried(&self.pet);
        self.sanctuary.sync_active(&self.pet);
        match save_sanctuary(&self.sanctuary, &self.paths) {
            Ok(()) => self.last_storage_status = "💾 saved".to_string(),
            Err(err) => self.last_storage_status = format!("save err: {err}"),
        }
    }

    pub(super) fn play_feedback_for_message(&mut self, msg: &str, evolved_or_hatched: bool) {
        if evolved_or_hatched
            || msg.contains("CRACK!")
            || msg.contains("Evolution!")
            || msg.contains("PHOENIX REVIVAL")
        {
            self.sound.play(SoundEffect::HatchFanfare);
        } else if msg.contains("Toothache")
            || msg.contains("⏳")
            || msg.contains("unhatched")
            || msg.contains("asleep")
            || msg.contains("sleeping")
            || msg.contains("completely full")
            || msg.contains("maxed out")
            || msg.contains("Not enough coins")
            || msg.contains("Unfair scold")
            || msg.contains("too tired")
            || msg.contains("too hungry")
            || msg.contains("passed out")
        {
            self.sound.play(SoundEffect::ErrorBuzz);
        } else if msg.contains("Fed ") {
            self.sound.play(SoundEffect::EatChime);
        } else if msg.contains("warm ") {
            self.sound.play(SoundEffect::WarmEgg);
        } else if msg.contains("Played ") || msg.contains("training") {
            self.sound.play(SoundEffect::PlayWin);
        } else {
            self.sound.play(SoundEffect::BeepConfirm);
        }
    }
}
