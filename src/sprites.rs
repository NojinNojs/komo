//! High-Detail 32×20 TrueColor Half-Block Pixel Art Engine (`▀`, `▄`, `█`) for
//! **Mochigotchi v1.1.0**.
//!
//! Every companion is rendered as a genuine **32×20 pixel bitmap** using
//! Unicode half-blocks (`▀` upper half, `▄` lower half, `█` full block, and
//! transparent space). Two vertical bitmap pixels pack into a single terminal
//! cell (`1:1` square pixel aspect ratio, 32 cols × 10 rows), with a 10-tone
//! retro pixel palette per species:
//! - `.` = Transparent background
//! - `O` = Dark pixel outline (`#201b32`)
//! - `D` = Deep shadow (`0.72 × primary`) for 16-bit depth & shading
//! - `B` = Primary species body color
//! - `L` = Light highlight (`1.22 × primary`) on crests, scales & domes
//! - `S` = Secondary / belly / highlight color
//! - `A` = Accent color (horns, flames, pearl, ribbons, markings)
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

pub const CANVAS_W: usize = 32;
pub const CANVAS_H: usize = 20;

/// Ultra-detailed 32×20 pixel art bitmaps for each species (Adult form).
fn adult_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        // 🐱 Neko Mochi — pointed ears with pink inner, bell collar, striped tail, shaded paws
        PetSpecies::MochiCat => [
            "................................",
            ".....OOO................OOO.....",
            "....OBLBO..............OBLBO....",
            "...OBPPPBO............OBPPPBO...",
            "...OBPPPBOOOOOOOOOOOOOBPPPBO....",
            "..OBLLLLBBBBBBBBBBBBBBBBLLLBO...",
            "..OBBBBBBBBBBBBBBBBBBBBBBBBBO...",
            ".ODBBEEKKBBBBBBBBBBBBEEKKBBDO...",
            ".ODBBKKKKBBBBBBBBBBBBKKKKBBDO...",
            ".ODBBPPPBBBBBBAAABBBBBPPPBBDO...",
            "..ODBBBBBBBBOOGGGOOBBBBBBBDO....",
            "...OOOOOOOBBSSSSSSSSBBOOOOO.....",
            "...ODBBBBSSSSSSSSSSSSSSBBBDO..OO",
            "..ODBBBBBSSSSSSSSSSSSSSBBBBDOOBA",
            "..ODBBBBBSSSSSSSSSSSSSSBBBBDOBBA",
            "...ODBBBBBBSSSSSSSSSSBBBBBBDOBA.",
            "...OODBBBBBBBBBBBBBBBBBBBBDOOO..",
            "....OOODBBBOOOOOOOOOBBBDOOO.....",
            "......OOOOO........OOOOO........",
            "................................",
        ],
        // 🐶 Boba Pup — floppy dark ears, biscuit snout, boba pearls on belly, wagging tail
        PetSpecies::BobaPup => [
            "................................",
            "....OOOOOO............OOOOOO....",
            "...OSSSSSSOOOOOOOOOOOOSSSSSSO...",
            "..OSSSSSSLLLBBBBBBBBLLLSSSSSSO..",
            ".OSSSSSSBBBBBBBBBBBBBBBBSSSSSSO.",
            ".OSSSSSBBBBBBBBBBBBBBBBBBSSSSSO.",
            ".OSSSSBBEEKKBBBBBBBBEEKKBBSSSSO.",
            "..OOOOBBKKKKBBBBBBBBKKKKBBOOOO..",
            "...ODBBBBPPPSSSSSSSSPPPBBBBDO...",
            "...ODBBBBSSSSOOOOOOSSSSBBBBDO...",
            "....OOOOOBBBBOOAAOOBBBBOOOOO....",
            "...ODBBBBBSSSSSSSSSSSSBBBBBDO...",
            "..ODBBBBBBSSOSOSOSOSSBBBBBBDO.OO",
            "..ODBBBBBBSSSSSSSSSSSSBBBBBBDOBS",
            "...ODBBBBBBSSOSSOSSOSBBBBBBDOBBA",
            "...ODBBBBBBBBBBBBBBBBBBBBBBDOOO.",
            "....OOODBBBOOOOOOOOOBBBDOOOO....",
            "......OOOOO........OOOOO........",
            "................................",
            "................................",
        ],
        // 🐰 Cloud Bun — long pastel bunny ears, fluffy cloud skirt, rosy cheeks
        PetSpecies::CloudBun => [
            "......OOOO............OOOO......",
            ".....OBLPBO..........OBLPBO.....",
            ".....OBPPBO..........OBPPBO.....",
            ".....OBPPBO..........OBPPBO.....",
            ".....OBBBBO..........OBBBBO.....",
            ".....OBBBBOOOOOOOOOOOOBBBBO.....",
            "....OBLLLBBBBBBBBBBBBBBLLLBO....",
            "...OBBBBBBBBBBBBBBBBBBBBBBBBO...",
            "..ODBBEEKKBBBBBBBBBBBBEEKKBBDO..",
            "..ODBBKKKKBBBBBBBBBBBBKKKKBBDO..",
            "..ODBBPPPBBBBBBPPBBBBBBPPPBBDO..",
            "...ODBBBBBBBBOOOOOOBBBBBBBBDO...",
            "..OOODBBBBBBSSSSSSSSBBBBBBDOOO..",
            ".OWWWBBBBBBBSSSSSSSSBBBBBBBWWWO.",
            "OWWWWWBBBBBBSSSSSSSSBBBBBBWWWWWO",
            ".OWWWWBBBBBBBBBBBBBBBBBBBBWWWWO.",
            "..OOOWWBBBBBBBBBBBBBBBBBBWWOOO..",
            ".....OOODBBBOOOOOOOOBBBDOOO.....",
            ".......OOOOO........OOOOO.......",
            "................................",
        ],
        // 🐧 Star Pengu — aurora crest, snow white belly, golden beak & cozy scarf
        PetSpecies::StarPengu => [
            "................................",
            ".........OOOOOOOOOOOOOO.........",
            ".......OOBBLLLLLLLLLLBBOO.......",
            ".....OOBBBBBBBGAAGBBBBBBBOO.....",
            "....ODBBBBBWWWWWWWWWWBBBBBDO....",
            "....ODBBBBEEKKWWWWEEKKBBBBDO....",
            "....ODBBBBKKKKWWWWKKKKBBBBDO....",
            "....ODBBBBWWWWGGGGWWWWBBBBDO....",
            "...OODBBBWWWWGGGGGGWWWWBBBDOO...",
            "..ODBBBBBAAAAAAAAAAAAAABBBBBDO..",
            ".ODBBBBBWWWWWWWWWWWWWWWWBBBBBDO.",
            ".ODBBBBWWWWWWWWGGWWWWWWWWBBBBDO.",
            ".ODBBBBWWWWWWWGGGGWWWWWWWBBBBDO.",
            "..OODBBWWWWWWWWWWWWWWWWWWBBDOO..",
            "...OOODWWWWWWWWWWWWWWWWWWDOOO...",
            ".....OOODBBBBBBBBBBBBBBDOOO.....",
            "......OOOGGGGOOOOOOGGGGOOO......",
            "........OOOOO......OOOOO........",
            "................................",
            "................................",
        ],
        // 🐉 Ignis Drake — golden horns, wide bat-like dragon wings, ribbed belly, fire tail
        PetSpecies::IgnisDragon => [
            "....AA....................AA....",
            "...GAAG......OOOOOO......GAAG...",
            "....OAAOOOOOBBLLLLBBOOOOAAOO....",
            ".....OOOBBBBBBBBBBBBBBBBOOO.....",
            ".OOO...OBBBEKKBBBBEEKKBBBO...OOO",
            "OAAAO..OBBBKKKBBBBKKKKBBBO..OAAA",
            "OABBBO.ODBBBBBBAAABBBBBBDO.OBBBA",
            "OABBBBOOOODBBBSSSSBBBDOOOOBBBBBA",
            "OADBBBBBBBBSSSSSSSSSSBBBBBBBBDAO",
            ".OADBBBBBBBSSSSSSSSSSBBBBBBBDAO.",
            "..OOODBBBBBSSSSSSSSSSBBBBBDOOO..",
            "....ODBBBBBSSSSSSSSSSBBBBBDO..AA",
            "....ODBBBBBSSSSSSSSSSBBBBBDO.AGG",
            "....ODBBBBBBSSSSSSSSBBBBBBDOOAA.",
            ".....ODBBBBBBBBBBBBBBBBBBDOOO...",
            ".....OOODBBBOOOOOOOOBBBDOOO.....",
            ".......OOOOO........OOOOO.......",
            "................................",
            "................................",
            "................................",
        ],
        // 🐲 Azure Wyrm — serpentine celestial sea dragon, antler horns, glowing moon pearl
        PetSpecies::AzureWyrm => [
            ".....AA..................AA.....",
            "....OAOO....OOOOOOOO....OOAO....",
            ".....OOAOOOBBLLLLLLBBOOOAOO.....",
            "......OOOBBBBBBBBBBBBBBOOO......",
            ".....ODBBBEEKKBBBBEEKKBBBDO.....",
            ".....ODBBBKKKKBBBBKKKKBBBDO.....",
            "....OOODBBBBBBSSSSBBBBBBDOOO....",
            "...OABBBOOOOBBWWWWBBOOOOBBBAO...",
            "..OABBBBBBBBOWWWWWWOABBBBBBBBAO.",
            ".OABBBBSSSSSOOWWWWOOSSSSSBBBBAO.",
            "..OODBBSSSSSBBBBBBBBSSSSSBBDOO..",
            "....OOOODBBBBBBBBBBBBBBDOOOO....",
            "...OABBBBBSSSSSSSSSSSSBBBBBAO...",
            "..OABBBBDDDDDDDDDDDDDDDDBBBBAO..",
            "...OOOOO................OOOOO...",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        // 🦊 Kitsune Nine — shrine fox ears, vermilion fur, fanned 9-tail silhouette & foxfire
        PetSpecies::EmberFox => [
            "...OOOO..................OOOO...",
            "..OBWWBO................OBWWBO..",
            "..OBWWWBO....OOOOOO....OBWWWBO..",
            "...OBBBBBOOBBLLLLLLBBOOBBBBBO...",
            "....OBBBBBBBBBBBBBBBBBBBBBBO....",
            "...ODBBBEEKKBBBBBBBBEEKKBBBDO...",
            "...ODBBBKKKKBBBBBBBBKKKKBBBDO...",
            "...ODBBBPPPBBBWWWWBBBPPPBBBDO...",
            "....OOODBBBBBBWAAWBBBBBBDOOO....",
            ".AA....OOOOBBBSSSSBBBOOOO....AA.",
            "AWWA.....ODBBSSSSSSBBDO.....AWWA",
            ".AA.....ODBBBSSSSSSBBBDO.....AA.",
            ".......ODBBBBSSSSSSBBBBDOOO.OO..",
            "......ODBBBBBBBBBBBBBBBBDOSSWSO.",
            ".....OOODBBBOOOOOOOOBBBDOOSSSSO.",
            ".......OOOOO........OOOOO..OOO..",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        // 🦅 Sol Phoenix — solar crown plumes, blazing outspread wings, golden tail feathers
        PetSpecies::PhoenixChick => [
            "...........GG..GG..GG...........",
            "..........OGGAAggaAGGO..........",
            "..OOO....OBBLLLLLLLLBBO....OOO..",
            ".OAAAO..OBBBBBBBBBBBBBBO..OAAAO.",
            "OAGGGGOOBBBEEKKWWEEKKBBBOOGGGGAO",
            "OAGBBBBOOBBKKKKGGKKKKBBOOBBBBGAO",
            "OABBBBBBOODBBBBGGGGBBDOOBBBBBBAO",
            ".OADBBBBBBBBBBBBBBBBBBBBBBBBDAO.",
            "..OOODBBBBBBSSSSSSSSBBBBBBDOOO..",
            "....OOOODBBBSSSSSSSSBBBDOOOO....",
            "......ODBBBBSSSSSSSSBBBBDO......",
            "......ODBBBBBBBBBBBBBBBBDO......",
            ".....OOOGGGGOOOOOOOOGGGGOOO.....",
            "....OAGAGAGAO......OAGAGAGAO....",
            ".....OOOOOOO........OOOOOOO.....",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        // 🦄 Astra Unicorn — golden spiral horn, flowing prism mane, celestial hooves
        PetSpecies::CrystalUni => [
            "..............GGGG..............",
            ".............OGGGGO.............",
            "........OO...OGGGGO...OO........",
            ".......OBBOOOOGGGGOOOBBO........",
            "......OBLLLLBBBBBBBBLLLLBO......",
            ".....OBBBBBBBBBBBBBBBBBBBBO.....",
            "....ODBBEEKKBBBBBBBBEEKKBBDO....",
            "....ODBBKKKKBBBBBBBBKKKKBBDO....",
            "....ODBBPPPBBBBBBBBBBPPPBBDO.AA.",
            ".....OOODBBBBBBBBBBBBBBDOOOASSA.",
            "....OOODBBBBSSSSSSSSBBBBDOOASSA.",
            "...OABBBBBBBSSSSSSSSBBBBBBBOAA..",
            "...OABBBBBBBBBBBBBBBBBBBBBBOO...",
            "....OOODBBBDDDDDDDDDDBBBDOOO....",
            ".....OOOGGGGOOOOOOOOGGGGOOO.....",
            ".......OOOOO........OOOOO.......",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        // 👑 Royal Jelly — grand golden crown with gems, glossy translucent slime monarch
        PetSpecies::SlimeKing => [
            "................................",
            ".......GG.....GGGG.....GG.......",
            "......OGGGO..OGAAGGO..OGGGO.....",
            "......OGGGGGGGGGGGGGGGGGGGO.....",
            "......OOOOOOOOOOOOOOOOOOOOO.....",
            "....OOOBLLLLLLLLLLLLLLLLBBOOO...",
            "...OBBBWWWBBBBBBBBBBBBBBBBBBBO..",
            "..OBBBWWWBBBBBBBBBBBBBBBBBBBBBO.",
            ".ODBBWWBBBBBBBBBBBBBBBBBBBBBBBDO",
            ".ODBBBBBEEKKBBBBBBBBEEKKBBBBBBDO",
            ".ODBBBBBKKKKBBBBBBBBKKKKBBBBBBDO",
            ".ODBBBPPPBBBBWWWWWWBBBBPPPBBBBDO",
            ".ODBBBBBBBBSSSSSSSSSSBBBBBBBBBDO",
            ".ODBBBBBBBBBBBBBBBBBBBBBBBBBBBDO",
            "..OODDDDDDDDDDDDDDDDDDDDDDDDDOO.",
            "....OOOOOOOOOOOOOOOOOOOOOOOO....",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
    }
}

/// Unique 32×20 Teen stage bitmaps with sleeker intermediate proportions.
fn teen_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick => [
            "................................",
            ".......AA..............AA.......",
            "......OAAO...OOOOOO...OAAO......",
            ".......OOOBBBLLLLLLBBBOOO.......",
            "....OO..OBBBEEKKBBEEKKBBBO..OO..",
            "...OAAO.OBBBKKKKBBKKKKBBBO.OAAO.",
            "...OABBBBBBBBBAAABBBBBBBBBBAO...",
            "....OADBBBBSSSSSSSSSSBBBBDAO....",
            ".....OOODBBSSSSSSSSSSBBDOOO.....",
            "......ODBBBSSSSSSSSSSBBBDO......",
            "......ODBBBBBBBBBBBBBBBBDO..AA..",
            ".......OOODBBBOOOOBBBDOOOOAA....",
            ".........OOOOO....OOOOO.OO......",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        PetSpecies::AzureWyrm | PetSpecies::CrystalUni => [
            "................................",
            ".........AA..........AA.........",
            "........OAOOOOOOOOOOOOAO........",
            ".......OBBLLLLLLLLLLLLBBO.......",
            "......OBBBEEKKBBBBEEKKBBBO......",
            "......OBBBKKKKBBBBKKKKBBBO......",
            "......OBBBPPPBBBBBBPPPBBBO......",
            ".......OOOBBSSSSSSSSBBOOO.......",
            "......OABBBBSSSSSSSSBBBBAO......",
            ".....OADBBBBSSSSSSSSBBBBDAO.....",
            "......OOODBBBBBBBBBBBBDOOO......",
            ".......OOODBBBOOOOBBBDOOO.......",
            ".........OOOOO....OOOOO.........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        PetSpecies::SlimeKing => [
            "................................",
            "................................",
            "..........GG....GG..............",
            ".........OGGGGGGGGO.............",
            ".........OOOOOOOOOO.............",
            ".......OOBBLLLLLLBBOO...........",
            ".....OOBBWWBBBBBBBBBBBOO........",
            "....ODBBBEEKKBBBBEEKKBBBDO......",
            "....ODBBBKKKKBBBBKKKKBBBDO......",
            "....ODBBPPPBBBWWBBBPPPBBDO......",
            "....ODBBBBBSSSSSSSSBBBBBDO......",
            ".....OODDDDDDDDDDDDDDDDDOO......",
            ".......OOOOOOOOOOOOOOOO.........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        _ => [
            "................................",
            "......OOOO............OOOO......",
            ".....OBLPBOOOOOOOOOOOOBPLBO.....",
            "....OBLLLBBBBBBBBBBBBBBLLLBO....",
            "....OBBBEEKKBBBBBBBBEEKKBBBO....",
            "....OBBBKKKKBBBBBBBBKKKKBBBO....",
            "....OBBBPPPBBBAAAABBBPPPBBBO....",
            ".....OOODBBBBSSSSSSBBBBDOOO.....",
            "....OOODBBBBBSSSSSSBBBBBDOOO....",
            "....ODBBBBBBBSSSSSSBBBBBBBDO....",
            ".....ODBBBBBBBBBBBBBBBBBBDO.....",
            "......OOODBBBOOOOOOBBBDOOO......",
            "........OOOOO......OOOOO........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
    }
}

/// 10 distinct 32×20 Baby stage bitmaps so every species has its own chibi silhouette!
fn baby_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::MochiCat | PetSpecies::EmberFox => [
            "................................",
            "................................",
            ".......OOO............OOO.......",
            "......OBPBOOOOOOOOOOOOBPBO......",
            ".....OBLLLBBBBBBBBBBBBLLLBO.....",
            "....OBBBBBBBBBBBBBBBBBBBBBBO....",
            "....ODBBEEKKBBBBBBBBEEKKBBDO....",
            "....ODBBKKKKBBBBBBBBKKKKBBDO....",
            "....ODBBPPPBBBBAAABBBPPPBBDO....",
            ".....ODBBBBBBSSSSSSBBBBBBDO..OO.",
            "......ODBBBBBSSSSSSBBBBBDO..OBO.",
            "......OOODBBBBBBBBBBBBDOOO.OBA..",
            "........OOODBBBOOOOBBBDOOOOO....",
            "..........OOOOO....OOOOO........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        PetSpecies::CloudBun => [
            "........OOO..........OOO........",
            ".......OBPBO........OBPBO.......",
            ".......OBPBO........OBPBO.......",
            ".......OBBBOOOOOOOOOOBBBO.......",
            "......OBLLLBBBBBBBBBBLLLBO......",
            ".....OBBBEKKBBBBBBBBEEKKBBO.....",
            ".....OBBBKKKBBBBBBBBKKKKBBO.....",
            ".....ODBBPPPBBBBBBBBPPPBBDO.....",
            "....OWWDBBBBBSSSSSSBBBBBDBWWO...",
            "....OWWWBBBBBBBBBBBBBBBBWWWO....",
            ".....OOODBBBOOOOOOOOBBBDOOO.....",
            ".......OOOOO........OOOOO.......",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick => [
            "................................",
            "........AA............AA........",
            ".......OAAOOOOOOOOOOOOAAO.......",
            "......OBLLLBBBBBBBBBBLLLBO......",
            "....OOOBBBEKKBBBBBBEEKKBBBOOO...",
            "...OAAOBBBKKKBBBBBBKKKKBBBOAAO..",
            "...OAAODBBPPPBBBBBBPPPBBDOAAO...",
            "....OO.ODBBBBSSSSSSBBBBDO.OO....",
            ".......ODBBBBSSSSSSBBBBDO..AA...",
            "........OODBBBBBBBBBBDOO..OAG...",
            ".........OODBBBOOOOBBDOOOOO.....",
            "..........OOOOO....OOOOO........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        PetSpecies::SlimeKing => [
            "................................",
            "................................",
            "..............GGGG..............",
            ".............OGGGGO.............",
            "...........OOOBLLBBOOO..........",
            ".........OOBBWWBBBBBBBOO........",
            "........OBBBEEKKBBEEKKBBBO......",
            "........OBBBKKKKBBKKKKBBBO......",
            "........ODBBPPPBBBBPPPBBDO......",
            "........ODBBBBSSSSSSBBBBDO......",
            ".........OODDDDDDDDDDDDOO.......",
            "...........OOOOOOOOOOOO.........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
        _ => [
            "................................",
            "................................",
            ".......OOOO..........OOOO.......",
            "......OSSSSOOOOOOOOOOSSSSO......",
            ".....OBLLLBBBBBBBBBBBBLLLBO.....",
            "....OBBBBBBBBBBBBBBBBBBBBBBO....",
            "....ODBBEEKKBBBBBBBBEEKKBBDO....",
            "....ODBBKKKKBBBBBBBBKKKKBBDO....",
            "....ODBBPPPBBBBGGBBBBPPPBBDO....",
            ".....ODBBBBBBSSSSSSBBBBBBDO.....",
            "......ODBBBBBSSSSSSBBBBBDO......",
            "......OOODBBBBBBBBBBBBDOOO......",
            "........OOODBBBOOOOBBBDOOO......",
            "..........OOOOO....OOOOO........",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
            "................................",
        ],
    }
}

/// Species-customized 32×20 Egg stage bitmap with rocking animation.
fn egg_bitmap(species: PetSpecies, frame_idx: usize) -> [&'static str; CANVAS_H] {
    let mut base = if frame_idx.is_multiple_of(2) {
        [
            "................................",
            "...........OOOOOOOOOO...........",
            ".........OOBBLLLLLLBBOO.........",
            ".......OOBBBWWWBBBBBBBBOO.......",
            "......OBBBBBWWWBBBBAAAABBO......",
            ".....OBBBBBBBBBBBBBAAAABBBO.....",
            ".....OBBBBBAAAABBBBBBBBBBBO.....",
            "....ODBBBBBAAAABBBBBSSSSBBDO....",
            "....ODBBBBBBBBBBBBBBSSSSBBDO....",
            "....ODBBBBSSSSBBBBBBBBBBBBDO....",
            "....ODBBBBSSSSBBBBBBAAAABBDO....",
            ".....ODBBBBBBBBBBBBBAAAABDO.....",
            ".....OODBBBBBBBBBBBBBBBBDOO.....",
            "......OOODDDDDDDDDDDDDDOOO......",
            ".....OOSSSSSSSSSSSSSSSSSSOO.....",
            "....OAAAAAAAAAAAAAAAAAAAAAAO....",
            ".....OOOOOOOOOOOOOOOOOOOOOO.....",
            "................................",
            "................................",
            "................................",
        ]
    } else {
        [
            "................................",
            "............OOOOOOOOOO..........",
            "..........OOBBLLLLLLBBOO........",
            "........OOBBBWWWBBBBBBBBOO......",
            ".......OBBBBBWWWBBBBAAAABBO.....",
            "......OBBBBBBBBBBBBBAAAABBBO....",
            "......OBBBBBAAAABBBBBBBBBBBO....",
            ".....ODBBBBBAAAABBBBBSSSSBBDO...",
            ".....ODBBBBBBBBBBBBBBSSSSBBDO...",
            ".....ODBBBBSSSSBBBBBBBBBBBBDO...",
            ".....ODBBBBSSSSBBBBBBAAAABBDO...",
            "......ODBBBBBBBBBBBBBAAAABDO....",
            ".....OODBBBBBBBBBBBBBBBBDOO.....",
            "......OOODDDDDDDDDDDDDDOOO......",
            ".....OOSSSSSSSSSSSSSSSSSSOO.....",
            "....OAAAAAAAAAAAAAAAAAAAAAAO....",
            ".....OOOOOOOOOOOOOOOOOOOOOO.....",
            "................................",
            "................................",
            "................................",
        ]
    };

    if matches!(
        species,
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::SlimeKing
    ) {
        base[0] = "............GG....GG............";
    } else if matches!(
        species,
        PetSpecies::MochiCat | PetSpecies::CloudBun | PetSpecies::EmberFox
    ) {
        base[0] = "..........OO........OO..........";
    }
    base
}

/// Build the 32×20 character grid for any `(species, stage, mood, frame_idx)`.
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
        let crown = ["..G..G..G..", "..GGGGGGG..", "..OOOOOOO.."];
        for (cy, crow) in crown.iter().enumerate() {
            for (cx, cch) in crow.chars().enumerate() {
                if cch != '.' && cx + 10 < CANVAS_W && cy < CANVAS_H {
                    grid[cy][cx + 10] = cch;
                }
            }
        }
        grid[1][3] = 'G';
        grid[2][28] = 'G';
        grid[14][2] = 'W';
        grid[13][29] = 'W';
    }

    // Blink / sleeping eye transform: ONLY turn `E` and `K` eye pixels into closed eyelids `O`!
    // White body/belly/cloud pixels (`W`) remain 100% untouched!
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
                if bch != '.' && bx + 26 < CANVAS_W && by < CANVAS_H {
                    grid[by][bx + 26] = bch;
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

fn scale_rgb(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            ((r as f32 * factor).round() as u16).min(255) as u8,
            ((g as f32 * factor).round() as u16).min(255) as u8,
            ((b as f32 * factor).round() as u16).min(255) as u8,
        ),
        other => other,
    }
}

fn pixel_color(code: char, species: PetSpecies) -> Option<Color> {
    let (primary, secondary, accent) = species_palette(species);
    match code {
        '.' => None,
        'O' | '0' => Some(Color::Rgb(34, 28, 54)),
        'D' => Some(scale_rgb(primary, 0.72)),
        'B' | 'b' => Some(primary),
        'L' => Some(scale_rgb(primary, 1.18)),
        'S' | 's' => Some(secondary),
        'A' | 'a' => Some(accent),
        'W' | 'E' => Some(Color::Rgb(250, 252, 255)),
        'K' => Some(Color::Rgb(20, 16, 32)),
        'P' => Some(Color::Rgb(255, 110, 165)),
        'G' | 'g' => Some(Color::Rgb(255, 220, 75)),
        _ => Some(primary),
    }
}

/// Convert a 32×20 pixel grid into 10 half-block terminal [`Line`]s.
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

/// Convert a 32×20 pixel grid into 10 monochrome half-block String lines.
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
        PetMood::Idle => &["·   ✦   ·   ★   ·   ✦   ·", "✦   ·   ★   ·   ✦   ·   ★"],
        PetMood::Eating => &["♪   ♥   ★   ♥   ♪   ★   ♪", "★   ♪   ♥   ♪   ★   ♥   ★"],
        PetMood::Sleeping => &["☾   z   Z   zZ  ·   ✦   ☾", "✦   Z   zZ  z   ☾   ·   ✦"],
        PetMood::Happy => &["✦   ♥   ★   ♥   ✦   ♥   ✦", "♥   ✦   ♥   ✦   ★   ✦   ♥"],
        PetMood::Sick => &[".   ~   x   ~   .   x   .", "~   .   x   .   ~   .   x"],
        PetMood::Training => &["★   ♦   ▲   ♦   ★   ▲   ★", "♦   ★   ▲   ★   ♦   ★   ♦"],
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

/// TrueColor 32×20 Half-Block Pixel Art [`Line`]s for the habitat LCD screen.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 6 == 5);
    let grid = build_grid(pet.species, pet.stage, pet.mood, frame_idx);
    let (_, secondary, accent) = species_palette(pet.species);

    let mut out = Vec::with_capacity(13);
    out.push(Line::from(Span::styled(
        get_mood_particles(pet.mood, tick).to_string(),
        Style::default().fg(accent),
    )));
    out.extend(render_grid_colored(&grid, pet.species));
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
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

/// TrueColor 32×20 Half-Block Pixel Art [`Line`]s for the Step 1 Companion Selector.
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
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        Style::default().fg(secondary),
    )));
    out
}

/// A friendly preview of a species (adult form, idle) as plain strings.
pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// What your companion is saying out loud right now (`💬`).
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
        format!("💬 \"{} is happy to see you! (Press [Y] to Talk!)\"", pet.name),
        format!("💬 \"Craving {}...\"", pet.species.favorite_snack()),
        "💬 \"Let's play the 3-in-1 Retro Arcade [G]!\"".to_string(),
        "💬 \"Headpats [T] & chats [Y] grow our bond!\"".to_string(),
    ];
    chatter[(tick / 3) % chatter.len()].clone()
}

/// Inner thought bubble (`💭`) revealing what your companion is secretly thinking!
pub fn thought_bubble(pet: &Pet, tick: usize) -> String {
    if pet.stage == EvolutionStage::Egg {
        return "💭 (Thinking: It's so cozy in here... I want to meet my keeper soon!)"
            .to_string();
    }
    if pet.has_toothache {
        return "💭 (Thinking: Note to self... 3 sweet snacks in a row was a mistake!)"
            .to_string();
    }
    if pet.is_sleeping {
        return format!(
            "💭 (Dreaming of flying over a mountain of {}...)",
            pet.species.favorite_snack()
        );
    }
    let species_thoughts = match pet.species {
        PetSpecies::MochiCat => [
            "💭 (Thinking: If I purr louder, maybe I'll get extra Taiyaki...)",
            "💭 (Thinking: That sunny spot on the floor looks perfect for a nap.)",
            "💭 (Thinking: My keeper's keyboard clicks sound like a lullaby!)",
        ],
        PetSpecies::BobaPup => [
            "💭 (Thinking: Is it playtime yet?! My tail won't stop wagging!)",
            "💭 (Thinking: I wonder where I buried my favorite honey biscuit...)",
            "💭 (Thinking: I'll guard the Sanctuary while my keeper works!)",
        ],
        PetSpecies::CloudBun => [
            "💭 (Thinking: My ears feel extra fluffy today~)",
            "💭 (Thinking: Floating on a daydream... carrot clouds everywhere!)",
            "💭 (Thinking: Press [Y] to chat with me anytime!)",
        ],
        PetSpecies::StarPengu => [
            "💭 (Thinking: I counted 42 shooting stars in the aurora last night!)",
            "💭 (Thinking: Waddling is the most dignified way to travel.)",
            "💭 (Thinking: Snowberries taste best when shared with a friend!)",
        ],
        PetSpecies::IgnisDragon => [
            "💭 (Thinking: My flame breath is getting warmer! Soon I'll soar!)",
            "💭 (Thinking: Every true dragon hoards shiny coins and spicy chilies!)",
            "💭 (Thinking: I love when my keeper polishes my golden horns.)",
        ],
        PetSpecies::AzureWyrm => [
            "💭 (Thinking: The tides and stars are in harmony today...)",
            "💭 (Thinking: My moon pearl glows brighter whenever you visit!)",
            "💭 (Thinking: Ancient dragons value patience, wisdom, and snacks.)",
        ],
        PetSpecies::EmberFox => [
            "💭 (Thinking: One day I'll grow all nine tails of cyan foxfire!)",
            "💭 (Thinking: Inari tofu is the secret to spiritual power~)",
            "💭 (Thinking: Should I play a tiny harmless prank, or behave?)",
        ],
        PetSpecies::PhoenixChick => [
            "💭 (Thinking: My feathers feel warm like sunrise!)",
            "💭 (Thinking: Even a legendary solar firebird loves headpats.)",
            "💭 (Thinking: Let's set a new high score in Hurdle Dash [G]!)",
        ],
        PetSpecies::CrystalUni => [
            "💭 (Thinking: My prism horn caught a rainbow beam just now!)",
            "💭 (Thinking: Starlight sugar makes my hooves sparkle~)",
            "💭 (Thinking: Our bond feels stronger every single day!)",
        ],
        PetSpecies::SlimeKing => [
            "💭 (Thinking: *boing* *wobble* Being a royal monarch is hard work!)",
            "💭 (Thinking: Make sure my golden crown isn't tilted!)",
            "💭 (Thinking: Crystal dew keeps my gel 100% bouncy!)",
        ],
    };
    species_thoughts[(tick / 4) % species_thoughts.len()].to_string()
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
    match (tick / 90) % 4 {
        0 => ("🌞 DAY", "Sunny"),
        1 => ("🌸 BREEZE", "Petal Wind"),
        2 => ("🌧️ RAIN", "Cozy Drops"),
        _ => ("🌠 NIGHT", "Starfall"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_species_stages_and_moods_render_32x20_half_block_pixel_sprites() {
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
        let g0 = build_grid(PetSpecies::StarPengu, EvolutionStage::Adult, PetMood::Idle, 0);
        let g1 = build_grid(PetSpecies::StarPengu, EvolutionStage::Adult, PetMood::Idle, 1);
        let w_count_0 = g0.iter().flatten().filter(|&&c| c == 'W').count();
        let w_count_1 = g1.iter().flatten().filter(|&&c| c == 'W').count();
        assert_eq!(
            w_count_0, w_count_1,
            "white body pixels 'W' must never turn into eyelid 'O'"
        );
    }

    #[test]
    fn test_render_animated_colored_sprite_and_thought_bubble() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let colored = render_colored_sprite(&pet, 0);
        assert!(!colored.is_empty());
        assert!(thought_bubble(&pet, 0).contains("Thinking"));

        pet.stage = EvolutionStage::Baby;
        pet.poop_count = 2;
        let dirty_lines = render_animated_sprite(&pet, 0);
        assert!(dirty_lines.iter().any(|l| l.contains('💩')));
        assert!(speech_bubble(&pet, 0).contains("bubble bath"));
        assert!(thought_bubble(&pet, 0).contains("Thinking"));
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
