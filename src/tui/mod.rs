//! Spacious, Balanced Retro Handheld LCD Interface for **Mochigotchi v1.0.0**:
//! - **Structured 3-Zone Vertical Layouts** inside every LCD screen (Top Header
//!   with breathing spacer, Center 40×22 3/4 Side-Profile Pixel Art Stage, and
//!   Bottom Vitals/Lore & Controls anchored cleanly at the bottom of the LCD!).
//! - **15 Rich Emotions (`PetMood`)** displayed live with facial expressions &
//!   autonomous strolling across the habitat floor.
//! - **Direct First-Person Pet Reactions**: when you feed, cuddle, scold, or spam
//!   a button, your companion talks back to you immediately in the `💬` & `💭`
//!   bubbles!
//! - **Realistic Starvation, Sickness, Spirit Memorial (`👻`) & Phoenix Revival (`[H]`)**.

mod prelude;
use prelude::*;
mod app_init;
mod app_nav;
mod chrome;
mod keys_arcade_roster;
mod keys_dispatch;
mod keys_habitat;
mod keys_modals;
mod keys_onboarding;
mod runner;
mod screen_habitat;
mod screen_habitat_footer;
mod screen_hosting;
mod screen_meter;
mod screen_meter_pages;
mod screen_onboarding;
mod screen_onboarding_steps;
mod screen_overlays;
mod screen_roster_arcade;
mod screen_talk;
mod screen_wardrobe_photo;
mod state;
pub use chrome::*;
pub use runner::*;
use screen_habitat::*;
use screen_habitat_footer::*;
use screen_hosting::*;
use screen_meter::*;
use screen_meter_pages::*;
use screen_onboarding::*;
use screen_onboarding_steps::*;
use screen_overlays::*;
use screen_roster_arcade::*;
use screen_talk::*;
use screen_wardrobe_photo::*;
pub use state::*;
