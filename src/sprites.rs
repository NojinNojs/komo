//! TrueColor Half-Block Pixel Art Engine (`▀`, `▄`, `█`) for **Mochigotchi v1.0.0**.
//!
//! Every companion is rendered as a genuine **24×16 pixel bitmap** using
//! Unicode half-blocks (`▀` upper half, `▄` lower half, `█` full block, and
//! transparent space). Two vertical bitmap pixels pack into a single terminal
//! cell (`1:1` square pixel aspect ratio), producing crisp 16-bit handheld
//! V-Pet sprites that never misalign or wrap on any terminal font.
//!
//! Palette codes:
//! - `.` = Transparent background
//! - `O` = Dark pixel outline (`#2a243e`)
//! - `B` = Primary species body color
//! - `S` = Secondary / belly / highlight color
//! - `A` = Accent color (horns, flames, pearl, crown, markings)
//! - `W` = Pure white body/belly/cloud/pearl (never affected by eye blinking!)
//! - `E` = Eye shine white (turns into eyelid `O` when blinking/sleeping)
//! - `K` = Eye pupil dark (turns into eyelid `O` when blinking/sleeping)
//! - `P` = Rosy cheek / heart pink
//! - `G` = Royal Gold

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

const CANVAS_W: usize = 24;
const CANVAS_H: usize = 16;

/// Hand-crafted 24×16 pixel art bitmaps for each species (Adult form).
/// Uses `E` + `K` for eyes so white body parts (`W`) never flicker on blink!
fn adult_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        // 🐱 Neko Mochi
        PetSpecies::MochiCat => [
            "........................",
            "....OO..........OO......",
            "...OBBO........OBBO.....",
            "..OBPPBOOOOOOOOBPPBO....",
            "..OBBBBBBBBBBBBBBBBO....",
            ".OBBBBBBBBBBBBBBBBBBO...",
            ".OBBEEKBBBBBBBBEEKBBO...",
            ".OBBKKKBBBBBBBBKKKBBO...",
            ".OBBPPBBBBAAABBBBPPBO...",
            "..OBBBBBBOOGGOOBBBBBO...",
            "...OOOOOBSSSSSSBOOOO....",
            "...OBBBBSSSSSSSSBBBBO...",
            "...OBBBBSSSSSSSSBBBBO.OO",
            "...OBBBBBBBBBBBBBBBBOOBA",
            "....OOBBBOOOOOOBBBOOOBA.",
            ".....OOOO......OOOO.OO..",
        ],
        // 🐶 Boba Pup
        PetSpecies::BobaPup => [
            "........................",
            "...OOOOO........OOOOO...",
            "..OSSSSSOOOOOOOOSSSSSO..",
            ".OSSSSSBBBBBBBBBBSSSSSO.",
            ".OSSSSBBBBBBBBBBBBSSSSO.",
            ".OSSSBBEEKBBBBEEKBBSSSO.",
            "..OOOBBKKKBBBBKKKBBOOO..",
            "...OBBBBSSSSSSSSBBBBBO..",
            "...OBBBSSSOOOOSSSBbbBO..",
            "....OOOBBBOAAOBBBOOOO...",
            "...OBBBBSSSSSSSSBBBBO...",
            "...OBBBBSOSOSOSSBBBBO.OO",
            "...OBBBBSSSSSSSSBBBBOOBS",
            "....OBBBBBBBBBBBBBBOOBA.",
            "....OOBBBOOOOOOBBBOOOO..",
            ".....OOOO......OOOO.....",
        ],
        // 🐰 Cloud Bun
        PetSpecies::CloudBun => [
            ".....OOO........OOO.....",
            "....OBPBO......OBPBO....",
            "....OBPBO......OBPBO....",
            "....OBPBO......OBPBO....",
            "....OBBBOOOOOOOOBBBO....",
            "...OBBBBBBBBBBBBBBBBO...",
            "..OBBEEKBBBBBBBBEEKBBO..",
            "..OBBKKKBBBBBBBBKKKBBO..",
            "..OBBPPBBBBPPBBBBPPBBO..",
            "...OBBBBBOOOOOOBBBBBO...",
            "..OOBBBBBSSSSSSBBBBBOO..",
            ".OWWBBBBSSSSSSSSBBBBWWO.",
            ".OWWWBBBBBBBBBBBBBBWWWO.",
            "..OOWWBBBBBBBBBBBBWWOO..",
            "....OOBBBOOOOOOBBBOO....",
            ".....OOOO......OOOO.....",
        ],
        // 🐧 Star Pengu (uses W for snow white belly, E/K for eyes!)
        PetSpecies::StarPengu => [
            "........................",
            ".......OOOOOOOOOO.......",
            ".....OOBBBBBBBBBBOO.....",
            "....OBBBBBGAAGBBBBBO....",
            "...OBBBBWWWWWWWWBBBBO...",
            "...OBBBEEKWWWWEEKBBBO...",
            "...OBBBKKKWWWWKKKBBBO...",
            "...OBBBWWWWGGWWWWBBBO...",
            "..OOBBWWWWGGGGWWWWBBOO..",
            ".OBBBBAAAAAAAAAAAAABBBO.",
            ".OBBBWWWWWWWWWWWWWWBBBO.",
            ".OBBBWWWWWWGGWWWWWWBBBO.",
            "..OOBWWWWWWWWWWWWWWBOO..",
            "...OOWWWWWWWWWWWWWWOO...",
            "....OOGGGOOOOOOGGGOO....",
            ".....OOOO......OOOO.....",
        ],
        // 🐉 Ignis Drake
        PetSpecies::IgnisDragon => [
            "...AA..............AA...",
            "..GAAG....OOOO....GAAG..",
            "...OAAOOOBBBBBBOOAAO....",
            "....OOBBBBBBBBBBBBOO....",
            ".OO..OBBEEKBBEEKBBO..OO.",
            "OAAO.OBBKKKBBKKKBBO.OAAO",
            "OABBOOBBBBBAAABBBBOOBBAO",
            "OABBBOOOOBSSSSBOOOOBBBAO",
            ".OABBBBBSSSSSSSSBBBBBAO.",
            "..OOBBBBSSSSSSSSBBBBOO..",
            "...OBBBBSSSSSSSSBBBBO.AA",
            "...OBBBBSSSSSSSSBBBBOAGG",
            "....OBBBBBBBBBBBBBBOOAA.",
            "....OOBBBOOOOOOBBBOOO...",
            ".....OOOO......OOOO.....",
            "........................",
        ],
        // 🐲 Azure Wyrm
        PetSpecies::AzureWyrm => [
            "....AA............AA....",
            "...OAOO..OOOOOO..OOAO...",
            "....OOAOBBBBBBBBOAO.....",
            ".....OBBBBBBBBBBBBO.....",
            "....OBBEEKBBBBEEKBBO....",
            "....OBBKKKBBBBKKKBBO....",
            "...OOBBBBBSSSSBBBBBOO...",
            "..OABBOOOOBWWBOOOOBBAO..",
            ".OABBBBBBOWWWWOBBBBBBAO.",
            ".OABBSSSSOOWWOOSSSSBBAO.",
            "..OOBSSSSBBBBBBSSSSBOO..",
            "...OOOBBBBBBBBBBBBOOO...",
            "..OABBBBSSSSSSSSBBBBAO..",
            ".OABBBOOOOOOOOOOOOBBBAO.",
            "..OOOO............OOOO..",
            "........................",
        ],
        // 🦊 Kitsune Nine
        PetSpecies::EmberFox => [
            "..OOO..............OOO..",
            ".OBWBO............OBWBO.",
            ".OBWWBO..OOOOOO..OBWWBO.",
            "..OBBBBOOBBBBBBOOBBBBO..",
            "...OBBBBBBBBBBBBBBBBO...",
            "..OBBEEKBBBBBBBBEEKBBO..",
            "..OBBKKKBBBBBBBBKKKBBO..",
            "..OBBPPBBBWWWWBBBPPBBO..",
            "...OOBBBBBWAAWBBBBBOO...",
            ".AA..OOOBBSSSSBBOOO..AA.",
            "AWWA..OBBSSSSSSBBO..AWWA",
            ".AA..OBBBSSSSSSBBBO..AA.",
            "....OBBBBBBBBBBBBBBOOO..",
            "...OOBBBOOOOOOBBBOOSSS0.",
            "....OOOO......OOOO.OSSO.",
            "....................OO..",
        ],
        // 🦅 Sol Phoenix
        PetSpecies::PhoenixChick => [
            ".........GG..GG.........",
            "........OGGAAGGO........",
            "..OO...OBBBBBBBBO...OO..",
            ".OAAO.OBBBBBBBBBBO.OAAO.",
            "OAGGGOOBBEEKWEEKBBOOGGGA",
            "OAGBBBOOBKKKGGGKKBOOBBGA",
            "OABBBBBOOBBGGGGBOOBBBBAO",
            ".OABBBBBBBBBBBBBBBBBBAO.",
            "..OOBBBBBSSSSSSBBBBBOO..",
            "...OOOBBSSSSSSSSBBOOO...",
            ".....OBBSSSSSSSSBBO.....",
            ".....OBBBBBBBBBBBBO.....",
            "....OOGGGOOOOOOGGGOO....",
            "...OAGAGAO....OAGAGAO...",
            "....OOOOO......OOOOO....",
            "........................",
        ],
        // 🦄 Astra Unicorn
        PetSpecies::CrystalUni => [
            "...........GG...........",
            "..........OGGO..........",
            "......OO..OGGO..OO......",
            ".....OBBOOOGGOOOBBO.....",
            "....OBBBBBBBBBBBBBBO....",
            "...OBBEEKBBBBBBEEKBBO...",
            "...OBBKKKBBBBBBKKKBBO...",
            "...OBBPPBBBBBBBBPPBBO...",
            "....OOBBBBBBBBBBBBOOAA..",
            "...OOBBBBSSSSSSBBBBOSSA.",
            "..OABBBBBSSSSSSBBBBBOAA.",
            "..OABBBBBBBBBBBBBBBBOO..",
            "...OOBBBOOOOOOOOBBBOO...",
            "...OOGGGO......OOGGGO...",
            "....OOOO........OOOO....",
            "........................",
        ],
        // 👑 Royal Jelly
        PetSpecies::SlimeKing => [
            "........................",
            "......GG...GG...GG......",
            ".....OGGGOOGGGOOGGO.....",
            ".....OGGGGGGGGGGGGO.....",
            ".....OOOOOOOOOOOOOO.....",
            "...OOBBBBBBBBBBBBBBOO...",
            "..OBBWWBBBBBBBBBBBBBBBO.",
            ".OBBWWBBBBBBBBBBBBBBBBO.",
            ".OBBBBEEKBBBBBBEEKBBBBO.",
            ".OBBBBKKKBBBBBBKKKBBBBO.",
            ".OBBPPBBBBWWWWBBBBPPBBO.",
            ".OBBBBBBBSSSSSSBBBBBBBO.",
            ".OBBBBBBBBBBBBBBBBBBBBO.",
            "..OOBBBBBBBBBBBBBBBBOO..",
            "....OOOOOOOOOOOOOOOO....",
            "........................",
        ],
    }
}

/// Proper closed-outline Teen stage 24×16 bitmaps (sleeker intermediate form).
fn teen_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick => [
            "........................",
            ".....AA..........AA.....",
            "....OAAO..OOOO..OAAO....",
            ".....OOBBBBBBBBBBOO.....",
            "...OO.OBBEEKBBEEKBBO.OO.",
            "..OAAOOBBKKKBBKKKBBOOAAO",
            "..OABBBBBBBBAAABBBBBBAO.",
            "...OABBBBSSSSSSSSBBBBAO.",
            "....OOBBBSSSSSSSSBBBOO..",
            ".....OBBBSSSSSSSSBBBO...",
            ".....OBBBBBBBBBBBBBBO.AA",
            "......OOBBBOOOOBBBOOOAA.",
            ".......OOOO....OOOO.OO..",
            "........................",
            "........................",
            "........................",
        ],
        PetSpecies::AzureWyrm | PetSpecies::CrystalUni => [
            "........................",
            ".......AA......AA.......",
            "......OAOOOOOOOAO.......",
            ".....OBBBBBBBBBBBBO.....",
            "....OBBEEKBBBBEEKBBO....",
            "....OBBKKKBBBBKKKBBO....",
            "....OBBPPBBBBBBPPBBO....",
            ".....OOBBSSSSSSBBOO.....",
            "....OABBBSSSSSSBBBAO....",
            "...OABBBBSSSSSSBBBBAO...",
            "....OOBBBBBBBBBBBBOO....",
            ".....OOBBBOOOOBBBOO.....",
            "......OOOO....OOOO......",
            "........................",
            "........................",
            "........................",
        ],
        PetSpecies::SlimeKing => [
            "........................",
            "........................",
            "........GG..GG..........",
            ".......OGGGGGGO.........",
            ".......OOOOOOOO.........",
            ".....OOBBBBBBBBOO.......",
            "....OBBWWBBBBBBBBBO.....",
            "...OBBBEEKBBBBEEKBBO....",
            "...OBBBKKKBBBBKKKBBO....",
            "...OBBPPBBBWWBBBPPBBO...",
            "...OBBBBBSSSSSSBBBBBO...",
            "....OOBBBBBBBBBBBBOO....",
            "......OOOOOOOOOOOO......",
            "........................",
            "........................",
            "........................",
        ],
        _ => [
            "........................",
            ".....OOO........OOO.....",
            "....OBBBOOOOOOOOBBBO....",
            "...OBBBBBBBBBBBBBBBBO...",
            "...OBBEEKBBBBBBEEKBBO...",
            "...OBBKKKBBBBBBKKKBBO...",
            "...OBBPPBBBAAABBBPPBO...",
            "....OOBBBBSSSSBBBBOO....",
            "...OOBBBBBSSSSBBBBBOO...",
            "...OBBBBBSSSSSSBBBBBO...",
            "....OBBBBBBBBBBBBBBO....",
            ".....OOBBBOOOOBBBOO.....",
            "......OOOO....OOOO......",
            "........................",
            "........................",
            "........................",
        ],
    }
}

/// Species-customized Egg stage 24×16 bitmap with shell motifs.
fn egg_bitmap(species: PetSpecies, frame_idx: usize) -> [&'static str; CANVAS_H] {
    let mut base = if frame_idx.is_multiple_of(2) {
        [
            "........................",
            "........OOOOOOOO........",
            "......OOBBBBBBBBOO......",
            ".....OBBBWWBBBBBBBO.....",
            "....OBBBWWBBBBAAABBO....",
            "....OBBBBBBBBBAAABBO....",
            "...OBBBBAAABBBBBBBBBO...",
            "...OBBBBAAABBBBSSSBBO...",
            "...OBBBBBBBBBBBSSSBBO...",
            "...OBBBSSSBBBBBBBBBBO...",
            "....OBBSSSBBBBAAABBO....",
            "....OBBBBBBBBBAAABBO....",
            ".....OOBBBBBBBBBBOO.....",
            "....OOSSSSSSSSSSSSOO....",
            "...OAAAAAAAAAAAAAAAAO...",
            "....OOOOOOOOOOOOOOOO....",
        ]
    } else {
        [
            "........................",
            ".........OOOOOOOO.......",
            ".......OOBBBBBBBBOO.....",
            "......OBBBWWBBBBBBBO....",
            ".....OBBBWWBBBBAAABBO...",
            ".....OBBBBBBBBBAAABBO...",
            "....OBBBBAAABBBBBBBBBO..",
            "....OBBBBAAABBBBSSSBBO..",
            "....OBBBBBBBBBBBSSSBBO..",
            "....OBBBSSSBBBBBBBBBBO..",
            ".....OBBSSSBBBBAAABBO...",
            ".....OBBBBBBBBBAAABBO...",
            ".....OOBBBBBBBBBBOO.....",
            "....OOSSSSSSSSSSSSOO....",
            "...OAAAAAAAAAAAAAAAAO...",
            "....OOOOOOOOOOOOOOOO....",
        ]
    };

    if matches!(
        species,
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::SlimeKing
    ) {
        base[0] = ".........GG..GG.........";
    } else if matches!(species, PetSpecies::MochiCat | PetSpecies::CloudBun | PetSpecies::EmberFox)
    {
        base[0] = ".......OO......OO.......";
    }
    base
}

/// Baby stage 24×16 bitmap (cute compact chibi companion).
fn baby_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    let has_horns = matches!(
        species,
        PetSpecies::IgnisDragon
            | PetSpecies::AzureWyrm
            | PetSpecies::CrystalUni
            | PetSpecies::PhoenixChick
            | PetSpecies::SlimeKing
    );
    if has_horns {
        [
            "........................",
            "........................",
            "......AA........AA......",
            ".....OAAOOOOOOOAAO......",
            "....OBBBBBBBBBBBBBO.....",
            "...OBBBBBBBBBBBBBBBO....",
            "...OBBEEKBBBBEEKBBBO....",
            "...OBBKKKBBBBKKKBBBO....",
            "...OBBPPBBBBBBPPBBBO....",
            "...OBBBBBSSSSBBBBBBO....",
            "....OBBBBSSSSBBBBBO.....",
            "....OOBBBBBBBBBBBOO.....",
            ".....OOBBBOOOOBBBOO.....",
            "......OOOO....OOOO......",
            "........................",
            "........................",
        ]
    } else {
        [
            "........................",
            "........................",
            ".....OOO........OOO.....",
            "....OBBBOOOOOOOOBBBO....",
            "....OBBBBBBBBBBBBBBO....",
            "...OBBBBBBBBBBBBBBBBO...",
            "...OBBEEKBBBBBBEEKBBO...",
            "...OBBKKKBBBBBBKKKBBO...",
            "...OBBPPBBBBBBBBPPBBO...",
            "...OBBBBBSSSSSSBBBBBO...",
            "....OBBBBSSSSSSBBBBO....",
            "....OOBBBBBBBBBBBBOO....",
            ".....OOBBBOOOOBBBOO.....",
            "......OOOO....OOOO......",
            "........................",
            "........................",
        ]
    }
}

/// Build the 24×16 character grid for any `(species, stage, mood, frame_idx)`.
fn build_grid(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
) -> [[char; CANVAS_W]; CANVAS_H] {
    let raw = match stage {
        EvolutionStage::Egg => egg_bitmap(species, frame_idx),
        EvolutionStage::Baby => baby_bitmap(species),
        EvolutionStage::Teen => teen_bitmap(species),
        EvolutionStage::Adult | EvolutionStage::CyberElder => adult_bitmap(species),
    };

    let mut grid = [['.'; CANVAS_W]; CANVAS_H];
    for (y, row) in raw.iter().enumerate() {
        for (x, ch) in row.chars().take(CANVAS_W).enumerate() {
            grid[y][x] = ch;
        }
    }

    // CyberElder form: stamp a gleaming golden crown & starlight sparkles in the corners
    if stage == EvolutionStage::CyberElder {
        let crown = ["..G.G.G..", "..GGGGG..", "..OOOOO.."];
        for (cy, crow) in crown.iter().enumerate() {
            for (cx, cch) in crow.chars().enumerate() {
                if cch != '.' && cx + 7 < CANVAS_W && cy < CANVAS_H {
                    grid[cy][cx + 7] = cch;
                }
            }
        }
        grid[1][2] = 'G';
        grid[2][21] = 'G';
        grid[12][1] = 'W';
        grid[11][22] = 'W';
    }

    // Blink / sleeping eye transform: ONLY turn `E` and `K` eye pixels into closed eyelids `O`!
    // White body/belly/cloud pixels (`W`) remain untouched!
    let blink = frame_idx % 2 == 1 || mood == PetMood::Sleeping;
    if stage != EvolutionStage::Egg && blink {
        for row in &mut grid {
            for cell in row.iter_mut() {
                if *cell == 'E' || *cell == 'K' {
                    *cell = 'O';
                }
            }
        }
    } else if stage != EvolutionStage::Egg && mood == PetMood::Sick {
        for row in &mut grid {
            for cell in row.iter_mut() {
                if *cell == 'E' {
                    *cell = 'P';
                }
            }
        }
    }

    // Gentle 1-pixel vertical breathing bounce on frame 1 when awake & hatched
    if stage != EvolutionStage::Egg && mood != PetMood::Sleeping && frame_idx % 2 == 1 {
        let mut shifted = [['.'; CANVAS_W]; CANVAS_H];
        shifted[..CANVAS_H - 1].copy_from_slice(&grid[1..CANVAS_H]);
        grid = shifted;
    }

    // Stamp a 4×4 pixel mood badge in the top-right corner on frame 0
    if stage != EvolutionStage::Egg && frame_idx.is_multiple_of(2) {
        let badge: [&str; 4] = match mood {
            PetMood::Happy => [".P.P", "PPPP", ".PP.", "..P."],
            PetMood::Sleeping => ["GGGG", "..G.", ".G..", "GGGG"],
            PetMood::Eating => [".GG.", "GWWG", "GWWG", ".GG."],
            PetMood::Sick => [".P..", "PPP.", ".P..", "..P."],
            PetMood::Training => ["..G.", ".GGG", "GGA.", ".G.."],
            PetMood::Idle => ["..G.", ".GGG", "..G.", "...."],
        };
        for (by, brow) in badge.iter().enumerate() {
            for (bx, bch) in brow.chars().enumerate() {
                if bch != '.' && bx + 19 < CANVAS_W && by < CANVAS_H {
                    grid[by][bx + 19] = bch;
                }
            }
        }
    }

    grid
}

/// Primary, secondary, and accent RGB colors for each species.
pub fn species_palette(species: PetSpecies) -> (Color, Color, Color) {
    match species {
        PetSpecies::MochiCat => (
            Color::Rgb(255, 175, 205),
            Color::Rgb(255, 235, 245),
            Color::Rgb(255, 95, 145),
        ),
        PetSpecies::BobaPup => (
            Color::Rgb(220, 170, 120),
            Color::Rgb(130, 88, 56),
            Color::Rgb(255, 215, 120),
        ),
        PetSpecies::CloudBun => (
            Color::Rgb(210, 235, 255),
            Color::Rgb(255, 255, 255),
            Color::Rgb(255, 155, 195),
        ),
        PetSpecies::StarPengu => (
            Color::Rgb(110, 180, 255),
            Color::Rgb(240, 248, 255),
            Color::Rgb(255, 210, 85),
        ),
        PetSpecies::IgnisDragon => (
            Color::Rgb(245, 80, 75),
            Color::Rgb(255, 205, 115),
            Color::Rgb(255, 155, 50),
        ),
        PetSpecies::AzureWyrm => (
            Color::Rgb(65, 210, 235),
            Color::Rgb(195, 245, 255),
            Color::Rgb(120, 155, 255),
        ),
        PetSpecies::EmberFox => (
            Color::Rgb(255, 140, 65),
            Color::Rgb(255, 238, 205),
            Color::Rgb(90, 235, 255),
        ),
        PetSpecies::PhoenixChick => (
            Color::Rgb(255, 185, 55),
            Color::Rgb(255, 240, 165),
            Color::Rgb(255, 90, 60),
        ),
        PetSpecies::CrystalUni => (
            Color::Rgb(210, 165, 255),
            Color::Rgb(245, 235, 255),
            Color::Rgb(120, 230, 255),
        ),
        PetSpecies::SlimeKing => (
            Color::Rgb(95, 225, 190),
            Color::Rgb(180, 250, 230),
            Color::Rgb(255, 215, 80),
        ),
    }
}

fn pixel_color(code: char, species: PetSpecies) -> Option<Color> {
    let (primary, secondary, accent) = species_palette(species);
    match code {
        '.' => None,
        'O' | '0' => Some(Color::Rgb(42, 36, 62)),
        'B' | 'b' => Some(primary),
        'S' | 's' => Some(secondary),
        'A' => Some(accent),
        'W' | 'E' => Some(Color::Rgb(250, 252, 255)),
        'K' => Some(Color::Rgb(22, 18, 36)),
        'P' => Some(Color::Rgb(255, 110, 165)),
        'G' => Some(Color::Rgb(255, 220, 75)),
        _ => Some(primary),
    }
}

/// Convert a 24×16 pixel grid into 8 half-block terminal [`Line`]s.
fn render_grid_colored(
    grid: &[[char; CANVAS_W]; CANVAS_H],
    species: PetSpecies,
) -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(CANVAS_H / 2);
    for pair in 0..(CANVAS_H / 2) {
        let top_row = &grid[pair * 2];
        let bot_row = &grid[pair * 2 + 1];
        let mut spans = Vec::with_capacity(CANVAS_W);

        for x in 0..CANVAS_W {
            let t = pixel_color(top_row[x], species);
            let b = pixel_color(bot_row[x], species);
            let span = match (t, b) {
                (None, None) => Span::raw(" "),
                (Some(tc), None) => Span::styled("▀", Style::default().fg(tc)),
                (None, Some(bc)) => Span::styled("▄", Style::default().fg(bc)),
                (Some(tc), Some(bc)) if tc == bc => Span::styled("█", Style::default().fg(tc)),
                (Some(tc), Some(bc)) => Span::styled("▀", Style::default().fg(tc).bg(bc)),
            };
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }
    lines
}

/// Convert a 24×16 pixel grid into 8 monochrome half-block String lines.
fn render_grid_plain(grid: &[[char; CANVAS_W]; CANVAS_H]) -> String {
    let mut rows = Vec::with_capacity(CANVAS_H / 2);
    for pair in 0..(CANVAS_H / 2) {
        let top_row = &grid[pair * 2];
        let bot_row = &grid[pair * 2 + 1];
        let mut line = String::with_capacity(CANVAS_W);
        for x in 0..CANVAS_W {
            let t = top_row[x] != '.';
            let b = bot_row[x] != '.';
            let ch = match (t, b) {
                (false, false) => ' ',
                (true, false) => '▀',
                (false, true) => '▄',
                (true, true) => '█',
            };
            line.push(ch);
        }
        rows.push(line);
    }
    rows.join("\n")
}

/// Both half-block animation frames for a species at a given stage & mood.
pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let g0 = build_grid(species, stage, mood, 0);
    let g1 = build_grid(species, stage, mood, 1);
    vec![render_grid_plain(&g0), render_grid_plain(&g1)]
}

/// Ambient retro status particles above the pixel sprite.
pub fn get_mood_particles(mood: PetMood, tick: usize) -> &'static str {
    let frames: &[&str] = match mood {
        PetMood::Idle => &["·  ✦  ·  ★  ·  ✦  ·", "✦  ·  ★  ·  ✦  ·  ★"],
        PetMood::Eating => &["♪  ♥  ★  ♥  ♪  ★  ♪", "★  ♪  ♥  ♪  ★  ♥  ★"],
        PetMood::Sleeping => &["☾  z  Z  zZ ·  ✦  ☾", "✦  Z  zZ z  ☾  ·  ✦"],
        PetMood::Happy => &["✦  ♥  ★  ♥  ✦  ♥  ✦", "♥  ✦  ♥  ✦  ★  ✦  ♥"],
        PetMood::Sick => &[".  ~  x  ~  .  x  .", "~  .  x  .  ~  .  x"],
        PetMood::Training => &["★  ♦  ▲  ♦  ★  ▲  ★", "♦  ★  ▲  ★  ♦  ★  ♦"],
    };
    frames[tick % frames.len()]
}

/// Plain String lines of the current animation frame (used by CLI & tests).
pub fn render_animated_sprite(pet: &Pet, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = &frames[tick % frames.len()];

    let mut lines = vec![get_mood_particles(pet.mood, tick).to_string()];
    lines.extend(frame.lines().map(|l| l.to_string()));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("{} (needs bath [C]!)", messes.trim()));
    }
    lines
}

/// TrueColor Half-Block Pixel Art [`Line`]s for the habitat LCD screen.
/// Uses a natural blink cadence (`tick % 6 == 5`) instead of strobing every 250ms.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 6 == 5);
    let grid = build_grid(pet.species, pet.stage, pet.mood, frame_idx);
    let (_, secondary, accent) = species_palette(pet.species);

    let mut out = Vec::with_capacity(11);
    out.push(Line::from(Span::styled(
        get_mood_particles(pet.mood, tick).to_string(),
        Style::default().fg(accent),
    )));
    out.extend(render_grid_colored(&grid, pet.species));
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        Style::default().fg(secondary),
    )));

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        out.push(Line::from(Span::styled(
            format!("{} (press [C] to bath)", messes.trim()),
            Style::default().fg(Color::LightRed),
        )));
    }

    out
}

/// TrueColor Half-Block Pixel Art [`Line`]s for the Step 1 Companion Selector.
pub fn render_colored_preview(
    species: PetSpecies,
    stage: EvolutionStage,
    tick: usize,
) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 6 == 5);
    let grid = build_grid(species, stage, PetMood::Happy, frame_idx);
    let (_, secondary, _) = species_palette(species);
    let mut out = render_grid_colored(&grid, species);
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        Style::default().fg(secondary),
    )));
    out
}

/// A friendly preview of a species (adult form, idle) as plain strings.
pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// Concise retro LCD speech line.
pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 80 {
            "💬 \"*crack* Almost ready to hatch!\"".to_string()
        } else {
            "💬 \"A warm heartbeat waits inside...\"".to_string()
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
        format!("💬 \"{} is happy to see you!\"", pet.name),
        format!("💬 \"Craving {}...\"", pet.species.favorite_snack()),
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
        0 => ("🌞 DAY", "Sunny"),
        1 => ("🌇 DUSK", "Breezy"),
        _ => ("🌙 NIGHT", "Starlit"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_species_stages_and_moods_render_half_block_pixel_sprites() {
        for species in PetSpecies::all() {
            for stage in EvolutionStage::all() {
                for mood in PetMood::all() {
                    let frames = get_sprite_frames(*species, *stage, *mood);
                    assert_eq!(frames.len(), 2, "{species:?}/{stage:?}/{mood:?}");
                    for frame in &frames {
                        assert!(!frame.trim().is_empty());
                        let lines: Vec<&str> = frame.lines().collect();
                        assert_eq!(lines.len(), CANVAS_H / 2);
                        for line in lines {
                            assert_eq!(line.chars().count(), CANVAS_W);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_white_body_pixels_do_not_turn_dark_when_blinking() {
        // Verify StarPengu's white snow belly ('W') stays white on blink frame 1!
        let g0 = build_grid(PetSpecies::StarPengu, EvolutionStage::Adult, PetMood::Idle, 0);
        let g1 = build_grid(PetSpecies::StarPengu, EvolutionStage::Adult, PetMood::Idle, 1);
        let w_count_0 = g0.iter().flatten().filter(|&&c| c == 'W').count();
        let w_count_1 = g1.iter().flatten().filter(|&&c| c == 'W').count();
        assert_eq!(w_count_0, w_count_1, "white body pixels 'W' must never turn into eyelid 'O'");
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
            assert_eq!(preview.len(), CANVAS_H / 2);
            let colored_preview = render_colored_preview(*species, EvolutionStage::Adult, 0);
            assert!(!colored_preview.is_empty());
        }
        let _ = sky_scene(500);
    }
}
