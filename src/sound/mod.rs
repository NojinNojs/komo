//! 8-Bit Retro Chiptune Piezo Sound Synthesizer (`src/sound.rs`) for **Mochigotchi**.
//!
//! Synthesizes authentic 1990s Tamagotchi square-wave melodies into standard
//! 16-bit mono 22,050 Hz `.wav` files in pure Rust (zero extra audio crate
//! dependencies!) and plays them non-blockingly in the background via PipeWire
//! (`pw-play`), PulseAudio (`paplay`), or ALSA (`aplay`).
//!
//! Press `[U]` at any time in the TUI to toggle sound effects `ON` / `OFF`.

use std::fs;
use std::path::{Path, PathBuf};
#[cfg(not(test))]
use std::process::{Command, Stdio};

const SAMPLE_RATE: u32 = 22_050;

/// Distinct 8-bit chiptune sound cues for every Tamagotchi event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundEffect {
    /// Crisp 1-note cursor tick (`880 Hz`) when navigating menus/bezel.
    BeepMove,
    /// Classic 2-note Tamagotchi confirm chirp (`C6 -> E6`).
    BeepConfirm,
    /// Cozy rising warmth chord (`C5 -> E5 -> G5`).
    WarmEgg,
    /// Joyful 6-note Egg Hatch fanfare (`C5-E5-G5-C6-G5-C6`).
    HatchFanfare,
    /// Cheerful snack munching chime (`E5-G5-A5-G5`).
    EatChime,
    /// Arcade point / round win jingle (`G5-B5-D6-G6`).
    PlayWin,
    /// Low retro buzz (`220 -> 165 Hz`) when hitting a bomb/hurdle.
    PlayHit,
    /// Triumphant evolution / level-up fanfare (`C5-D5-E5-G5-C6-E6`).
    LevelUpFanfare,
    /// Classic double Tamagotchi piezo call (`G6 - pause - G6`).
    AttentionBeep,
    /// Short refusal buzz (`E4 -> C4`) when full, asleep, or on cooldown.
    ErrorBuzz,
    /// Cute companion voice blip (`G5 -> B5 -> A5`) when chatting!
    ChatChirp,
}

impl SoundEffect {
    pub fn all() -> &'static [SoundEffect] {
        &[
            Self::BeepMove,
            Self::BeepConfirm,
            Self::WarmEgg,
            Self::HatchFanfare,
            Self::EatChime,
            Self::PlayWin,
            Self::PlayHit,
            Self::LevelUpFanfare,
            Self::AttentionBeep,
            Self::ErrorBuzz,
            Self::ChatChirp,
        ]
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Self::BeepMove => "beep_move",
            Self::BeepConfirm => "beep_confirm",
            Self::WarmEgg => "warm_egg",
            Self::HatchFanfare => "hatch_fanfare",
            Self::EatChime => "eat_chime",
            Self::PlayWin => "play_win",
            Self::PlayHit => "play_hit",
            Self::LevelUpFanfare => "level_up",
            Self::AttentionBeep => "attention_beep",
            Self::ErrorBuzz => "error_buzz",
            Self::ChatChirp => "chat_chirp",
        }
    }

    pub fn visual_badge(&self) -> &'static str {
        match self {
            Self::BeepMove => "♪ tick",
            Self::BeepConfirm => "♫ chirp!",
            Self::WarmEgg => "🎶 warm~",
            Self::HatchFanfare => "🎺 fanfare!!",
            Self::EatChime => "🎵 nom-nom♪",
            Self::PlayWin => "🎶 win!★",
            Self::PlayHit => "💥 bonk!",
            Self::LevelUpFanfare => "🎺 evolve!!",
            Self::AttentionBeep => "🔔 peep-peep!",
            Self::ErrorBuzz => "💢 buzz",
            Self::ChatChirp => "💬 pi-po♪",
        }
    }

    /// Sequence of `(frequency_hz, duration_ms)` notes (`0.0` Hz = rest).
    pub fn notes(&self) -> &'static [(f32, u32)] {
        match self {
            Self::BeepMove => &[(880.0, 22)],
            Self::BeepConfirm => &[(1046.5, 38), (1318.5, 55)],
            Self::WarmEgg => &[(523.25, 55), (659.25, 55), (783.99, 85)],
            Self::HatchFanfare => &[
                (523.25, 65),
                (659.25, 65),
                (783.99, 65),
                (1046.5, 110),
                (783.99, 60),
                (1046.5, 160),
            ],
            Self::EatChime => &[(659.25, 48), (783.99, 48), (880.0, 55), (783.99, 75)],
            Self::PlayWin => &[(783.99, 45), (987.77, 45), (1174.66, 55), (1567.98, 110)],
            Self::PlayHit => &[(220.0, 65), (164.81, 95)],
            Self::LevelUpFanfare => &[
                (523.25, 55),
                (587.33, 55),
                (659.25, 55),
                (783.99, 65),
                (1046.5, 80),
                (1318.5, 150),
            ],
            Self::AttentionBeep => &[(1567.98, 65), (0.0, 35), (1567.98, 75)],
            Self::ErrorBuzz => &[(329.63, 55), (261.63, 85)],
            Self::ChatChirp => &[(783.99, 38), (987.77, 42), (880.0, 55)],
        }
    }
}

/// Synthesize a sequence of square-wave chiptune notes into a complete RIFF WAV byte buffer.
pub fn synthesize_wav(notes: &[(f32, u32)]) -> Vec<u8> {
    let mut samples: Vec<i16> = Vec::new();
    for &(freq, dur_ms) in notes {
        let count = ((SAMPLE_RATE * dur_ms) / 1000) as usize;
        for i in 0..count {
            if freq <= 1.0 {
                samples.push(0);
                continue;
            }
            let t = i as f32 / SAMPLE_RATE as f32;
            // Soft attack & exponential decay envelope so piezo tones sound warm, never harsh
            let progress = i as f32 / count.max(1) as f32;
            let attack = (i as f32 / 48.0).min(1.0);
            let decay = 1.0 - 0.55 * progress;
            let amp = 4_200.0 * attack * decay;

            // Classic 50% duty-cycle square wave with a subtle 2nd harmonic
            let phase = (t * freq).fract();
            let square = if phase < 0.5 { 1.0f32 } else { -1.0f32 };
            let sample = (square * amp) as i16;
            samples.push(sample);
        }
        // Tiny 8ms gap between notes for crisp articulation
        let gap = ((SAMPLE_RATE * 8) / 1000) as usize;
        samples.extend(std::iter::repeat_n(0i16, gap));
    }

    let data_bytes = (samples.len() * 2) as u32;
    let riff_size = 36 + data_bytes;
    let byte_rate = SAMPLE_RATE * 2;

    let mut wav = Vec::with_capacity((44 + data_bytes) as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&riff_size.to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // PCM chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format = 1
    wav.extend_from_slice(&1u16.to_le_bytes()); // Mono = 1 channel
    wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes()); // Block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // 16 bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_bytes.to_le_bytes());
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

mod engine;
pub use engine::*;
