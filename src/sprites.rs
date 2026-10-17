//! Precision-aligned, multi-colored TrueColor ASCII art engine for **Mochigotchi**.
//!
//! Every companion has hand-crafted multi-line ASCII art across all five
//! [`EvolutionStage`]s (`Egg`, `Baby`, `Teen`, `Adult`, `CyberElder`).
//! Inside sprite templates, every character is strictly 1 terminal cell wide
//! (pure ASCII + 1-cell geometric glyphs `★`, `✦`, `♥`, `♦`, `▽`, `ω`, `ᴥ`, `∪`)
//! and at most 28 columns wide so lines NEVER wrap or shift on any terminal
//! font (Ghostty, Kitty, WezTerm, Alacritty).

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Token replaced by a 5-character facial expression (`o . o`, `^ ω ^`, etc.).
const EYES: &str = "{E}";
/// Token replaced by a 2-character mood effect (`♥ `, `zZ`, `✦ `, etc.).
const FX: &str = "{X}";

/// Detailed multi-line body templates for every species and stage.
fn body_template(species: PetSpecies, stage: EvolutionStage) -> &'static [&'static str] {
    match (species, stage) {
        // ── 🐱 Neko Mochi ───────────────────────────────────────────────
        (PetSpecies::MochiCat, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ♥   ♥  '.",
            "   /   ( {E} )   \\",
            "  |   ~~~~~~~~~   |",
            "   \\   '.....'   /",
            "    '._________.' {X}",
            "    [===========]",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Baby) => &[
            "      /\\___/\\  {X}",
            "     /  ♥ ♥  \\",
            "    (  {E}  )",
            "    (   >ω<   )~",
            "     (\")___(\")",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Teen) => &[
            "     /\\_____/\\   {X}",
            "    /  /\\ /\\  \\",
            "   ( = {E} = )",
            "   (   >★<    )~~",
            "    \\  (___)  /",
            "     (\")___(\")",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Adult) => &[
            "     /\\_______/\\   {X}",
            "    /  /\\   /\\  \\",
            "   ( == {E} == )",
            "    \\   >★<    /  .~.",
            "    /  .-----.  \\ ( ♥ )",
            "   (  (  ♥ ♥  )  ) '~'",
            "    \\__)_____(__/",
        ],
        (PetSpecies::MochiCat, EvolutionStage::CyberElder) => &[
            "  ✦  /\\_______/\\  ✦",
            " *  / /\\  ★  /\\ \\  *",
            "   ( == {E} == ) {X}",
            "  ~ \\   >★<    / ~",
            "  /  / .-----. \\  \\",
            " (  ( ( ★ ♥ ★ ) )  )",
            "  \\__\\_)_____(_/__/",
        ],

        // ── 🐶 Boba Pup ─────────────────────────────────────────────────
        (PetSpecies::BobaPup, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  •   •  '.",
            "   /   ( {E} )   \\",
            "  |   • • • • •   |",
            "   \\   • • • •   /",
            "    '._________.' {X}",
            "    [===========]",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Baby) => &[
            "    .-\"-. .-\"-. {X}",
            "   (  |     |  )",
            "    \\ ( {E} ) /",
            "     (  /ᴥ\\  )~~",
            "      (\")_(\")",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Teen) => &[
            "    .-\"\"-. .-\"\"-. {X}",
            "   / /|  '-'  |\\ \\",
            "  ( | ( {E} ) | )",
            "   \\    \\ ᴥ /    /~~",
            "    |  [=====]  |/",
            "   (__) • • • (__)",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Adult) => &[
            "    .-\"\"-.   .-\"\"-. {X}",
            "   / /|  '---'  |\\ \\",
            "  ( |  ( {E} )  | )",
            "   \\     \\ ᴥ /     /",
            "    |   [==♦==]   | ~~",
            "   /   / • • • \\   \\/",
            "  (___)_________(___)",
        ],
        (PetSpecies::BobaPup, EvolutionStage::CyberElder) => &[
            "  ✦ .-\"\"-. ★ .-\"\"-. ✦",
            " * / /|  '---'  |\\ \\ *",
            "  ( |  ( {E} )  | ) {X}",
            "   \\     \\ ᴥ /     /",
            "    |   [==★==]   | ~~",
            "   /   / • ★ • \\   \\/",
            "  (___)_________(___)",
        ],

        // ── 🐰 Cloud Bun ────────────────────────────────────────────────
        (PetSpecies::CloudBun, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ~   ~  '.",
            "   /   ( {E} )   \\",
            "  |   ~~~~~~~~~   |",
            "   \\   ~~~~~~~   /",
            "    '._________.' {X}",
            "    [===========]",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Baby) => &[
            "     (\\_/) (\\_/) {X}",
            "     (  |   |  )",
            "    (  {E}  )",
            "    (   >∪<   )o",
            "     (\")___(\")",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Teen) => &[
            "     /\"\\     /\"\\  {X}",
            "    | | |   | | |",
            "    \\  \\___/  /",
            "   (   {E}   )",
            "  ~(   (>♥<)   )~",
            "    (\")_____(\")",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Adult) => &[
            "     /\"\"\\     /\"\"\\  {X}",
            "    | || |   | || |",
            "     \\  \\___/  /",
            "    (   {E}   )",
            "  ~ (  ( >♥< )  ) ~",
            "   ( \\  '---'  / )",
            "    (___)___(___)",
        ],
        (PetSpecies::CloudBun, EvolutionStage::CyberElder) => &[
            "  ✦  /\"\"\\  ★  /\"\"\\  ✦",
            " *  | || |   | || |  *",
            "     \\  \\___/  /    {X}",
            "    (   {E}   )",
            "  ~ (  ( >★< )  ) ~",
            " ( ~ \\  '---'  / ~ )",
            "  (____)_____(____)",
        ],

        // ── 🐧 Star Pengu ──────────────────────────────────────────────
        (PetSpecies::StarPengu, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ✦   ✦  '.",
            "   /   ( {E} )   \\",
            "  |   * * * * *   |",
            "   \\   ✦  *  ✦   /",
            "    '._________.' {X}",
            "    [===========]",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Baby) => &[
            "      .----.  {X}",
            "     /  ✦   \\",
            "    | {E} |",
            "   /|   ▽   |\\",
            "  (_|  (_)  |_)",
            "     \"\"   \"\"",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Teen) => &[
            "      .------.   {X}",
            "     /  ★  ★  \\",
            "    |  {E}  |",
            "   /|    ▽    |\\",
            "  / | [=====] | \\",
            " (_/   (   )   \\_)",
            "      \"\"\" \"\"\"",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Adult) => &[
            "       .--------.   {X}",
            "      /  ✦  ★  ✦ \\",
            "     |   {E}  |",
            "    /|     ▽     |\\",
            "   / |  [==★==]  | \\",
            "  (  |  (  ♦  )  |  )",
            "   \\_|___\\___/___|_/ ",
        ],
        (PetSpecies::StarPengu, EvolutionStage::CyberElder) => &[
            "  ✦    .---★---.    ✦",
            " *    / ✦  ★  ✦ \\    *",
            "     |  {E}  |  {X}",
            "    /|     ▽     |\\",
            "   / |  [==★==]  | \\",
            "  (  |  ( ★♦★ )  |  )",
            "   \\_|___\\___/___|_/ ",
        ],

        // ── 🐉 Ignis Drake (Crimson Western Dragon) ────────────────────
        (PetSpecies::IgnisDragon, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ♦   ♦  '.",
            "   /   ( {E} )   \\",
            "  |   /\\ /\\ /\\    |",
            "   \\  \\/ \\/ \\/   /",
            "    '._________.' {X}",
            "    [===♦♦♦♦♦===]",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Baby) => &[
            "     /\\     /\\   {X}",
            "    /  \\___/  \\",
            "   (   {E}   )",
            "  /|    \\▽/    |\\",
            " /_|   (vvv)   |_\\~♦",
            "    \\__)___(__/",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Teen) => &[
            "      /\\     /\\    {X}",
            "  /| /  \\___/  \\ |\\",
            " / |/  {E}  \\| \\",
            "(  |    \\ ▽ /    |  )",
            " \\  \\  /^^^^^\\  /  /~♦",
            "  \\__| ( vvv ) |__/",
            "      \\__)_(__/",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Adult) => &[
            "       /\\       /\\    {X}",
            "  /\\  /  \\_____/  \\  /\\",
            " /  \\/ / {E} \\ \\/  \\",
            "( /\\  |   \\ ▽ /   |  /\\ )",
            " \\  \\  \\ /^^^^^\\ /  /  /",
            "  \\__\\__| |vvv| |__/~~♦♦",
            "      (__)_____(__)",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::CyberElder) => &[
            "  ✦    /\\   ★   /\\    ✦",
            " /\\  /\\ \\_____/ /\\  /\\ {X}",
            "/  \\/  \\ {E} /  \\/  \\",
            "\\ /\\  |   \\ ▽ /   |  /\\ /",
            " \\  \\  \\ /^^^^^\\ /  /  /",
            "  \\__\\__| ★♦★♦★ |__/~~♦♦",
            "      (__)_____(__)",
        ],

        // ── 🐲 Azure Wyrm (Eastern Sky & Sea Dragon) ───────────────────
        (PetSpecies::AzureWyrm, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ~   ♦  '.",
            "   /   ( {E} )   \\",
            "  |   ~ ~ ~ ~ ~   |",
            "   \\   ~ ~ ~ ~   /",
            "    '._________.' {X}",
            "    [===~~♦~~===]",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Baby) => &[
            "     ~\\     /~  {X}",
            "    ~( {E} )~",
            "      /  (♦)  \\",
            "    ~(   ~~~   )~",
            "      '~~~~~~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Teen) => &[
            "     ~\\  ___  /~   {X}",
            "   ~~(  {E}  )~~",
            "      \\  (♦)  /",
            "     /  /~~~\\  \\",
            "   ~(  ( ~~~ )  )~",
            "     '~~\\___/~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Adult) => &[
            "      ~\\  ___  /~    {X}",
            "   ~~~(  {E}  )~~~",
            "       \\  (♦)  /",
            "     .---'~~~'---.",
            "    (  /~~\\ ♦ /~~\\ )",
            "     '~\\  '~~~'  /~'",
            "        '~~~~~~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::CyberElder) => &[
            "  ✦   ~\\  _★_  /~   ✦",
            " * ~~~(  {E}  )~~~ * {X}",
            "       \\  (♦)  /",
            "    .----'~~~'----.",
            "   (  /~~\\ ★♦★ /~~\\ )",
            "    '~\\   '~~~'   /~'",
            "       '~~~~~~~~~'",
        ],

        // ── 🦊 Kitsune Nine (Mystic Shrine Fox) ────────────────────────
        (PetSpecies::EmberFox, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ✦   ✦  '.",
            "   /   ( {E} )   \\",
            "  |   ✦  ~ ~  ✦   |",
            "   \\   '~~~~~'   /",
            "    '._________.' {X}",
            "    [===✦✦✦✦✦===]",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Baby) => &[
            "     /\\     /\\   {X}",
            "    /  \\___/  \\",
            "   (   {E}   )",
            "    \\   \\ω/   /  .~.",
            "     (  ~~~  )  ( ✦ )",
            "     (\")___(\")   '~'",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Teen) => &[
            "     /\\     /\\    {X}",
            "    / /\\___/\\ \\  .~..~.",
            "   (  {E}  )( ✦||✦ )",
            "    \\   \\ω/   /  \\ || /",
            "    / [==★==] \\___/ /",
            "   (__)_____(__)___/",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Adult) => &[
            "     /\\       /\\    {X}",
            "    / /\\_____/\\ \\  .~..~..~.",
            "   (   {E}   )( ✦|★|✦ )",
            "    \\    \\ω/    /  \\  \\ /  /",
            "    /  [==★==]  \\___\\  V  /",
            "   (  (   ♥   )  )_______/",
            "    (___)___(___)",
        ],
        (PetSpecies::EmberFox, EvolutionStage::CyberElder) => &[
            "  ✦  /\\   ★   /\\  ✦",
            " ✦  / /\\_____/\\ \\ .~..~..~.",
            "   (   {E}   )( ✦|★|✦ )",
            "    \\    \\ω/    /  \\ \\|/ /",
            "    /  [==★==]  \\___\\ V /",
            "   (  (  ★♥★  )  )___\\_/",
            "    (___)___(___)     {X}",
        ],

        // ── 🦅 Sol Phoenix (Immortal Solar Firebird) ───────────────────
        (PetSpecies::PhoenixChick, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ★   ♦  '.",
            "   /   ( {E} )   \\",
            "  |   \\ / \\ / \\   |",
            "   \\   V   V   V /",
            "    '._________.' {X}",
            "    [===★♦★♦★===]",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Baby) => &[
            "       \\ | /   {X}",
            "      .-\"\"\"-.",
            "     ( {E} )",
            "    /\\  >▽<  /\\",
            "   /  \\_____/  \\",
            "      //   \\\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Teen) => &[
            "        \\ \\|/ /    {X}",
            "       .-'\"\"\"'-.",
            "  /\\  (  {E}  )  /\\",
            " /  \\  \\  >▽<  /  /  \\",
            "/ /\\ \\_/  ★ ★  \\_/ /\\ \\",
            "  \\____)_______(____/",
            "        //   \\\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Adult) => &[
            "        \\ \\|/ /     {X}",
            "  /\\  .-'\"\"★\"\"'-.  /\\",
            " /  \\/ ( {E} ) \\/  \\",
            "/ /\\ \\  \\ >▽< /  / /\\ \\",
            "\\ \\_\\ \\__'---'__/ /_/ /",
            " \\____/ / ♦★♦ \\ \\____/",
            "       /__/ | \\__\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::CyberElder) => &[
            "  ✦     \\ \\★/ /     ✦",
            " * /\\ .-'\"\"★\"\"'-. /\\ * {X}",
            "  /  \\(  {E}  )/  \\",
            " / /\\ \\ \\ >▽< / / /\\ \\",
            "( /  \\ \\_'---'_/ /  \\ )",
            " \\____\\_/ ★♦★ \\_/____/",
            "       /__/||\\__\\",
        ],

        // ── 🦄 Astra Unicorn (Celestial Prism Steed) ───────────────────
        (PetSpecies::CrystalUni, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ✦   ★  '.",
            "   /   ( {E} )   \\",
            "  |   ✧  * *  ✧   |",
            "   \\   '*****'   /",
            "    '._________.' {X}",
            "    [===✦✧★✧✦===]",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Baby) => &[
            "        /\\    {X}",
            "      /\\||/\\",
            "     ( {E} )",
            "     (  >∪<  )~✦",
            "     /|     |\\",
            "    (_|     |_)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Teen) => &[
            "         /\\      {X}",
            "        /★ \\",
            "      /\\ || /\\ ~~✦",
            "     (  {E}  )~~~",
            "      \\  >∪<  /",
            "     /  '---'  \\~~",
            "    (__)     (__)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Adult) => &[
            "          /\\       {X}",
            "         /✦★\\",
            "       /\\ || /\\  ~~~✦",
            "      (  {E}  )~~~~",
            "       \\  >∪<  / ~~~",
            "      / .-----. \\ ~~",
            "     ( (  ✦★✦  ) )",
            "    (__) '---' (__)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::CyberElder) => &[
            "  ✦       /\\       ✦",
            " *      ★/✦★\\★      * {X}",
            "       /\\ || /\\  ~~~★",
            "      (  {E}  )~~~~",
            "       \\  >∪<  / ~~~",
            "      / .-----. \\ ~~",
            "     ( ( ★✦♦✦★ ) )",
            "    (__) '---' (__)",
        ],

        // ── 👑 Royal Jelly (Mystic Slime Monarch) ──────────────────────
        (PetSpecies::SlimeKing, EvolutionStage::Egg) => &[
            "      .-\"\"\"\"\"-.",
            "    .'  ♦   ★  '.",
            "   /   ( {E} )   \\",
            "  |   ~ ~ ~ ~ ~   |",
            "   \\   o  o  o   /",
            "    '._________.' {X}",
            "    [===♦★♦★♦===]",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Baby) => &[
            "       .---.   {X}",
            "     .'  ♦  '.",
            "    (  {E}  )",
            "   (   ~~~~~   )",
            "    '---------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Teen) => &[
            "       /\\^/\\    {X}",
            "      | o o |",
            "    .-\"\"\"\"\"\"\"-.",
            "   /  {E}   \\",
            "  (   ~~~~~~~   )",
            "   '-----------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Adult) => &[
            "      /\\ /\\ /\\    {X}",
            "     |  ★ ♦ ★ |",
            "   .-\"\"\"\"\"\"\"\"\"-.",
            "  /   {E}    \\",
            " (  ♦ ~~~~~~~ ♦  )",
            " (   ~~~~~~~~~   )",
            "  '-------------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::CyberElder) => &[
            "  ✦   /\\  ★  /\\   ✦",
            " *   |  ★♦★♦★  |   * {X}",
            "   .-\"\"\"\"\"\"\"\"\"\"\"-.",
            "  /   {E}     \\",
            " (  ♦  ~~~ ★ ~~~ ♦ )",
            " (   ~~~~~~~~~~~   )",
            "  '---------------'",
        ],
    }
}

/// 5-character expression pair (open, blink/alt) for every mood.
fn mood_expression(mood: PetMood) -> (&'static str, &'static str) {
    match mood {
        PetMood::Idle => ("o . o", "- . -"),
        PetMood::Eating => ("^ O ^", "^ w ^"),
        PetMood::Sleeping => ("- . -", "- w -"),
        PetMood::Happy => ("^ ω ^", "> ω <"),
        PetMood::Sick => ("x _ x", "T _ T"),
        PetMood::Training => ("• _ •", "> _ <"),
    }
}

/// 2-character single-width ambient effect for a mood so `{X}` is always 2 cells.
fn mood_effect(mood: PetMood) -> &'static str {
    match mood {
        PetMood::Idle => "✦ ",
        PetMood::Eating => "♪ ",
        PetMood::Sleeping => "zZ",
        PetMood::Happy => "♥ ",
        PetMood::Sick => "💧",
        PetMood::Training => "★ ",
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
/// Frame 1 uses two spaces for `{X}` so there is no stray floating dot.
pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let template = body_template(species, stage);
    let (eyes_open, eyes_blink) = mood_expression(mood);
    let fx = mood_effect(mood);

    vec![
        compose(template, eyes_open, fx),
        compose(template, eyes_blink, "  "),
    ]
}

/// Ambient particle row drawn above the sprite (pure 1-cell glyphs so centering is exact).
pub fn get_mood_particles(mood: PetMood, tick: usize) -> &'static str {
    let frames: &[&str] = match mood {
        PetMood::Idle => &["✦   ~   ★   ~   ✦", "★   ~   ✦   ~   ★"],
        PetMood::Eating => &["♪   ♥   ★   ♥   ♪", "★   ♪   ♥   ♪   ★"],
        PetMood::Sleeping => &["☾   z   Z   zZ  ✦", "✦   Z   zZ  z   ☾"],
        PetMood::Happy => &["✦   ♥   ★   ♥   ✦", "★   ✦   ♥   ✦   ★"],
        PetMood::Sick => &[".   ~   x   ~   .", "~   .   x   .   ~"],
        PetMood::Training => &["★   ♦   ▲   ♦   ★", "♦   ★   ▲   ★   ♦"],
    };
    frames[tick % frames.len()]
}

/// Primary, secondary, and accent RGB colors for each species.
pub fn species_palette(species: PetSpecies) -> (Color, Color, Color) {
    match species {
        PetSpecies::MochiCat => (
            Color::Rgb(255, 182, 206),
            Color::Rgb(255, 235, 242),
            Color::Rgb(255, 110, 155),
        ),
        PetSpecies::BobaPup => (
            Color::Rgb(222, 178, 130),
            Color::Rgb(145, 102, 68),
            Color::Rgb(255, 215, 140),
        ),
        PetSpecies::CloudBun => (
            Color::Rgb(215, 238, 255),
            Color::Rgb(150, 208, 255),
            Color::Rgb(255, 170, 205),
        ),
        PetSpecies::StarPengu => (
            Color::Rgb(135, 195, 255),
            Color::Rgb(235, 245, 255),
            Color::Rgb(255, 215, 100),
        ),
        PetSpecies::IgnisDragon => (
            Color::Rgb(255, 95, 85),
            Color::Rgb(255, 185, 75),
            Color::Rgb(255, 235, 120),
        ),
        PetSpecies::AzureWyrm => (
            Color::Rgb(80, 215, 235),
            Color::Rgb(140, 175, 255),
            Color::Rgb(220, 250, 255),
        ),
        PetSpecies::EmberFox => (
            Color::Rgb(255, 150, 75),
            Color::Rgb(255, 235, 190),
            Color::Rgb(110, 235, 255),
        ),
        PetSpecies::PhoenixChick => (
            Color::Rgb(255, 195, 70),
            Color::Rgb(255, 105, 70),
            Color::Rgb(255, 245, 170),
        ),
        PetSpecies::CrystalUni => (
            Color::Rgb(215, 175, 255),
            Color::Rgb(150, 235, 255),
            Color::Rgb(255, 225, 135),
        ),
        PetSpecies::SlimeKing => (
            Color::Rgb(110, 225, 195),
            Color::Rgb(165, 245, 225),
            Color::Rgb(255, 215, 90),
        ),
    }
}

/// Colorize a single line of sprite art character-by-character in TrueColor RGB.
fn colorize_sprite_line(line: &str, species: PetSpecies, stage: EvolutionStage) -> Line<'static> {
    let (primary, secondary, accent) = species_palette(species);
    let body_color = if stage == EvolutionStage::Egg {
        secondary
    } else {
        primary
    };

    let mut spans: Vec<Span<'static>> = Vec::new();
    for ch in line.chars() {
        let style = match ch {
            '♦' | '▲' => Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
            '★' | '☆' | '✧' | '✦' | '*' | '♪' => Style::default()
                .fg(Color::Rgb(255, 225, 95))
                .add_modifier(Modifier::BOLD),
            '♥' | '♡' => Style::default()
                .fg(Color::Rgb(255, 130, 180))
                .add_modifier(Modifier::BOLD),
            'o' | 'O' | '•' | '^' | '>' | '<' | 'x' | 'T' => Style::default()
                .fg(Color::Rgb(255, 250, 240))
                .add_modifier(Modifier::BOLD),
            'ω' | 'ᴥ' | '▽' | '∪' => Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
            '~' | '=' | '[' | ']' => Style::default().fg(secondary),
            '/' | '\\' | '(' | ')' | '|' | '_' | '-' | '.' | '\'' | '"' => {
                Style::default().fg(body_color)
            }
            _ => Style::default().fg(primary),
        };
        spans.push(Span::styled(ch.to_string(), style));
    }
    Line::from(spans)
}

/// Plain String lines of the current animation frame (used by CLI & tests).
pub fn render_animated_sprite(pet: &Pet, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];

    let mut lines = vec![get_mood_particles(pet.mood, tick).to_string(), String::new()];
    lines.extend(frame.lines().map(|l| l.to_string()));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("{} (needs bath [C]!)", messes.trim()));
    }
    lines
}

/// Multi-colored Ratatui [`Line`]s for the habitat canvas.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];
    let (_, secondary, accent) = species_palette(pet.species);

    let mut out: Vec<Line<'static>> = Vec::new();
    out.push(Line::from(Span::styled(
        get_mood_particles(pet.mood, tick).to_string(),
        Style::default().fg(accent),
    )));

    if tick.is_multiple_of(2) && !pet.is_sleeping {
        out.push(Line::from(""));
    }

    for line in frame.lines() {
        out.push(colorize_sprite_line(line, pet.species, pet.stage));
    }

    let ground = match pet.species.family() {
        "Mythical & Fantasy" => "═══════✦═══════✦═══════",
        _ => "░░░░░░░★░░░░░░░★░░░░░░░",
    };
    out.push(Line::from(Span::styled(
        ground.to_string(),
        Style::default().fg(secondary),
    )));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        out.push(Line::from(Span::styled(
            format!("{} (flush & bath [C]!)", messes.trim()),
            Style::default().fg(Color::LightRed),
        )));
    }

    out
}

/// Multi-colored Ratatui [`Line`]s for the Adoption Wizard species preview.
pub fn render_colored_preview(
    species: PetSpecies,
    stage: EvolutionStage,
    tick: usize,
) -> Vec<Line<'static>> {
    let frames = get_sprite_frames(species, stage, PetMood::Happy);
    let frame = &frames[tick % frames.len()];
    frame
        .lines()
        .map(|l| colorize_sprite_line(l, species, stage))
        .collect()
}

/// A friendly preview of a species (adult form, idle) as plain strings.
pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// What your companion would say right now (concise so it never wraps awkwardly).
pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 80 {
            "💬 \"*crack* Almost ready to hatch!\"".to_string()
        } else {
            "💬 \"A tiny heartbeat waits for warmth...\"".to_string()
        };
    }
    if pet.has_toothache {
        return "💬 \"Ouch, toothache! Medicine [H] please!\"".to_string();
    }
    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return "💬 \"Zzz... dreaming of sweet snacks...\"".to_string();
    }
    if pet.is_sick || pet.mood == PetMood::Sick {
        return "💬 \"Ugh... could I have medicine [H]?\"".to_string();
    }
    if pet.poop_count > 0 {
        return "💬 \"Eep! A bubble bath [C] would help!\"".to_string();
    }
    if pet.hunger <= 30 {
        return format!("💬 \"Hungry for {}!\"", pet.species.favorite_snack());
    }
    if pet.energy <= 25 {
        return "💬 \"*yawn* Time for bed [S]?\"".to_string();
    }

    let chatter = [
        format!("💬 \"{} loves spending time with you!\"", pet.name),
        format!("💬 \"Thinking about {}...\"", pet.species.favorite_snack()),
        "💬 \"Let's play Snack Catcher [G]!\"".to_string(),
        "💬 \"Headpats [T] grow our bond!\"".to_string(),
    ];
    chatter[(tick / 3) % chatter.len()].clone()
}

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
        ShellTheme::StrawberryPink => Color::Rgb(255, 130, 175),
        ShellTheme::MatchaGreen => Color::Rgb(130, 215, 155),
        ShellTheme::SkyBlue => Color::Rgb(115, 195, 255),
        ShellTheme::LavenderDream => Color::Rgb(190, 150, 255),
        ShellTheme::SunsetGold => Color::Rgb(255, 195, 95),
    }
}

pub fn sky_scene(tick: usize) -> (&'static str, &'static str) {
    match (tick / 120) % 3 {
        0 => ("🌞 Daylight", "Sunny"),
        1 => ("🌇 Golden Hour", "Breezy"),
        _ => ("🌙 Starlit Night", "Calm"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_species_stages_and_moods_have_multi_frame_sprites_within_width() {
        for species in PetSpecies::all() {
            for stage in EvolutionStage::all() {
                for mood in PetMood::all() {
                    let frames = get_sprite_frames(*species, *stage, *mood);
                    assert!(frames.len() >= 2, "{species:?}/{stage:?}/{mood:?}");
                    for frame in &frames {
                        assert!(!frame.trim().is_empty());
                        assert!(!frame.contains(EYES), "unreplaced eye token");
                        assert!(!frame.contains(FX), "unreplaced fx token");
                        for line in frame.lines() {
                            assert!(
                                line.chars().count() <= 30,
                                "{species:?}/{stage:?} line too wide ({} chars): {line}",
                                line.chars().count()
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_render_animated_and_colored_sprite_and_speech_bubble() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let colored = render_colored_sprite(&pet, 0);
        assert!(!colored.is_empty());

        let egg_speech = speech_bubble(&pet, 0);
        assert!(egg_speech.contains("heartbeat") || egg_speech.contains("hatch"));

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
            let colored_preview = render_colored_preview(*species, EvolutionStage::Adult, 0);
            assert!(!colored_preview.is_empty());
        }
        let _ = sky_scene(500);
    }
}
