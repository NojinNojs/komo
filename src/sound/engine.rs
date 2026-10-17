use super::*;

/// Manages chiptune synthesis, caching, and non-blocking background playback.
#[derive(Debug, Clone)]
pub struct SoundEngine {
    pub enabled: bool,
    pub last_cue: Option<SoundEffect>,
    cache_dir: PathBuf,
}

impl Default for SoundEngine {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SoundEngine {
    pub fn new(enabled: bool) -> Self {
        let cache_dir = std::env::temp_dir().join("mochigotchi_sfx_v1");
        Self {
            enabled,
            last_cue: None,
            cache_dir,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        if self.enabled {
            self.play(SoundEffect::BeepConfirm);
        }
        self.enabled
    }

    pub fn status_badge(&self) -> String {
        if !self.enabled {
            "🔇 Sound:OFF".to_string()
        } else if let Some(cue) = self.last_cue {
            format!("🔊 {}", cue.visual_badge())
        } else {
            "🔊 Sound:ON".to_string()
        }
    }

    fn ensure_wav_file(&self, effect: SoundEffect) -> Option<PathBuf> {
        let _ = fs::create_dir_all(&self.cache_dir);
        let path = self.cache_dir.join(format!("{}.wav", effect.slug()));
        if !path.exists() {
            let bytes = synthesize_wav(effect.notes());
            if fs::write(&path, bytes).is_err() {
                return None;
            }
        }
        Some(path)
    }

    /// Trigger a retro chiptune sound effect non-blockingly.
    /// In `#[cfg(test)]`, records `last_cue` and synthesizes the WAV buffer
    /// without spawning external audio processes so unit tests stay fast and silent.
    pub fn play(&mut self, effect: SoundEffect) {
        self.last_cue = Some(effect);
        if !self.enabled {
            return;
        }
        let Some(wav_path) = self.ensure_wav_file(effect) else {
            return;
        };

        #[cfg(not(test))]
        spawn_audio_player(&wav_path);

        #[cfg(test)]
        let _ = wav_path;
    }
}

#[cfg(not(test))]
fn spawn_audio_player(wav_path: &Path) {
    let candidates = [
        "/run/current-system/sw/bin/pw-play",
        "pw-play",
        "/run/current-system/sw/bin/paplay",
        "paplay",
        "/run/current-system/sw/bin/aplay",
        "aplay",
    ];
    for bin in candidates {
        if Command::new(bin)
            .arg(wav_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_ok()
        {
            return;
        }
    }
}

#[allow(dead_code)]
fn _keep_path_import(_p: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_sound_effects_synthesize_valid_riff_wav() {
        let mut engine = SoundEngine::new(true);
        for &fx in SoundEffect::all() {
            let wav = synthesize_wav(fx.notes());
            assert!(wav.len() > 44, "{fx:?} wav too short");
            assert_eq!(&wav[0..4], b"RIFF");
            assert_eq!(&wav[8..12], b"WAVE");
            engine.play(fx);
            assert_eq!(engine.last_cue, Some(fx));
            assert!(engine.status_badge().contains(fx.visual_badge()));
        }
        assert!(!engine.toggle());
        assert_eq!(engine.status_badge(), "🔇 Sound:OFF");
    }
}
