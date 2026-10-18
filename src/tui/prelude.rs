//! Shared imports for the `tui` submodules (glob-imported via `use super::*`).
#![allow(unused_imports)]

pub(super) use std::io;
pub(super) use std::time::{Duration, Instant};

pub(super) use chrono::Utc;
pub(super) use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub(super) use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame, Terminal,
};

pub(super) use crate::minigame::{GameMode, MiniGame};
pub(super) use crate::pet::{
    format_duration_human, format_hearts, AbsenceTier, Accessory, AttentionCall, CoatPreset,
    EvolutionStage, FoodChoice, LcdIcon, OfflineReport, Pet, PetSpecies, Sanctuary, ShellTheme,
    WelcomeHostEvent, CURRENT_APP_VERSION, WHATS_NEW_FEATURES,
};
pub(super) use crate::senses::sense_environment;
pub(super) use crate::share::{default_share_output_path, save_share_card_png, ShareRatio};
pub(super) use crate::sound::{SoundEffect, SoundEngine};
pub(super) use crate::sprites::{
    mood_color, render_colored_preview, render_colored_sprite, sky_scene, species_palette,
    speech_bubble, stage_color, theme_color,
};
pub(super) use crate::storage::{save_sanctuary, StoragePaths};
