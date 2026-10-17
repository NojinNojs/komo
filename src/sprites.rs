//! ASCII art engine for **Mochigotchi**.
//!
//! Every companion has a hand-drawn multi-frame sprite for each of the five
//! [`EvolutionStage`]s. Frames are composited from a body template plus a
//! mood-driven expression, so every species × stage × mood combination gets
//! real, breathing, blinking animation.

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::style::Color;

/// Token replaced by a 3-character facial expression.
const EYES: &str = "{E}";
/// Token replaced by a 2-character mood effect (sparkles, zZ, hearts, ...).
const FX: &str = "{X}";

/// Body templates for every species and stage. `{E}` marks the face slot.
fn body_template(species: PetSpecies, stage: EvolutionStage) -> &'static [&'static str] {
    match (species, stage) {
        // ── 🐱 Neko Mochi ───────────────────────────────────────────────
        (PetSpecies::MochiCat, EvolutionStage::Egg) => &[
            "      .-------.",
            "     /  {E}  \\",
            "    |  ~~~~~~  |",
            "     \\________/",
            "      ({X})",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Baby) => &[
            "      /\\_/\\",
            "     ( {E} )",
            "      > ^ <",
            "      ({X})",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Teen) => &[
            "     /\\___/\\",
            "    (  {E}  )",
            "    (  =^=  )",
            "     (\")_(\")  {X}",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Adult) => &[
            "     /\\_____/\\",
            "    /  {E}   \\",
            "   ( ==  ^  == )",
            "    )         (  {X}",
            "   ( (__)   (__) )",
        ],
        (PetSpecies::MochiCat, EvolutionStage::CyberElder) => &[
            "   * /\\________/\\ *",
            "    /   {E}    \\",
            "   ( ==  ω  == )",
            "    )  ~★~★~  (   {X}",
            "   ( (_(____)_) )",
            "    ~~~  ~~~  ~~~",
        ],

        // ── 🐶 Boba Pup ─────────────────────────────────────────────────
        (PetSpecies::BobaPup, EvolutionStage::Egg) => &[
            "      .-------.",
            "     /  {E}  \\",
            "    |  ••••••  |",
            "     \\________/",
            "      ({X})",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Baby) => &[
            "      /\\__/\\",
            "     ( {E} )",
            "      /    \\",
            "      ({X})",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Teen) => &[
            "     / \\__/ \\",
            "    (  {E}  )",
            "     /  =  \\",
            "    (__)  (__)  {X}",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Adult) => &[
            "    /  \\____/  \\",
            "   (   {E}    )",
            "    \\   ==   /",
            "     |  ()  |   {X}",
            "    /        \\",
            "   (__)    (__)",
        ],
        (PetSpecies::BobaPup, EvolutionStage::CyberElder) => &[
            "   * /  \\______/  \\ *",
            "    (    {E}    )",
            "     \\   ==   /",
            "      |  ★  |    {X}",
            "     /        \\",
            "    (_)      (_)",
        ],

        // ── 🐰 Cloud Bun ────────────────────────────────────────────────
        (PetSpecies::CloudBun, EvolutionStage::Egg) => &[
            "      .-------.",
            "     /  {E}  \\",
            "    |  ~~~~~~  |",
            "     \\________/",
            "      ({X})",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Baby) => &[
            "      (\\_/)",
            "     ( {E} )",
            "     /  ☁  \\",
            "      ({X})",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Teen) => &[
            "     (\\_/)\\_/)",
            "    (   {E}   )",
            "    /   >☁<   \\",
            "    (__)   (__)  {X}",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Adult) => &[
            "   (\\_/)     (\\_/)",
            "  (    {E}     )",
            "   \\    >☁<    /",
            "    |   ~~~   |   {X}",
            "   /           \\",
            "  (___)     (___)",
        ],
        (PetSpecies::CloudBun, EvolutionStage::CyberElder) => &[
            "  * (\\_/)  (\\_/) *",
            "   (    {E}    )",
            "    \\   >★<   /",
            "     |  ~~~  |",
            "    /         \\   {X}",
            "   (___)     (___)",
        ],

        // ── 🐧 Star Pengu ──────────────────────────────────────────────
        (PetSpecies::StarPengu, EvolutionStage::Egg) => &[
            "      .-------.",
            "     /  {E}  \\",
            "    |  ❄❄❄❄❄  |",
            "     \\________/",
            "      ({X})",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Baby) => &[
            "      .--.",
            "     | {E} |",
            "     |:_/ |",
            "    //   \\ \\",
            "   (| {X} | )",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Teen) => &[
            "     .-----.",
            "    |  {E}  |",
            "    |  :_/  |",
            "   //       \\ \\",
            "  (|  {X}   |)",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Adult) => &[
            "     .-------.",
            "    |   {E}   |",
            "    |   :_/   |",
            "   //         \\ \\",
            "  (|   {X}    |)",
            "   \\\\___(___)__/",
        ],
        (PetSpecies::StarPengu, EvolutionStage::CyberElder) => &[
            "   * .---------. *",
            "    |    {E}    |",
            "    |    :_/    |",
            "   //     ★     \\ \\",
            "  (|    {X}     |)",
            "   ~  ~~~~~~~~~  ~",
        ],

        // ── 🐉 Ignis Drake ─────────────────────────────────────────────
        (PetSpecies::IgnisDragon, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  🔥🔥🔥🔥  |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Baby) => &[
            "      /\\ /\\",
            "     ( {E} )",
            "     /  ▽  \\",
            "     \\_____/  {X}",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Teen) => &[
            "    /\\  /\\ /\\",
            "   (  \\/ {E} )",
            "    \\   ▽   /",
            "    /^^^^^^\\   {X}",
            "   (__)  (__)",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Adult) => &[
            "   /\\ \\    / /\\",
            "  /  \\ \\__/ /  \\",
            " (    {E}    )",
            "  \\     ▽     /   {X}",
            "  /^^^^^^^^^^^\\",
            " (__)       (__)",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::CyberElder) => &[
            "  /\\ \\    ★    / /\\",
            " /  \\ \\______/ /  \\",
            "(     {E}     )",
            " \\      ▽      /",
            "  \\^^^^^^^^^^^/    {X}",
            " (§)         (§)",
            "   ~ ~ ~ ~ ~ ~",
        ],

        // ── 🐲 Azure Wyrm ──────────────────────────────────────────────
        (PetSpecies::AzureWyrm, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  🫧🫧🫧🫧  |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Baby) => &[
            "     ~~~ ~~~",
            "    (  {E}  )",
            "    /  (o)  \\",
            "   ~~~~ ~~~~  {X}",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Teen) => &[
            "   ~ ~ ~ ~ ~ ~",
            "    (   {E}   )",
            "     \\  (o)  /",
            "     ~~~ ~ ~~~",
            "    (  {X}  )",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Adult) => &[
            "  ~  ~  ~  ~  ~  ~",
            "   (    {E}     )",
            "    \\    (o)    /",
            "     ~~~  ~  ~~~",
            "   (    {X}     )",
            "     ~~~~~~~~~~~",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::CyberElder) => &[
            " ~  ~  ~  ~  ~  ~  ~",
            "  (     {E}      )",
            "   \\    (★)     /",
            "    ~~~  ~  ~~~",
            "  (     {X}      )",
            "    ~~~~~~~~~~~~~",
            "  ~  ~  ~  ~  ~  ~",
        ],

        // ── 🦊 Kitsune Nine ────────────────────────────────────────────
        (PetSpecies::EmberFox, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  ✦ ✦ ✦ ✦ ✦ |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Baby) => &[
            "     /\\   /\\",
            "    ( {E} )",
            "     \\  ω  /",
            "    ~~~~~~~~  {X}",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Teen) => &[
            "   /\\ /\\   /\\ /\\",
            "   (   {E}   )",
            "    \\   ω   /",
            "   ~~~ ~~~ ~~~   {X}",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Adult) => &[
            "  /\\ /\\ /\\   /\\ /\\ /\\",
            "   (      {E}      )",
            "    \\     ω      /",
            "   ~~~  ~~~  ~~~",
            "  (     {X}      )",
        ],
        (PetSpecies::EmberFox, EvolutionStage::CyberElder) => &[
            " /\\ /\\ /\\  ✦  /\\ /\\ /\\",
            "   (      {E}      )",
            "    \\     ω      /",
            "   ~~~  ~~~  ~~~",
            "  (     {X}      )",
            "        ~ ~ ~",
        ],

        // ── 🐦‍🔥 Sol Phoenix ─────────────────────────────────────────────
        (PetSpecies::PhoenixChick, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  ☀☀☀☀☀  |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Baby) => &[
            "     \\   /",
            "    ( {E} )",
            "     /|_|\\",
            "    ~~~~~~~  {X}",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Teen) => &[
            "    \\ \\   / /",
            "     (  {E}  )",
            "     /  >|<  \\",
            "    ~~~  ~  ~~~  {X}",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Adult) => &[
            "   \\ \\ \\     / / /",
            "    \\  \\   /  /",
            "     (   {E}   )",
            "      \\  >|<  /",
            "     ~~~  ~  ~~~   {X}",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::CyberElder) => &[
            "  \\ \\ \\  ☀  / / /",
            "   \\  \\   /  /",
            "    (    {E}    )",
            "     \\   >|<   /",
            "    ~~~  ~  ~~~",
            "   (   {X}     )",
            "      ~ ~ ~ ~",
        ],

        // ── 🦄 Astra Unicorn ───────────────────────────────────────────
        (PetSpecies::CrystalUni, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  ✧ ✧ ✧ ✧ ✧ |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Baby) => &[
            "       ▲",
            "     / | \\",
            "    ( {E} )",
            "     \\  ∪  /  {X}",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Teen) => &[
            "        ▲",
            "      / | \\",
            "     (  {E}  )",
            "      \\  ∪  /   {X}",
            "     (__)  (__)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Adult) => &[
            "         ▲",
            "       / | \\",
            "      (   {E}   )",
            "       \\   ∪   /",
            "      (__)   (__)  {X}",
            "        ~ ~ ~",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::CyberElder) => &[
            "          ✧",
            "         ▲",
            "       / | \\",
            "      (   {E}   )",
            "       \\   ∪   /",
            "      (__)   (__)",
            "       (   {X}     )",
        ],

        // ── 👑 Royal Jelly ────────────────────────────────────────────
        (PetSpecies::SlimeKing, EvolutionStage::Egg) => &[
            "     .---------.",
            "    /   {E}   \\",
            "   |  💧💧💧💧  |",
            "    \\_________/",
            "     ({X})",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Baby) => &[
            "       _^_",
            "      ( {E} )",
            "     /~~~~~~~\\",
            "    (_)     (_)  {X}",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Teen) => &[
            "       _^^_",
            "      (  {E}  )",
            "     /~~~~~~~~~\\",
            "    (_)       (_)  {X}",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Adult) => &[
            "      _^___^_",
            "     (   {E}   )",
            "    /~~~~~~~~~~~\\",
            "   /  ~  ~~~  ~  \\  {X}",
            "  (_)           (_)",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::CyberElder) => &[
            "     _^__★__^_",
            "    (    {E}    )",
            "   /~~~~~~~~~~~~~\\",
            "  ( ~~~  ~  ~~~  )",
            "  (_)           (_)  {X}",
            "     ~  ~  ~  ~",
        ],
    }
}

/// Expression pair (open, blink) for every mood.
fn mood_expression(mood: PetMood) -> (&'static str, &'static str) {
    match mood {
        PetMood::Idle => ("o.o", "-.-"),
        PetMood::Eating => ("^O^", "^w^"),
        PetMood::Sleeping => ("-.-", "-w-"),
        PetMood::Happy => ("^ω^", ">ω<"),
        PetMood::Sick => ("x_x", "T_T"),
        PetMood::Training => ("•_•", ">_<"),
    }
}

/// The two-character ambient effect for a mood.
fn mood_effect(mood: PetMood) -> &'static str {
    match mood {
        PetMood::Idle => "· ",
        PetMood::Eating => "🍡",
        PetMood::Sleeping => "zZ",
        PetMood::Happy => "♥ ",
        PetMood::Sick => "💧",
        PetMood::Training => "💨",
    }
}

fn compose(template: &[&str], eyes: &str, fx: &str) -> String {
    template
        .iter()
        .map(|line| line.replace(EYES, eyes).replace(FX, fx))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Both animation frames for a species at a given stage & mood.
/// Frame 0 = eyes open + ambient effect, frame 1 = blink + effect faded.
pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let template = body_template(species, stage);
    let (eyes_open, eyes_blink) = mood_expression(mood);
    let fx = mood_effect(mood);

    vec![
        compose(template, eyes_open, fx),
        compose(template, eyes_blink, "  "),
    ]
}

/// Ambient particle row drawn above the sprite.
pub fn get_mood_particles(mood: PetMood, tick: usize) -> &'static str {
    let frames: &[&str] = match mood {
        PetMood::Idle => &["☁️   ~   ✨   ~  ☁️", "✨   ~   ☁️   ~  ✨"],
        PetMood::Eating => &["🍴  😋  🎵  🍰  🍴", "🎵  🍰  🍴  😋  🎵"],
        PetMood::Sleeping => &["🌙   z   Z   zZ  ⭐", "⭐   Z   zZ  z   🌙"],
        PetMood::Happy => &["✨  💖  ✨  🌸  ✨", "🌸  ✨  💖  ✨  🌸"],
        PetMood::Sick => &["💧  🤒  💊  🌧️  💧", "🌧️  💊  💧  🤒  🌧️"],
        PetMood::Training => &["🔥  ⚡  💪  ⭐  🔥", "⭐  💪  🔥  ⚡  ⭐"],
    };
    frames[tick % frames.len()]
}

/// The lines of the current animation frame for a live pet.
pub fn render_animated_sprite(pet: &Pet, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];

    let mut lines = vec![format!("   {}", get_mood_particles(pet.mood, tick)), String::new()];
    lines.extend(frame.lines().map(|l| l.to_string()));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("  {} (needs cleaning!)", messes.trim()));
    }
    lines
}

/// A friendly preview of a species (adult form, idle) for menus.
pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// What your companion would say right now.
pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 80 {
            "💬 \"*crack* *wiggle* I think I'm almost ready!\"".to_string()
        } else {
            "💬 \"Somewhere inside, a tiny heartbeat waits for warmth...\"".to_string()
        };
    }
    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return "💬 \"Zzz... dreaming of sweet snacks...\"".to_string();
    }
    if pet.is_sick || pet.mood == PetMood::Sick {
        return "💬 \"Ugh... my tummy hurts... could I have medicine?\"".to_string();
    }
    if pet.poop_count > 0 {
        return "💬 \"Eep! My room is messy... a bubble bath would help!\"".to_string();
    }
    if pet.hunger <= 30 {
        return format!(
            "💬 \"My tummy is rumbling! Can I have {}?\"",
            pet.species.favorite_snack()
        );
    }
    if pet.energy <= 25 {
        return "💬 \"*yawn* So sleepy... time for a nap?\"".to_string();
    }

    let chatter = [
        format!("💬 \"Yay! {} loves spending time with you!\"", pet.name),
        format!("💬 \"Mmm, thinking about {}...\"", pet.species.favorite_snack()),
        "💬 \"Let's play the Snack Catcher mini-game!\"".to_string(),
        "💬 \"Headpats make our bond grow stronger!\"".to_string(),
    ];
    chatter[(tick / 3) % chatter.len()].clone()
}

/// Convenience wrapper used by CLI output.
pub fn mood_dialogue(pet: &Pet) -> String {
    speech_bubble(pet, 0)
}

pub fn mood_color(mood: PetMood) -> Color {
    match mood {
        PetMood::Idle => Color::Cyan,
        PetMood::Eating => Color::LightGreen,
        PetMood::Sleeping => Color::LightBlue,
        PetMood::Happy => Color::LightMagenta,
        PetMood::Sick => Color::LightRed,
        PetMood::Training => Color::LightYellow,
    }
}

pub fn stage_color(stage: EvolutionStage) -> Color {
    match stage {
        EvolutionStage::Egg => Color::White,
        EvolutionStage::Baby => Color::LightGreen,
        EvolutionStage::Teen => Color::LightCyan,
        EvolutionStage::Adult => Color::LightMagenta,
        EvolutionStage::CyberElder => Color::LightYellow,
    }
}

pub fn theme_color(theme: ShellTheme) -> Color {
    match theme {
        ShellTheme::StrawberryPink => Color::LightMagenta,
        ShellTheme::MatchaGreen => Color::LightGreen,
        ShellTheme::SkyBlue => Color::LightCyan,
        ShellTheme::LavenderDream => Color::Magenta,
        ShellTheme::SunsetGold => Color::Yellow,
    }
}

/// Sky backdrop that walks through day → evening → night as ticks pass.
pub fn sky_scene(tick: usize) -> (&'static str, &'static str) {
    match (tick / 120) % 3 {
        0 => ("☀️   ☁️    ☁️", "Daylight"),
        1 => ("🌇   ·    ☁️", "Golden hour"),
        _ => ("🌙   ✦   ✦   ✦", "Starlit night"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_species_stages_and_moods_have_multi_frame_sprites() {
        for species in PetSpecies::all() {
            for stage in EvolutionStage::all() {
                for mood in PetMood::all() {
                    let frames = get_sprite_frames(*species, *stage, *mood);
                    assert!(frames.len() >= 2, "{species:?}/{stage:?}/{mood:?}");
                    for frame in &frames {
                        assert!(!frame.trim().is_empty());
                        assert!(!frame.contains(EYES), "unreplaced eye token");
                        assert!(!frame.contains(FX), "unreplaced fx token");
                    }
                }
            }
        }
    }

    #[test]
    fn test_render_animated_sprite_and_speech_bubble() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let egg_speech = speech_bubble(&pet, 0);
        assert!(egg_speech.contains("warmth") || egg_speech.contains("ready"));

        pet.stage = EvolutionStage::Baby;
        pet.poop_count = 2;
        let dirty_lines = render_animated_sprite(&pet, 0);
        assert!(dirty_lines.iter().any(|l| l.contains('💩')));
        assert!(speech_bubble(&pet, 0).contains("bubble bath"));
    }

    #[test]
    fn test_colors_particles_and_previews() {
        for theme in ShellTheme::all() {
            let _ = theme_color(*theme);
        }
        for stage in EvolutionStage::all() {
            let _ = stage_color(*stage);
        }
        for mood in PetMood::all() {
            let _ = mood_color(*mood);
            assert!(!get_mood_particles(*mood, 0).is_empty());
            assert_ne!(get_mood_particles(*mood, 0), get_mood_particles(*mood, 1));
        }
        for species in PetSpecies::all() {
            let preview = render_species_preview(*species);
            assert!(!preview.is_empty());
        }
        let _ = sky_scene(500);
    }
}
