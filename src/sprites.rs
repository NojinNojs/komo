//! Rich, detailed, multi-colored ASCII art engine for **Mochigotchi**.
//!
//! Every companion has hand-crafted multi-line ASCII art across all five
//! [`EvolutionStage`]s (`Egg`, `Baby`, `Teen`, `Adult`, `CyberElder`), plus
//! per-character/per-feature RGB colorization (`render_colored_sprite` and
//! `render_colored_preview`) so horns, wings, scales, cheeks, crowns, and
//! elemental auras glow with distinct colors inside the Ratatui habitat.

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Token replaced by a 5-character facial expression (`o . o`, `^ ω ^`, etc.).
const EYES: &str = "{E}";
/// Token replaced by a 2-character mood effect (`♥ `, `zZ`, `🔥`, etc.).
const FX: &str = "{X}";

/// Detailed multi-line body templates for every species and stage.
/// `{E}` is always 5 characters wide so box alignment stays rock-solid.
fn body_template(species: PetSpecies, stage: EvolutionStage) -> &'static [&'static str] {
    match (species, stage) {
        // ── 🐱 Neko Mochi ───────────────────────────────────────────────
        (PetSpecies::MochiCat, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  🌸  🌸  '.",
            "      /   ( {E} )   \\",
            "     |   ~~~~~~~~~   |",
            "      \\   '.....'   /",
            "       '._________.'  {X}",
            "       [===========]",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Baby) => &[
            "        /\\_/\\   {X}",
            "       / 🍓  \\",
            "      ( {E} )",
            "      (  >🎀< )~",
            "       (\")_(\")",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Teen) => &[
            "       /\\_____/\\    {X}",
            "      /  /\\ /\\  \\",
            "     ( = {E} = )",
            "     (   >🔔<    )~~",
            "      \\  (___)  /",
            "       (\")___(\")",
        ],
        (PetSpecies::MochiCat, EvolutionStage::Adult) => &[
            "       /\\_______/\\     {X}",
            "      /  /\\   /\\  \\   .---.",
            "     ( == {E} == ) ( 🍡  )",
            "      \\   >🔔<    /   '---'",
            "      /  .-----.  \\  //",
            "     (  (  ♥ ♥  )  )//",
            "      \\__)_____(__/",
        ],
        (PetSpecies::MochiCat, EvolutionStage::CyberElder) => &[
            "    ✨  /\\_________/\\  ✨",
            "   *   /  /\\  ★  /\\  \\   *",
            "      ( ==  {E}  == )   {X}",
            "     ~ \\   >👑<     / ~",
            "    /   / .-------. \\   \\",
            "   (   ( (  ★ ♥ ★  ) )   )",
            "    \\___\\_)_______(_/___/",
            "      ~ ~ ~ ~ ~ ~ ~ ~ ~",
        ],

        // ── 🐶 Boba Pup ─────────────────────────────────────────────────
        (PetSpecies::BobaPup, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  🧋  🧋  '.",
            "      /   ( {E} )   \\",
            "     |   • • • • •   |",
            "      \\   • • • •   /",
            "       '._________.'  {X}",
            "       [===========]",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Baby) => &[
            "      .-\"-. .-\"-.  {X}",
            "     (  |     |  )",
            "      \\ ( {E} ) /",
            "       (  /ᴥ\\  )~~",
            "        (\")_(\")",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Teen) => &[
            "     .-\"\"-.   .-\"\"-.  {X}",
            "    /  /|  '-'  |\\  \\",
            "   (  | ( {E} ) |  )",
            "    \\    \\ ᴥ /    / ~~",
            "     |  [=====]  |/",
            "    (__) • • • (__)",
        ],
        (PetSpecies::BobaPup, EvolutionStage::Adult) => &[
            "     .-\"\"\"-.     .-\"\"\"-.   {X}",
            "    /  /|   '---'   |\\  \\",
            "   (  |  (  {E}  )  |  )",
            "    \\     \\  ᴥ  /     /  .---.",
            "     |   [===🦴===]   |  ( 🧋 )",
            "    /   /  • • •  \\   \\  '---'",
            "   (___)___________(___)",
        ],
        (PetSpecies::BobaPup, EvolutionStage::CyberElder) => &[
            "   ✨ .-\"\"\"-.  👑  .-\"\"\"-. ✨",
            "  *  /  /|   '---'   |\\  \\  *",
            "    (  |  (  {E}  )  |  )  {X}",
            "     \\     \\  ᴥ  /     /",
            "      |   [== ★ ==]   |  ~~",
            "     /   /  • ★ •  \\   \\/",
            "    (___)___________(___)",
            "      ~ ~ ~ ~ ~ ~ ~ ~ ~",
        ],

        // ── 🐰 Cloud Bun ────────────────────────────────────────────────
        (PetSpecies::CloudBun, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  ☁️   ☁️  '.",
            "      /   ( {E} )   \\",
            "     |   ~~~~~~~~~   |",
            "      \\   ~~~~~~~   /",
            "       '._________.'  {X}",
            "       [===========]",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Baby) => &[
            "       (\\_/) (\\_/)  {X}",
            "       (  |   |  )",
            "      (  {E}  )",
            "      (   >☁<   )o",
            "       (\")___(\")",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Teen) => &[
            "       /\"\\     /\"\\   {X}",
            "      | | |   | | |",
            "      \\  \\___/  /",
            "     (   {E}   )",
            "    ~(   (>🥕<)   )~",
            "      (\")_____(\")",
        ],
        (PetSpecies::CloudBun, EvolutionStage::Adult) => &[
            "       /\"\"\\       /\"\"\\    {X}",
            "      | || |     | || |",
            "       \\  \\_____/  /",
            "      (    {E}    )",
            "    ☁️ (   ( >🌸< )   ) ☁️",
            "     (  \\  '-----'  /  )",
            "      (____)_____ (____)",
        ],
        (PetSpecies::CloudBun, EvolutionStage::CyberElder) => &[
            "    ✨  /\"\"\\  👑  /\"\"\\  ✨",
            "   *   | || |   | || |   *",
            "        \\  \\___/  /      {X}",
            "       (   {E}   )",
            "    ☁️  (  ( >★< )  )  ☁️",
            "   (  ☁️ \\ '-----' / ☁️  )",
            "    (_____)_______(_____)",
            "      ~ ☁️ ~ ~ ★ ~ ~ ☁️ ~",
        ],

        // ── 🐧 Star Pengu ──────────────────────────────────────────────
        (PetSpecies::StarPengu, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  ❄️   ❄️  '.",
            "      /   ( {E} )   \\",
            "     |   * * * * *   |",
            "      \\   ❄️  ❄️  ❄️   /",
            "       '._________.'  {X}",
            "       [===========]",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Baby) => &[
            "        .----.   {X}",
            "       /  ❄️   \\",
            "      | {E} |",
            "     /|   ▽   |\\",
            "    (_|  (_)  |_)",
            "       \"\"   \"\"",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Teen) => &[
            "        .------.    {X}",
            "       /  ★  ★  \\",
            "      |  {E}  |",
            "     /|    ▽    |\\",
            "    / | [=====] | \\",
            "   (_/   (   )   \\_)",
            "        \"\"\" \"\"\"",
        ],
        (PetSpecies::StarPengu, EvolutionStage::Adult) => &[
            "         .--------.     {X}",
            "        /  ❄️  ★  ❄️  \\",
            "       |   {E}   |",
            "      /|     ▽     |\\",
            "     / |  [==🧣==]  | \\",
            "    (  |  (  ★  )  |  )",
            "     \\_|___\\___/___|_/ ",
            "         \"\"\"\" \"\"\"\"",
        ],
        (PetSpecies::StarPengu, EvolutionStage::CyberElder) => &[
            "    ✨   .---👑---.   ✨",
            "   *    /  ❄️  ★  ❄️  \\    *",
            "       |   {E}   |   {X}",
            "      /|     ▽     |\\",
            "     / |  [==🌌==]  | \\",
            "    (  |  ( ★ 🫐 ★ ) |  )",
            "     \\_|___\\_____/___|_/ ",
            "       ~ ❄️ ~ ★ ~ ❄️ ~",
        ],

        // ── 🐉 Ignis Drake (Crimson Western Dragon) ────────────────────
        (PetSpecies::IgnisDragon, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  🔥  🔥  '.",
            "      /   ( {E} )   \\",
            "     |   /\\ /\\ /\\    |",
            "      \\  \\/ \\/ \\/   /",
            "       '._________.'  {X}",
            "       [===🔥🔥🔥===]",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Baby) => &[
            "       /\\     /\\    {X}",
            "      /  \\___/  \\",
            "     (   {E}   )",
            "    /|    \\▽/    |\\",
            "   /_|   (vvv)   |_\\~🔥",
            "      \\__)___(__/",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Teen) => &[
            "        /\\       /\\      {X}",
            "    /| /  \\_____/  \\ |\\",
            "   / |/   {E}   \\| \\",
            "  (  |     \\ ▽ /     |  )",
            "   \\  \\   /^^^^^\\   /  /~🔥",
            "    \\__| (  vvv  ) |__/",
            "        \\__)___(__/",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::Adult) => &[
            "         /\\         /\\        {X}",
            "    /\\  /  \\_______/  \\  /\\",
            "   /  \\/ /  {E}  \\ \\/  \\",
            "  / /\\  |    \\ ▽ /    |  /\\ \\",
            " ( /  \\  \\  /^^^^^\\  /  /  \\ )",
            "  \\    \\__| | vvv | |__/    /~~🔥🔥",
            "   \\____/ (__)___(__) \\____/",
        ],
        (PetSpecies::IgnisDragon, EvolutionStage::CyberElder) => &[
            "    ✨    /\\   👑   /\\    ✨",
            "   /\\  /\\ \\_______/ /\\  /\\   {X}",
            "  /  \\/  \\ {E} /  \\/  \\",
            " / /\\  /\\ |  \\ ▽ /  | /\\  /\\ \\",
            "( /  \\/  \\ \\/^^^^^\\/ /  \\/  \\ )",
            " \\        \\_| ★🔥★ |_/        /~~🔥🔥🔥",
            "  \\______/ (__)_(__) \\______/",
            "     ~ 🔥 ~ ★ ~ 🔥 ~ ★ ~",
        ],

        // ── 🐲 Azure Wyrm (Eastern Sky & Sea Dragon) ───────────────────
        (PetSpecies::AzureWyrm, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  🌊  🫧  '.",
            "      /   ( {E} )   \\",
            "     |   ~ ~ ~ ~ ~   |",
            "      \\   ~ ~ ~ ~   /",
            "       '._________.'  {X}",
            "       [===🌊🫧🌊===]",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Baby) => &[
            "       ~\\     /~   {X}",
            "      ~( {E} )~",
            "        /  (🫧) \\",
            "      ~(   ~~~   )~",
            "        '~~~~~~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Teen) => &[
            "       ~\\  ___  /~    {X}",
            "     ~~(  {E}  )~~",
            "        \\  (🫧)  /",
            "       /  /~~~\\  \\",
            "     ~(  ( ~~~ )  )~",
            "       '~~\\___/~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::Adult) => &[
            "        ~\\  _____  /~      {X}",
            "     ~~~(   {E}   )~~~",
            "         \\   (🫧)   /",
            "       .---'~~~~~~~'---.",
            "      (  /~~\\  🌊  /~~\\  )",
            "       '~\\   '~~~'   /~'",
            "          '~~~~~~~~~'",
        ],
        (PetSpecies::AzureWyrm, EvolutionStage::CyberElder) => &[
            "    ✨   ~\\  _👑_  /~   ✨",
            "   *  ~~~(   {E}   )~~~  *  {X}",
            "          \\  (🫧)  /",
            "      .----'~~~~~~~'----.",
            "     (  /~~\\  ★🌊★  /~~\\  )",
            "      '~\\    '~~~'    /~'",
            "         '~~~~~~~~~~~'",
            "       ~ 🌊 ~ 🫧 ~ 🌊 ~",
        ],

        // ── 🦊 Kitsune Nine (Mystic Shrine Fox) ────────────────────────
        (PetSpecies::EmberFox, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  🦊  ✦  '.",
            "      /   ( {E} )   \\",
            "     |   ✦  ~ ~  ✦   |",
            "      \\   '~~~~~'   /",
            "       '._________.'  {X}",
            "       [===✦🦊✦===]",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Baby) => &[
            "       /\\     /\\    {X}",
            "      /  \\___/  \\",
            "     (   {E}   )",
            "      \\   \\ω/   /  .~.",
            "       (  ~~~  )  ( ✦ )",
            "       (\")___(\")   '~'",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Teen) => &[
            "      /\\       /\\     {X}",
            "     / /\\_____/\\ \\   .~. .~.",
            "    (   {E}   ) ( ✦ | ✦ )",
            "     \\    \\ω/    /   \\  |  /",
            "     /  [==⛩️==]  \\___/ /",
            "    (__)_______(__)___/",
        ],
        (PetSpecies::EmberFox, EvolutionStage::Adult) => &[
            "       /\\         /\\       {X}",
            "      / /\\_______/\\ \\   .~. .~. .~.",
            "     (    {E}    ) ( ✦ | ✦ | ✦ )",
            "      \\     \\ω/     /   \\   \\ /   /",
            "      /   [==⛩️==]   \\___\\   V   /",
            "     (   (   ♥   )   )_______/",
            "      (___)_____(___)",
        ],
        (PetSpecies::EmberFox, EvolutionStage::CyberElder) => &[
            "    ✨  /\\   👑   /\\  ✨",
            "   ✦   / /\\_____/\\ \\   .~. .~. .~. .~.",
            "      (   {E}   ) ( ✦ | ★ | ✦ | ✦ )",
            "       \\    \\ω/    /   \\   \\  |  /   /",
            "       /  [==⛩️==]  \\___\\   \\ V /   /",
            "      (  (  ★🦊★  )  )_______\\ /___/",
            "       (___)_____(___)         {X}",
            "        ~ ✦ ~ 🦊 ~ ✦ ~",
        ],

        // ── 🐦‍🔥 Sol Phoenix (Immortal Solar Firebird) ──────────────────
        (PetSpecies::PhoenixChick, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  ☀️   🔥  '.",
            "      /   ( {E} )   \\",
            "     |   \\ / \\ / \\   |",
            "      \\   V   V   V /",
            "       '._________.'  {X}",
            "       [===☀️🔥☀️===]",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Baby) => &[
            "         \\ | /    {X}",
            "        .-\"\"\"-.",
            "       ( {E} )",
            "      /\\  >▽<  /\\",
            "     /  \\_____/  \\",
            "        //   \\\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Teen) => &[
            "          \\ \\ | / /     {X}",
            "         .-'\"\"\"\"\"'-.",
            "    /\\  (   {E}   )  /\\",
            "   /  \\  \\   >▽<   /  /  \\",
            "  / /\\ \\_/  ☀️   ☀️  \\_/ /\\ \\",
            "    \\_____)_______(_____/",
            "          //     \\\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::Adult) => &[
            "           \\ \\ \\ | / / /      {X}",
            "    /\\    .-'\"\"\"☀️\"\"\"'-.    /\\",
            "   /  \\  /  (  {E}  )  \\  /  \\",
            "  / /\\ \\/    \\  >▽<  /    \\/ /\\ \\",
            " ( /  \\___/\\  '-----'  /\\___/  \\ )",
            "  \\_______/  /  🔥☀️🔥  \\  \\_______/",
            "            /__/ | \\__\\",
        ],
        (PetSpecies::PhoenixChick, EvolutionStage::CyberElder) => &[
            "    ✨     \\ \\ \\ 👑 / / /     ✨",
            "   ☀️ /\\  .-'\"\"\"☀️\"\"\"'-.  /\\ ☀️",
            "    /  \\/  (  {E}  )  \\/  \\   {X}",
            "   / /\\ \\   \\  >▽<  /   / /\\ \\",
            "  ( /  \\ \\__/ '---' \\__/ /  \\ )",
            "   \\____\\___/ ★🔥☀️🔥★ \\___/____/",
            "           /__/ || \\__\\",
            "       ~ ☀️ ~ 🔥 ~ ★ ~ 🔥 ~",
        ],

        // ── 🦄 Astra Unicorn (Celestial Prism Steed) ───────────────────
        (PetSpecies::CrystalUni, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  ✨  🦄  '.",
            "      /   ( {E} )   \\",
            "     |   ✧  * *  ✧   |",
            "      \\   '*****'   /",
            "       '._________.'  {X}",
            "       [===✨✧✨===]",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Baby) => &[
            "          /\\     {X}",
            "        /\\||/\\",
            "       ( {E} )",
            "       (  >∪<  )~✨",
            "       /|     |\\",
            "      (_|     |_)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Teen) => &[
            "           /\\       {X}",
            "          /  \\",
            "        /\\ || /\\  ~~✨",
            "       (  {E}  )~~~",
            "        \\  >∪<  /",
            "       /  '---'  \\~~",
            "      (__)     (__)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::Adult) => &[
            "            /\\          {X}",
            "           /✨\\",
            "         /\\ || /\\   ~~~~✨",
            "        (   {E}   )~~~~",
            "         \\   >∪<   / ~~~",
            "        /  .-----.  \\  ~~~",
            "       (  (  ✨🌈✨ )  )",
            "      (__) '-----' (__)",
        ],
        (PetSpecies::CrystalUni, EvolutionStage::CyberElder) => &[
            "    ✨       /\\       ✨",
            "   *       👑/✨\\👑       *   {X}",
            "         /\\  ||  /\\   ~~~~🌟",
            "        (   {E}   )~~~~~",
            "         \\   >∪<   / ~~~~",
            "        /  .-----.  \\  ~~~",
            "       (  ( ★🌈🦄🌈★ ) )",
            "      (__) '-----' (__)",
            "       ~ ✨ ~ 🌈 ~ ✨ ~",
        ],

        // ── 👑 Royal Jelly (Mystic Slime Monarch) ──────────────────────
        (PetSpecies::SlimeKing, EvolutionStage::Egg) => &[
            "         .-\"\"\"\"\"-.",
            "       .'  💧  👑  '.",
            "      /   ( {E} )   \\",
            "     |   ~ ~ ~ ~ ~   |",
            "      \\   o  o  o   /",
            "       '._________.'  {X}",
            "       [===💧👑💧===]",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Baby) => &[
            "         .---.    {X}",
            "       .'  💧  '.",
            "      (  {E}  )",
            "     (   ~~~~~   )",
            "      '---------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Teen) => &[
            "         /\\^/\\     {X}",
            "        | o o |",
            "      .-\"\"\"\"\"\"\"-.",
            "     /  {E}   \\",
            "    (   ~~~~~~~   )",
            "     '-----------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::Adult) => &[
            "         /\\ /\\ /\\      {X}",
            "        |  👑   |",
            "      .-\"\"\"\"\"\"\"\"\"-.",
            "     /   {E}    \\",
            "    (  💧 ~~~~~~~ 💧 )",
            "    (   ~~~~~~~~~   )",
            "     '-------------'",
        ],
        (PetSpecies::SlimeKing, EvolutionStage::CyberElder) => &[
            "    ✨   /\\  ★  /\\   ✨",
            "   *    |  👑💎👑  |    *   {X}",
            "     .-\"\"\"\"\"\"\"\"\"\"\"\"\"-.",
            "    /    {E}     \\",
            "   (  💧  ~~~ ★ ~~~  💧 )",
            "   (    ~~~~~~~~~~~    )",
            "    '-----------------'",
            "      ~ 💧 ~ 👑 ~ 💧 ~",
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
        PetMood::Training => ("•̀ _ •́", "> _ <"),
    }
}

/// The two-character ambient effect for a mood.
fn mood_effect(mood: PetMood) -> &'static str {
    match mood {
        PetMood::Idle => "✨",
        PetMood::Eating => "🍡",
        PetMood::Sleeping => "zZ",
        PetMood::Happy => "💖",
        PetMood::Sick => "💧",
        PetMood::Training => "🔥",
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
/// Frame 0 = eyes open + ambient effect, frame 1 = blink + gentle shift.
pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let template = body_template(species, stage);
    let (eyes_open, eyes_blink) = mood_expression(mood);
    let fx = mood_effect(mood);

    vec![
        compose(template, eyes_open, fx),
        compose(template, eyes_blink, "· "),
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

/// Primary, secondary, and accent RGB colors for each species.
pub fn species_palette(species: PetSpecies) -> (Color, Color, Color) {
    match species {
        PetSpecies::MochiCat => (
            Color::Rgb(255, 182, 206), // soft sakura pink body
            Color::Rgb(255, 235, 242), // cream belly/ears
            Color::Rgb(255, 110, 155), // strawberry accent
        ),
        PetSpecies::BobaPup => (
            Color::Rgb(222, 178, 130), // warm milk-tea tan
            Color::Rgb(145, 102, 68),  // tapioca pearl brown
            Color::Rgb(255, 215, 140), // honey biscuit gold
        ),
        PetSpecies::CloudBun => (
            Color::Rgb(215, 238, 255), // fluffy cloud white-blue
            Color::Rgb(150, 208, 255), // sky ribbon blue
            Color::Rgb(255, 170, 205), // rosy ears
        ),
        PetSpecies::StarPengu => (
            Color::Rgb(135, 195, 255), // aurora ice blue
            Color::Rgb(235, 245, 255), // snow belly
            Color::Rgb(255, 215, 100), // starlight beak/crown
        ),
        PetSpecies::IgnisDragon => (
            Color::Rgb(255, 95, 85),   // crimson dragon scales
            Color::Rgb(255, 185, 75),  // molten gold horns/wings
            Color::Rgb(255, 235, 120), // dragonfire core
        ),
        PetSpecies::AzureWyrm => (
            Color::Rgb(80, 215, 235),  // celestial cyan scales
            Color::Rgb(140, 175, 255), // deep tide crest
            Color::Rgb(220, 250, 255), // moon pearl glow
        ),
        PetSpecies::EmberFox => (
            Color::Rgb(255, 150, 75),  // shrine vermilion-orange fur
            Color::Rgb(255, 235, 190), // cream tail tips
            Color::Rgb(110, 235, 255), // spirit foxfire cyan
        ),
        PetSpecies::PhoenixChick => (
            Color::Rgb(255, 195, 70),  // solar gold plumage
            Color::Rgb(255, 105, 70),  // sunburst scarlet wings
            Color::Rgb(255, 245, 170), // dawn halo
        ),
        PetSpecies::CrystalUni => (
            Color::Rgb(215, 175, 255), // lavender prism coat
            Color::Rgb(150, 235, 255), // astral cyan mane
            Color::Rgb(255, 225, 135), // golden horn
        ),
        PetSpecies::SlimeKing => (
            Color::Rgb(110, 225, 195), // mystic aqua-mint gel
            Color::Rgb(165, 245, 225), // translucent highlight
            Color::Rgb(255, 215, 90),  // royal crown gold
        ),
    }
}

/// Colorize a single line of sprite art character-by-character so eyes,
/// cheeks, crowns, flames, waves, and body outlines each pop in TrueColor.
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
            '🔥' | '☀' | '🌶' => Style::default().fg(Color::Rgb(255, 115, 60)),
            '🌊' | '🫧' | '💧' | '❄' => Style::default().fg(Color::Rgb(110, 220, 255)),
            '✨' | '★' | '⭐' | '✧' | '✦' | '👑' | '🔔' | '*' => Style::default()
                .fg(Color::Rgb(255, 225, 95))
                .add_modifier(Modifier::BOLD),
            '💖' | '♥' | '🌸' | '🎀' | '🍓' => Style::default().fg(Color::Rgb(255, 130, 180)),
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

    let mut lines = vec![
        format!("   {}", get_mood_particles(pet.mood, tick)),
        String::new(),
    ];
    lines.extend(frame.lines().map(|l| l.to_string()));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("  {} (needs cleaning [C]!)", messes.trim()));
    }
    lines
}

/// Multi-colored Ratatui [`Line`]s for the habitat canvas, including a subtle
/// breathing bounce on odd ticks and a ground/pedestal shadow.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];
    let (_, secondary, accent) = species_palette(pet.species);

    let mut out: Vec<Line<'static>> = Vec::new();
    out.push(Line::from(Span::styled(
        format!("   {}", get_mood_particles(pet.mood, tick)),
        Style::default().fg(accent),
    )));

    // Gentle vertical breathing bounce on even ticks when awake.
    if tick.is_multiple_of(2) && !pet.is_sleeping {
        out.push(Line::from(""));
    }

    for line in frame.lines() {
        out.push(colorize_sprite_line(line, pet.species, pet.stage));
    }

    let ground = match pet.species.family() {
        "Mythical & Fantasy" => "   ═══════✦═════════✦═══════",
        _ => "   ░░░░░░░🌸░░░░░░░🌸░░░░░░░",
    };
    out.push(Line::from(Span::styled(
        ground.to_string(),
        Style::default().fg(secondary),
    )));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        out.push(Line::from(Span::styled(
            format!("   {} (needs bubble bath [C]!)", messes.trim()),
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
        ShellTheme::StrawberryPink => Color::Rgb(255, 130, 175),
        ShellTheme::MatchaGreen => Color::Rgb(130, 215, 155),
        ShellTheme::SkyBlue => Color::Rgb(115, 195, 255),
        ShellTheme::LavenderDream => Color::Rgb(190, 150, 255),
        ShellTheme::SunsetGold => Color::Rgb(255, 195, 95),
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
    fn test_render_animated_and_colored_sprite_and_speech_bubble() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let colored = render_colored_sprite(&pet, 0);
        assert!(!colored.is_empty());

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
            let colored_preview = render_colored_preview(*species, EvolutionStage::Adult, 0);
            assert!(!colored_preview.is_empty());
        }
        let _ = sky_scene(500);
    }
}
