//! Professional 16-Bit 3/4 Side-Profile TrueColor Pixel Art Engine (`▀`, `▄`, `█`)
//! for **Mochigotchi v1.0.0**.
//!
//! Every companion is hand-crafted as a **40×22 pixel bitmap** (40 columns × 11
//! terminal rows of `1:1` square half-block pixels) using authentic **3/4
//! side-profile creature anatomy** (distinct snout/muzzle/beak facing left,
//! arched neck, 4 equine/feline/canine/draconic limbs, sweeping wings, and
//! multi-tone shading) that evolves across 5 distinct silhouettes:
//! 1. `🥚 Egg`: Elemental patterned egg on a warm sanctuary cushion.
//! 2. `🍼 Baby`: Cute big-headed 3/4 chibi hatchling.
//! 3. `🌱 Teen`: Sleek, agile juvenile striker with budding wings/horns/tails.
//! 4. `🌸 Adult`: Fierce full-grown guardian beast.
//! 5. `🌟 CyberElder`: Massive Apex Sovereign boss form with golden armor & aura.
//! 6. `👻 PassedOut`: Floating spirit & pixel memorial when a pet faints/dies.

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub const CANVAS_W: usize = 40;
pub const CANVAS_H: usize = 22;

/// 3/4 Side-Profile **APEX ELDER (`CyberElder`)** bitmaps (40×22).
/// Designed with distinct heads/snouts on the left, wings/manes sweeping back,
/// 4 distinct legs/claws on the ground, and sweeping tails on the right!
fn elder_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        // 🦄 09/10 Astra Unicorn (Elder — Celestial Kirin Pegasus)
        // 3/4 Equine side profile: long diagonal golden spiral horn on top-left,
        // horse head & snout facing left, arched stallion neck, sweeping feathered
        // Pegasus wings rising from the back, 4 distinct equine legs with golden hooves,
        // and a flowing starlight tail on the right!
        PetSpecies::CrystalUni => [
            "...GG...................................",
            "....OGG......OO..........OOOOOO.........",
            ".....OGGO...OBBO.......OOWWWWWWWOO......",
            "......OGGO.OBLLBO....OOWWWWWWWWWWWOO....",
            "....OOOGGGOOBBBBBO..OWWWWSSSSWWWWWWWO...",
            "...OBLLLLBBBBBBBBBOOWWWSSSSSSWWWWWWWO...",
            "..OBBBEEKKBBBBBBBBBOWWSSSSSSWWWWWWWWO...",
            ".ODBBBOOOOBBBBAAABBBWWSSSSWWWWWWWWWWO...",
            ".OSSSSBBBBBBBBAAAAABWWSSWWWWWWWWWOO.....",
            "..OOOOOOODBBBBBBAAAABWWWWWWWWOO.........",
            "........ODBBGGGGBBBBBWWWWOO.............",
            ".......ODBBGGGGGSSSSSSSSBBBBBOOO..OOOO..",
            "......ODBBBGGGGSSSSSSSSBBBBBBBBBBOAAAASO",
            ".....ODBBBBSSSSSSSSSSSSSSSSBBBBBBBAAASSO",
            ".....ODBBBBBBBBBBBBBBBBBBBBBBBBBBDOAAASO",
            ".....ODBBBDDDDDDDDDDDDDDDDDDDDBBBDO.OAAO",
            "....ODBBDO..ODBBDO......ODBBDO.ODBBDO...",
            "...ODBBDO...ODBBDO......ODBBDO..ODBBDO..",
            "..OOGGGO....OOGGGO......OOGGGO...OOGGGO.",
            "..OOOOOO....OOOOOO......OOOOOO...OOOOOO.",
            "........................................",
            "........................................",
        ],
        // 🐉 05/10 Ignis Drake (Elder — Inferno Bahamut Sovereign)
        // 3/4 Wyvern Dragon profile: horned dragon jaw facing left breathing fire,
        // arched armored neck, massive bat-like magma wings rising on the right,
        // muscular clawed arms & hind legs, and blazing flame-tipped tail!
        PetSpecies::IgnisDragon => [
            "...........GGGG.......OOOOOOO...........",
            ".........OOGGGGO....OOAAAAAAAO..........",
            "........OGGGGOO...OOAAAGGGGGGAAO........",
            "..RR...OOOOOO....OAAAGGGGGGGGGGAO.......",
            ".RGGR.OBLLLLBBO.OAAGGGDDDDGGGGGAO.......",
            "..OO.OBBBRKBBBBOOAAGGDDDDDDGGGGAO.......",
            "....ODBBBOOBBBBBBOAGDDDDDDDDGGGAO.......",
            "...ODBBBBWWWBBBBBOAGDDDDDDDDGGGAO.......",
            "...OOOOOOSSSSBBBBOAGDDDDDDGGGGGAO.......",
            "........ODSSSSBBBOAGGGDDGGGGGGAAO.......",
            "....OO..ODGGGGSSSSBBBBGGGGGGGAAO........",
            "...OWWO.ODGGGGSSSSSSSSBBBBBBAAO....RR...",
            "...OOOOODBBGGGSSSSSSSSBBBBBBBDO...RGGR..",
            ".......ODBBBBSSSSSSSSSSBBBBBBBDO..RGGGR.",
            ".......ODBBBBBBBBBBBBBBBBBBBBBDO...OAAO.",
            ".......ODBBBBDDDDDDDDDDDDDBBBBDO..OBBDO.",
            "......ODBBBDO.ODBBBDO...ODBBBBDOOOOBDO..",
            ".....ODBBBDO..ODBBBDO....ODBBBBBBBBDO...",
            "....OOWWWOO...OOWWWOO....OOWWWOOOOOO....",
            "....OOOOOOO...OOOOOOO....OOOOOOO........",
            "........................................",
            "........................................",
        ],
        // 🐲 06/10 Azure Wyrm (Elder — Tidal Leviathan Sovereign)
        // 3/4 Eastern Sea-Dragon Leviathan: antlered dragon head on top-left,
        // long whiskers, glowing Moon Pearl held in front claws, and a long
        // S-curved serpentine body with tidal dorsal fins across all 40 cols!
        PetSpecies::AzureWyrm => [
            ".........GGGG...........................",
            ".......OOGGGGO.........AA.......AA......",
            "......OGGGOO..........OAAO.....OAAO.....",
            "..AA.OOOOOOO.........OAAAAOOOOOAAAAO....",
            ".OAAOBLLLLLLBBO.....OAAAGGGGGGGGGAAO....",
            "....OBBBRKBBBBBO...OAAAGBBBBBBBBBBAAO...",
            "...ODBBBOOBBBBBBO.OAAABBBBBSSSSBBBBAAO..",
            "...ODBBBBWWWBBBBBOOAABBBBSSSSSSSSBBBBAO.",
            "....OOOOOSSSSBBBBBOABBBSSSSOOOOSSSSBBBAO",
            "...OAAWWAAOSSBBBBBOBBBSSSOO....OOSSSBBAO",
            "..OAWWWWWWAOOBBBBBBBBSSSO........OSSBBAO",
            "..OAWWWWWWAOODBBBBBBBSSO..........OSBBAO",
            "...OAAWWAAO.ODBBBBSSSSO...........OSBBAO",
            "....OOOOOO..ODBBBBSSSSO...........OSBBAO",
            "...........ODBBBBBSSSSO..........OOSBBAO",
            "..........OOWWBBBBSSSSOOOOOOOOOOOSSBBDAO",
            "..........OOOOODBBBSSSSSSSSSSSSSSSSBBDAO",
            "..............ODBBBBBBBBBBBBBBBBBBBBDAO.",
            "...............OOODDDDDDDDDDDDDDDDDDOO..",
            ".................OOOOOOOOOOOOOOOOOOO....",
            "........................................",
            "........................................",
        ],
        // 🦊 07/10 Kitsune Nine (Elder — Nine-Tailed Kyuubi Spirit)
        // 3/4 Fox Spirit profile: pointed fox snout & shrine ears on the left,
        // sleek white-maned chest, 4 fox legs, and a colossal fan of 9 tails
        // with cyan foxfire orbs filling the right half of the canvas!
        PetSpecies::EmberFox => [
            ".....OOO...........AA......AA......AA...",
            "....OBWBO.........AWWA....AWWA....AWWA..",
            "...OBWWWBO..OOO....AA.OOOO.AA.OOOO.AA...",
            "...OBBBBBBOOBWBO....OSSSSSSOSSSSSSO.....",
            "..OBLLLLBBBBWWWBO..OSSSSSSSOSSSSSSSO....",
            ".OBBBRKBBBBBBBBBO.OSSSSBBBSSOSSSBBSSO...",
            "ODBBBOOBBBRRBBBBBOSSSSBBBBBSSOSSBBBSSO..",
            "ODBBBBWWWBBBBBBBBOSSSBBBBBBBSSOSBBBBSSO.",
            ".OOOOOWWWWWWWBBBBBOSSBBBBBBBBSSSBBBBBSSO",
            ".....OWWWWWWWWBBBBOSBBBBBBBBBBSSBBBBBSSO",
            "....OWWWWWWWWWBBBBBBBBBBBBBBBBSSBBBBBSSO",
            "....OWWWGGGGSSSSSSSSSSSSSSSSBBSSBBBBBSSO",
            ".....OOOGGGGSSSSSSSSSSSSSSSSBBBBBBBBBSSO",
            ".......ODBBBBSSSSSSSSSSSSSSSBBBBBBBBSSO.",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBBBBBSSO..",
            ".......ODBBBDDDDDDDDDDDDDDDDDDDDBBSSO...",
            "......ODBBDO..ODBBDO......ODBBDO.OOOO...",
            ".....ODBBDO...ODBBDO......ODBBDO........",
            "....OOWWOO....OOWWOO......OOWWOO........",
            "....OOOOOO....OOOOOO......OOOOOO........",
            "........................................",
            "........................................",
        ],
        // 🦅 08/10 Sol Phoenix (Elder — Solar Garuda Deity)
        // 3/4 Firebird profile: hooked golden raptor beak & 5-plume solar crest on left,
        // armored golden chest, fierce talons, and giant sweeping flame wings & tail!
        PetSpecies::PhoenixChick => [
            ".......GG..GG..GG......OOOOOOO..........",
            "......OGGAAGGAAGGO...OOAAAAAAAO.........",
            ".....OOBLLLLLLLLBO..OAAAGGGGGGAAO.......",
            "..GGOBBBBBBBBBBBBBOOAAGGGGBBBGGGAO......",
            ".OGGGGBBRKBBBBBBBBBOAAGGBBBBBBBGGAO.....",
            "OGGGGOOOBBBBBBBBBBBOAGBBBBBBBBBBGAO.....",
            ".OOOO..ODBBBBGGGGBBOAGBBBBBBBBBBGAO.....",
            "......ODBBBGGGGGGGGBAGBBBBBBBBGGAO......",
            ".....ODBBBGGGGGGGGGGBBBBBBBBBGGAO.......",
            ".....ODBBBGGGGSSSSSSSBBBBBBGGAAO........",
            ".....ODBBBBSSSSSSSSSSSSBBBBAAO...OOOOO..",
            "......ODBBBBSSSSSSSSSSSBBBBDO..OOAAAAAO.",
            ".......ODBBBBBBBBBBBBBBBBBBDOOOAGGGGGGGA",
            "........OOODBBBBDDDDDDDDBBBBAAAGGGGGGGGA",
            "..........ODBBBDO.....ODBBBDOOOOOOOOOOO.",
            ".........OOGGGGO......OOGGGGO...........",
            "........OAGAGAGAO....OAGAGAGAO..........",
            ".........OOOOOOO......OOOOOOO...........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 👑 10/10 Royal Jelly (Elder — Titan Slime Emperor)
        // Hulking Slime Monarch with tilted jeweled crown, royal cape, scepter & gel gauntlets
        PetSpecies::SlimeKing => [
            "...........GG......GG......GG...........",
            "..........OGGO...OGRRGO...OGGO..........",
            ".........OGGGGOOOGGRRGGOOOGGGGO.........",
            ".........OGGGGGGGGGGGGGGGGGGGGO.........",
            ".........OOOOOOOOOOOOOOOOOOOOOO...GG....",
            "......OOOBLLLLLLLLLLLLLLLLLLLLBBOOGGO...",
            "....OOBBBWWWWBBBBBBBBBBBBBBBBBBBBOGGO...",
            "..OO..OBBWWBBBBBBBBBBBBBBBBBBBBBBBOOO...",
            ".OGGO.ODBBBBBREKBBBBBBBBKERBBBBBDO.OO...",
            "OGBBGOODBBBBBROOBBBBBBBBOORBBBBBDOOBBO..",
            "OBBBBGOODBBBBBBWWOOOOOOWWBBBBBBDOOBBBBO.",
            "OBBBBBBOODBBBBSSSSSSSSSSSSBBBBDOOBBBBBBO",
            ".OODBBBBOODBBGGGSSSSSSSSGGGBBDOOBBBBDOO.",
            "...OOOOO.ODBBBBSSSSSSSSSSBBBBDO.OOOOO...",
            ".........ODBBBBBBBBBBBBBBBBBBDO.........",
            "........ODDDDDDDDDDDDDDDDDDDDDDO........",
            ".......OOOOOOOOOOOOOOOOOOOOOOOOOO.......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐱 01/10 Neko Mochi (Elder — Byakko Saber Tiger Shogun)
        // 3/4 Armored Saber-Tiger profile: tiger head on left with white saber fangs,
        // golden shogun shoulder-armor, muscular 4-legged tiger torso & twin tails!
        PetSpecies::MochiCat => [
            ".....OOO.....OOO........................",
            "....OBPBO...OBPBO.......................",
            "...OBPPPBOoobPPPBO......................",
            "..OBLLLLLLLLLLLLBBO....GGGGGG...........",
            ".OBBBBBBBBBBBBBBBBBO..OGGGGGGO....OOOO..",
            ".OBBBRKBBBBBRKBBBBBO.OGGGGGGGGO..OBBBBO.",
            "ODBBBOOBBPPBOOBBBBBBOOGGGBBGGGGOOBBBBBAO",
            "ODBBBBWWWWWWWWBBBBBBOGGBBBBBGGGOBBBBBAO.",
            ".OOWWWOOOOOOOOWWWBBBOOBBBBBBBBBOBBBBAO..",
            "..OOOO..ODGGGGSSSSSSSSSSSSSSSSBBBBBAO...",
            "........ODGGGGSSSSSSSSSSSSSSSSBBBBAO.OO.",
            ".......ODBBBBSSSSSSSSSSSSSSSSSBBBBDOOBBO",
            "......ODBBBBBBSSSSSSSSSSSSSSSSBBBBDOBBAO",
            "......ODBBBBBBBBBBBBBBBBBBBBBBBBBBDOBAO.",
            "......ODBBBBDDDDDDDDDDDDDDDDDDBBBBDOAO..",
            ".....ODBBBDO..ODBBBDO....ODBBBDOODBBDO..",
            "....ODBBBDO...ODBBBDO....ODBBBDO.ODBBDO.",
            "...OOWWWOO....OOWWWOO....OOWWWOO.OOWWWOO",
            "...OOOOOOO....OOOOOOO....OOOOOOO.OOOOOOO",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐶 02/10 Boba Pup (Elder — Fenrir Dire Wolf Guardian)
        // 3/4 Dire Wolf profile: wolf snout & fangs on left, spiked golden mane,
        // muscular 4-legged wolf body & bushy tail on the right!
        PetSpecies::BobaPup => [
            "......OOO.....OOO.......................",
            ".....OSSSO...OSSSO....GG..GG..GG........",
            "....OSSSSSOOOSSSSSO..OGGGGGGGGGGO.......",
            "..OOBLLLLLLLLLLLLBBOOGGGGGGGGGGGGO......",
            ".OSSSSBBBBBBBBBBBBBOGGGGGBBBBBGGGGO.....",
            "OSSSSBBRKBBBBBBBBBBGGGGGBBBBBBBGGGOOOOO.",
            "ODBBBBBOOBBBBBBBBBBGGGSSSSSSSSBBBBSSSSSO",
            ".OOOOOWWWWWBBBBBBBBGGSSSSSSSSSSBBSSSSSSO",
            "......OOOOOSSSSBBBBGSSSSSSSSSSSSBSSSSSO.",
            "..........ODSSSSBBBBSSSSSSSSSSSSBSSSSO..",
            ".........ODBBBBBBSSOSOSOSOSOSOSSBBBDO...",
            "........ODBBBBBBBSSSSSSSSSSSSSSSBBBDO...",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBBBBBDO...",
            ".......ODBBBBDDDDDDDDDDDDDDDDDDBBBBDO...",
            "......ODBBBDO..ODBBBDO....ODBBBDOODBBDO.",
            ".....ODBBBDO...ODBBBDO....ODBBBDO.ODBBDO",
            "....OOWWWOO....OOWWWOO....OOWWWOO.OOWWWO",
            "....OOOOOOO....OOOOOOO....OOOOOOO.OOOOOO",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐰 03/10 Cloud Bun (Elder — Raijin Storm Valkyrie)
        // Long swept-back valkyrie wing-ears, storm cloud mantle & agile stance
        PetSpecies::CloudBun => [
            ".......OOOOOO....OOOOOO.................",
            ".....OOBLLLLBBOOBLLLLBBOO...............",
            "....OBBPPPPPPBBBBPPPPPPBBO..............",
            "...OBBPPPPPPPBBBBPPPPPPPBBO.............",
            "....OOBBBBBBBBBBBBBBBBBBBOO...WWWWWW....",
            ".....OBLLLLLLLLLLLLLLLLBBO..OWWWWWWWWO..",
            "....OBBBRKBBBBBBBBRKBBBBBO.OWWWWGGWWWWO.",
            "...ODBBBOOBBPPPPBBOOBBBBBBOWWWWGGGGWWWWO",
            "...ODBBBBBBBBWWBBBBBBBBBBBOWWWWGGGGWWWWO",
            "..OWWDBBBBGGGSSGGGBBBBBBBBOWWWWWGGWWWWO.",
            ".OWWWWDBBBSSSSSSSSBBBBBBBBOWWWWWWWWWWWO.",
            ".OWWWWODBBBSSSSSSSSBBBBBBBOOWWWWWWWWOO..",
            "..OOO.ODBBBBSSSSSSSSBBBBBBDO.OOOOOOOO...",
            "......ODBBBBBBBBBBBBBBBBBBDO............",
            "......ODBBBBDDDDDDDDDDBBBBDO............",
            ".....ODBBBDO..........ODBBBDO...........",
            "....OOWWWOO............OOWWWOO..........",
            "....OOOOOOO............OOOOOOO..........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐧 04/10 Star Pengu (Elder — Empoleon Frost Kaiser)
        // Golden trident crown, ice-emperor cape wings & armored snow breastplate
        PetSpecies::StarPengu => [
            "............GG.....GG.....GG............",
            "...........OGGO...OGGO...OGGO...........",
            "...........OGGGOOOGGGGOOOGGGO...........",
            "............OGGGGGGGGGGGGGGO............",
            "..........OOOBBLLLLLLLLLLBBOOO..........",
            "...AA...OOBBBBBBBBGGGGbbbbbbbBOO...AA...",
            "..OAAO.ODBBBBREKWWGGGGWWKERBBBBDO.OAAO..",
            ".OAAAAOODBBBBROOWWGGGGWWOORBBBBDOOAAAAO.",
            "OAAWWAAODBBBBBWWWWGGGGGGWWWWBBBBBDOAAWWAO",
            "OAWWWWAAODBBBBAAAAAAAAAAAAAABBBBDOAAWWWAO",
            "OAWWBBAAODBBBWWWWWWGGGGWWWWWWBBBDOAABBWAO",
            ".OABBBAO.ODBBWWWWWGGGGGGWWWWWBBDO.OABBAO.",
            "..OABBO..ODBBWWWWWWGGGGWWWWWWBBDO..OBBAO.",
            "...OOO...ODBBBWWWWWWWWWWWWWWBBBDO...OOO..",
            ".........ODBBBBBBBBBBBBBBBBBBBBDO........",
            "........OOOGGGGGGOOOOOOOOGGGGGGOOO.......",
            ".......OGGGGGGGGGO......OGGGGGGGGGO......",
            "........OOOOOOOOO........OOOOOOOOO.......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
    }
}

/// 3/4 Side-Profile **ADULT (`Adult`)** bitmaps (40×22).
fn adult_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::CrystalUni => [
            "........................................",
            ".....GG.................................",
            "......OGG.....OO........OOOOO...........",
            ".......OGGO..OBBO.....OOWWWWWOO.........",
            ".....OOOGGGOOBLLBO...OWWWSSSWWWO........",
            "....OBLLLLBBBBBBBBO.OWWWSSSSWWWO........",
            "...OBBBEEKKBBBBBBBBOOWWSSSSWWWO.........",
            "..ODBBBPPPBBBBAAABBBOOWWWWWWWOO.........",
            "..OSSSSBBBBBBBBAAABBBOOOOOOOO...........",
            "...OOOOOOODBBBBBBBBBBBO.................",
            ".........ODBBBBSSSSSSSSBBBBBOOO..OOO....",
            "........ODBBBBBSSSSSSSSBBBBBBBBBOAAAO...",
            ".......ODBBBBBBSSSSSSSSBBBBBBBBBBAAASO..",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBBDOAASO..",
            ".......ODBBBDDDDDDDDDDDDDDDDDBBBDO.OO...",
            "......ODBBDO..ODBBDO....ODBBDOODBBDO....",
            ".....OOGGGO...OOGGGO....OOGGGO.OOGGGO...",
            ".....OOOOOO...OOOOOO....OOOOOO.OOOOOO...",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::IgnisDragon => [
            "........................................",
            "...........GGG........OOOOOO............",
            ".........OOGGGO.....OOAAAAAAO...........",
            "........OOOOOO.....OAAAGGGGGAO..........",
            "......OBLLLLLBBO..OAAGGDDDDGGAO.........",
            ".....OBBBEEKKBBBO.OAGDDDDDDGGAO.........",
            "....ODBBBKKKKBBBBOOAGDDDDDDGGAO.........",
            "....OSSSSBBBBBBBBBOAGGDDDDGGAAO.........",
            ".....OOOOOOSSSSBBBBOAGGGGGGGAAO.........",
            "..........ODSSSSBBBBBBBBGGGGAAO.........",
            "......OO..ODBBBBSSSSSSSSBBBBAAO.........",
            ".....OWWO.ODBBBBSSSSSSSSBBBBBDO....AA...",
            ".....OOOOODBBBBBBSSSSSSSSBBBBDO...AGGA..",
            ".........ODBBBBBBBBBBBBBBBBBBDO..OAGGA..",
            ".........ODBBBBDDDDDDDDDDBBBBDO.OOBBA...",
            "........ODBBBDO..ODBBBDO.ODBBBDOOBBDO...",
            ".......OOWWWOO...OOWWWOO..OOWWWOOOO.....",
            ".......OOOOOOO...OOOOOOO..OOOOOOO.......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::AzureWyrm => [
            "........................................",
            "..........GGG...........................",
            "........OOGGGO..........AA......AA......",
            ".......OOOOOOO.........OAAOOOOOOAAO.....",
            ".....OBLLLLLLBBO......OAAAGGGGGGAAO.....",
            "....OBBBEEKKBBBBO....OAAABBBBBBBBAAO....",
            "...ODBBBKKKKBBBBBO..OAABBBSSSSSSBBAAO...",
            "...OSSSSBBBBBBBBBBO.OABBSSSSOOOOSSBAO...",
            "....OOOOOOSSSSBBBBBOOBBSSSOO....OSSBAO..",
            "....OAAWWAOOSSBBBBBBBBSSSO.......OSBAO..",
            "...OAWWWWWAOOBBBBBBBBSSSO........OSBAO..",
            "...OAAWWWWAOODBBBBBBSSSO........OOSBAO..",
            "....OOOOOOO.ODBBBBBSSSSOOOOOOOOOOSSBAO..",
            "............ODBBBBSSSSSSSSSSSSSSSSBBAO..",
            ".............ODBBBBBBBBBBBBBBBBBBBBAO...",
            "..............OOODDDDDDDDDDDDDDDDDOO....",
            "................OOOOOOOOOOOOOOOOOO......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::EmberFox => [
            "........................................",
            "......OOO..OOO..........................",
            ".....OBWB..BWBO.........AA......AA......",
            "....OBWWBOOBWWBO.......AWWA....AWWA.....",
            "...OBLLLLLLLLLLBO.......AA.OOOO.AA......",
            "..OBBBEEKKBBBBBBBO.....OSSSSSSSSSO......",
            ".ODBBBKKKKBBBBBBBBO...OSSSSBBBBSSSO.....",
            ".ODBBBPPPBBBBBBBBBBO.OSSSSBBBBBBSSSO....",
            "..OOOOOWWWWWWWBBBBBBBOSSSBBBBBBBBSSSO...",
            "......OWWWWWWWWBBBBBBBBBBBBBBBBBBSSSO...",
            "......OWWWSSSSSSSSSSSSSSSSBBBBBBBSSSO...",
            ".......OOODBBBSSSSSSSSSSSSSBBBBBBSSO....",
            ".........ODBBBBBBBBBBBBBBBBBBBBBBSSO....",
            ".........ODBBBDDDDDDDDDDDDDDDDBBSSO.....",
            "........ODBBDO..ODBBDO....ODBBDOOOO.....",
            ".......OOWWOO...OOWWOO....OOWWOO........",
            ".......OOOOOO...OOOOOO....OOOOOO........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::PhoenixChick => [
            "........................................",
            "........GG..GG.........OOOOOO...........",
            ".......OGGAAGGO......OOAAAAAAO..........",
            "......OOBLLLLLLBO...OAAAGGGGGAO.........",
            "...GGOBBBBBBBBBBBO.OAAGGBBBBGGAO........",
            "..OGGGGBBEEKKBBBBBOOAAGBBBBBBGAO........",
            "...OOOOOBKKKKBBBBBBOAGBBBBBBBGAO........",
            ".......ODBBBBGGGGBBBAGBBBBBBGGAO........",
            "......ODBBBGGGGGGGGGBBBBBBBGGAAO........",
            "......ODBBBBSSSSSSSSBBBBBBBAAO...OOOO...",
            ".......ODBBBBSSSSSSSSSSBBBBDO..OOAAAAO..",
            "........ODBBBBBBBBBBBBBBBBBDOOOAGGGGGGA.",
            ".........OOODBBBDDDDDDBBBBDOAAAGGGGGGGA.",
            "...........ODBBDO....ODBBDO..OOOOOOOOO..",
            "..........OOGGGO.....OOGGGO.............",
            "..........OOOOOO.....OOOOOO.............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::MochiCat => [
            "........................................",
            "......OOO.....OOO.......................",
            ".....OBPBO...OBPBO......................",
            "....OBPPPBOoobPPPBO.....................",
            "...OBLLLLLLLLLLLLBBO....................",
            "..OBBBBBBBBBBBBBBBBBO............OOOO...",
            "..OBBBEEKKBBBBEEKKBBBO..........OBBBBO..",
            ".ODBBBKKKKBBBBKKKKBBBDO........OBBBBBAO.",
            ".ODBBBPPPBBBBBBPPPBBBDO.......OBBBBBAO..",
            "..ODBBBBBBGGGGBBBBBBDO.......OBBBBBAO...",
            "...OOOOOSSSSSSSSSSSSSSSSBBBBBBBBBAO.....",
            ".......ODBBBBSSSSSSSSSSSSSSSSBBBBDO.....",
            "......ODBBBBBBSSSSSSSSSSSSSSSBBBBDO.....",
            "......ODBBBBBBBBBBBBBBBBBBBBBBBBBDO.....",
            "......ODBBBDDDDDDDDDDDDDDDDDDBBBDO......",
            ".....ODBBDO..ODBBDO....ODBBDOODBBDO.....",
            "....OOWWOO...OOWWOO....OOWWOO.OOWWOO....",
            "....OOOOOO...OOOOOO....OOOOOO.OOOOOO....",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::BobaPup => [
            "........................................",
            "......OOOO....OOOO......................",
            ".....OSSSSOOOOSSSSO.....................",
            "....OSSSSLLLLLLSSSSO....................",
            "...OSSSSBBBBBBBBSSSSO.............OOO...",
            "...OSSSBBEKKBBEKKSSSO............OSSSO..",
            "....OOOBBKKKBBKKKBBOOO..........OSSSSO..",
            "......ODBBPPPSSPPPBBDO.........OBBBSSO..",
            ".......OOOSSSSSSBBBBBBBBBBBBBBBBBBSSO...",
            ".........ODBBBBSSOSOSOSOSOSOSSBBBBDO....",
            "........ODBBBBBBSSSSSSSSSSSSSSBBBBDO....",
            "........ODBBBBBBBBBBBBBBBBBBBBBBBBDO....",
            "........ODBBBDDDDDDDDDDDDDDDDDDBBBDO....",
            ".......ODBBDO..ODBBDO....ODBBDOODBBDO...",
            "......OOWWOO...OOWWOO....OOWWOO.OOWWOO..",
            "......OOOOOO...OOOOOO....OOOOOO.OOOOOO..",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::CloudBun => [
            "..........OOOO............OOOO..........",
            ".........OBLPBO..........OBLPBO.........",
            ".........OBPPBO..........OBPPBO.........",
            ".........OBPPBO..........OBPPBO.........",
            ".........OBBBBOOOOOOOOOOOOBBBBO.........",
            "........OBLLLBBBBBBBBBBBBBBLLLBO........",
            ".......OBBBBBBBBBBBBBBBBBBBBBBBBO.......",
            "......ODBBEEKKBBBBBBBBBBBBEEKKBBDO......",
            "......ODBBKKKKBBBBBBBBBBBBKKKKBBDO......",
            "......ODBBPPPBBBBBBPPBBBBBBPPPBBDO......",
            ".......ODBBBBBBBBOOOOOOBBBBBBBBDO.......",
            "......OOODBBBBBBSSSSSSSSBBBBBBDOOO......",
            ".....OWWWBBBBBBBSSSSSSSSBBBBBBBWWWO.....",
            "....OWWWWWBBBBBBSSSSSSSSBBBBBBWWWWWO....",
            ".....OWWWWBBBBBBBBBBBBBBBBBBBBWWWWO.....",
            "......OOOWWBBBBBBBBBBBBBBBBBBWWOOO......",
            ".........OOODBBBOOOOOOOOBBBDOOO.........",
            "...........OOOOO........OOOOO...........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::StarPengu => [
            "........................................",
            ".............OOOOOOOOOOOOOO.............",
            "...........OOBBLLLLLLLLLLBBOO...........",
            ".........OOBBBBBBBGAAGBBBBBBBOO.........",
            "........ODBBBBBWWWWWWWWWWBBBBBDO........",
            "........ODBBBBEEKKWWWWEEKKBBBBDO........",
            "........ODBBBBKKKKWWWWKKKKBBBBDO........",
            "........ODBBBBWWWWGGGGWWWWBBBBDO........",
            ".......OODBBBWWWWGGGGGGWWWWBBBDOO.......",
            "......ODBBBBBAAAAAAAAAAAAAABBBBBDO......",
            ".....ODBBBBBWWWWWWWWWWWWWWWWBBBBBDO.....",
            ".....ODBBBBWWWWWWWWGGWWWWWWWWBBBBDO.....",
            ".....ODBBBBWWWWWWWGGGGWWWWWWWBBBBDO.....",
            "......OODBBWWWWWWWWWWWWWWWWWWBBDOO......",
            ".......OOODWWWWWWWWWWWWWWWWWWDOOO.......",
            ".........OOODBBBBBBBBBBBBBBDOOO.........",
            "..........OOOGGGGOOOOOOGGGGOOO..........",
            "............OOOOO......OOOOO............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::SlimeKing => [
            "........................................",
            "...........GG.....GGGG.....GG...........",
            "..........OGGGO..OGAAGGO..OGGGO.........",
            "..........OGGGGGGGGGGGGGGGGGGGO.........",
            "..........OOOOOOOOOOOOOOOOOOOOO.........",
            "........OOOBLLLLLLLLLLLLLLLLBBOOO.......",
            ".......OBBBWWWBBBBBBBBBBBBBBBBBBBO......",
            "......OBBBWWWBBBBBBBBBBBBBBBBBBBBBO.....",
            ".....ODBBWWBBBBBBBBBBBBBBBBBBBBBBBDO....",
            ".....ODBBBBBEEKKBBBBBBBBEEKKBBBBBBDO....",
            ".....ODBBBBBKKKKBBBBBBBBKKKKBBBBBBDO....",
            ".....ODBBBPPPBBBBWWWWWWBBBBPPPBBBBDO....",
            ".....ODBBBBBBBBSSSSSSSSSSBBBBBBBBBDO....",
            ".....ODBBBBBBBBBBBBBBBBBBBBBBBBBBBDO....",
            "......OODDDDDDDDDDDDDDDDDDDDDDDDDOO.....",
            "........OOOOOOOOOOOOOOOOOOOOOOOO........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
    }
}

/// Agile 3/4 Side-Profile **TEEN (`Teen`)** bitmaps (40×22).
fn teen_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::CrystalUni => [
            "........................................",
            "........................................",
            "........GG..............................",
            ".........OGG...OO......OOOO.............",
            ".......OOOGGOOBLBO...OOWWWWOO...........",
            "......OBLLLBBBBBBBO.OWWWSSWWWO..........",
            ".....OBBBEKKBBBBBBBOOWWSSSWWWO..........",
            "....ODBBBPPPBBBBBBBBOOWWWWWOO...........",
            "....OSSSSBBBBBBBBBBBBOOOOOO.............",
            ".....OOOOOODBBBBSSSSSSSSBBBBOOO..OO.....",
            "..........ODBBBBBSSSSSSSSBBBBBBBOAAO....",
            ".........ODBBBBBBBBBBBBBBBBBBBBBDOAO....",
            ".........ODBBBDDDDDDDDDDDDDDBBBDO.OO....",
            "........ODBBDO..ODBBDO....ODBBDO........",
            ".......OOGGGO...OOGGGO....OOGGGO........",
            ".......OOOOOO...OOOOOO....OOOOOO........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick => [
            "........................................",
            "........................................",
            "...........AA.........OOOOO.............",
            "..........OAAO......OOAAAAAO............",
            "........OBLLLLBBO..OAAGGGGAO............",
            ".......OBBBEEKKBBO.OAGDDDGGAO...........",
            "......ODBBBKKKKBBBOOAGDDDGGAO...........",
            "......OSSSSBBBBBBBBOAGGGGGAAO...........",
            ".......OOOOOSSSSBBBBBBBBGGAAO...........",
            "...........ODBBBBSSSSSSSSBBDO....AA.....",
            "..........ODBBBBBBSSSSSSSSBBDO..AGGA....",
            "..........ODBBBBBBBBBBBBBBBBDO.OAAO.....",
            ".........ODBBBDO......ODBBBDOOOOBO......",
            "........OOWWWOO.......OOWWWOOOOOO.......",
            "........OOOOOOO.......OOOOOOO...........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::AzureWyrm => [
            "........................................",
            "........................................",
            "...........AA............AA.............",
            "..........OAAO..........OAAO............",
            "........OBLLLLBBO......OAAAAO...........",
            ".......OBBBEEKKBBO....OAABBBAAO.........",
            "......ODBBBKKKKBBBO..OABBSSSSBAO........",
            "......OSSSSBBBBBBBBOOBBSSSOOSSBAO.......",
            ".......OOOOOSSSSBBBBBBSSSO...OSBAO......",
            ".......OAWWAOOBBBBBBBSSSO....OSBAO......",
            ".......OAWWAOODBBBBBSSSSOOOOOSSBAO......",
            "........OOOO.ODBBBBSSSSSSSSSSSBBAO......",
            "..............ODBBBBBBBBBBBBBBBBAO......",
            "...............OOODDDDDDDDDDDDDOO.......",
            ".................OOOOOOOOOOOOOO.........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::SlimeKing => [
            "........................................",
            "........................................",
            "..............GG....GG..................",
            ".............OGGGGGGGGO.................",
            ".............OOOOOOOOOO.................",
            "...........OOBBLLLLLLBBOO...............",
            ".........OOBBWWBBBBBBBBBBBOO............",
            "........ODBBBEEKKBBBBEEKKBBBDO..........",
            "........ODBBBKKKKBBBBKKKKBBBDO..........",
            "........ODBBPPPBBBWWBBBPPPBBDO..........",
            "........ODBBBBBSSSSSSSSBBBBBDO..........",
            ".........OODDDDDDDDDDDDDDDDDOO..........",
            "...........OOOOOOOOOOOOOOOO.............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        _ => [
            "........................................",
            "........................................",
            ".........OOO.....OOO....................",
            "........OBPBO...OBPBO...................",
            ".......OBLLLBBBBBLLLBO..........OOO.....",
            "......OBBBEEKKBBEEKKBBO........OBBBO....",
            ".....ODBBBKKKKBBKKKKBBDO......OBBBAO....",
            ".....ODBBBPPPBBBBPPPBBDO.....OBBBAO.....",
            "......OOOOOSSSSSSSSSSSSBBBBBBBBBAO......",
            "..........ODBBBBSSSSSSSSSSSSBBBBDO......",
            ".........ODBBBBBBBBBBBBBBBBBBBBBDO......",
            ".........ODBBBDDDDDDDDDDDDDDBBBDO.......",
            "........ODBBDO..ODBBDO....ODBBDO........",
            ".......OOWWOO...OOWWOO....OOWWOO........",
            ".......OOOOOO...OOOOOO....OOOOOO........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
    }
}

/// Cute Chibi **BABY (`Baby`)** bitmaps (40×22).
fn baby_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::CrystalUni => [
            "........................................",
            "........................................",
            ".............GG.........................",
            "..............OGG..OO...................",
            "............OOOGGOOBBO..................",
            "...........OBLLLBBBBBBBO................",
            "..........OBBBEEKKBBBBBBO...............",
            ".........ODBBBKKKKBBBBBBBO..OOO.........",
            ".........OSSSSPPPBBBBBBBBBOOWWWO........",
            "..........OOOOODBBBSSSSSSBBWWWWO........",
            "..............ODBBBBSSSSSSBBBDOO........",
            ".............ODBBBBBBBBBBBBBBDO.........",
            "............ODBBDO......ODBBDO..........",
            "...........OOGGGO.......OOGGGO..........",
            "...........OOOOOO.......OOOOOO..........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::AzureWyrm => [
            "........................................",
            "........................................",
            ".............AA..........AA.............",
            "............OAAOOOOOOOOOOAAO............",
            "...........OBLLLBBBBBBBBLLLBO...........",
            ".........OOOBBEEKKBBBBEEKKBBOOO.........",
            "........OAAOBBKKKKBBBBKKKKBBOAAO........",
            ".........OOODBBPPPBBBBPPPBBDOOO.........",
            "...........ODBBBBSSSSSSBBBBDO..AA.......",
            "............OODBBBBBBBBBBDOO..OGGA......",
            ".............OODBBBOOOOBBDOOOOOOO.......",
            "..............OOOOO....OOOO.............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        _ => [
            "........................................",
            "........................................",
            "............OOO..........OOO............",
            "...........OBPBOOOOOOOOOOBPBO...........",
            "..........OBLLLBBBBBBBBBBLLLBO..........",
            ".........OBBBEEKKBBBBBBEEKKBBBO.........",
            ".........OBBBKKKKBBBBBBKKKKBBBO.........",
            ".........ODBBPPPBBBBBBBBPPPBBDO.........",
            "..........ODBBBBBSSSSSSBBBBBDO..OO......",
            "...........OODBBBBBBBBBBBBDOO..OBO......",
            "............OOODBBBOOOOBBDOOOOOOO.......",
            "..............OOOOO....OOOO.............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
    }
}

/// Species-customized 40×22 Egg stage bitmap with rocking animation.
fn egg_bitmap(species: PetSpecies, frame_idx: usize) -> [&'static str; CANVAS_H] {
    let mut base = if frame_idx.is_multiple_of(2) {
        [
            "........................................",
            "........................................",
            "...............OOOOOOOOOO...............",
            ".............OOBBLLLLLLBBOO.............",
            "...........OOBBBWWWBBBBBBBBOO...........",
            "..........OBBBBBWWWBBBBAAAABBO..........",
            ".........OBBBBBBBBBBBBBAAAABBBO.........",
            ".........OBBBBBAAAABBBBBBBBBBBO.........",
            "........ODBBBBBAAAABBBBBSSSSBBDO........",
            "........ODBBBBBBBBBBBBBBSSSSBBDO........",
            "........ODBBBBSSSSBBBBBBBBBBBBDO........",
            "........ODBBBBSSSSBBBBBBAAAABBDO........",
            ".........ODBBBBBBBBBBBBBAAAABDO.........",
            ".........OODBBBBBBBBBBBBBBBBDOO.........",
            "..........OOODDDDDDDDDDDDDDOOO..........",
            ".........OOSSSSSSSSSSSSSSSSSSOO.........",
            "........OAAAAAAAAAAAAAAAAAAAAAAO........",
            ".........OOOOOOOOOOOOOOOOOOOOOO.........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ]
    } else {
        [
            "........................................",
            "........................................",
            "................OOOOOOOOOO..............",
            "..............OOBBLLLLLLBBOO............",
            "............OOBBBWWWBBBBBBBBOO..........",
            "...........OBBBBBWWWBBBBAAAABBO.........",
            "..........OBBBBBBBBBBBBBAAAABBBO........",
            "..........OBBBBBAAAABBBBBBBBBBBO........",
            ".........ODBBBBBAAAABBBBBSSSSBBDO.......",
            ".........ODBBBBBBBBBBBBBBSSSSBBDO.......",
            ".........ODBBBBSSSSBBBBBBBBBBBBDO.......",
            ".........ODBBBBSSSSBBBBBBAAAABBDO.......",
            "..........ODBBBBBBBBBBBBBAAAABDO........",
            ".........OODBBBBBBBBBBBBBBBBDOO.........",
            "..........OOODDDDDDDDDDDDDDOOO..........",
            ".........OOSSSSSSSSSSSSSSSSSSOO.........",
            "........OAAAAAAAAAAAAAAAAAAAAAAO........",
            ".........OOOOOOOOOOOOOOOOOOOOOO.........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ]
    };

    if matches!(
        species,
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::SlimeKing
    ) {
        base[1] = "................GG....GG................";
    } else if matches!(species, PetSpecies::CrystalUni) {
        base[1] = "...................GG...................";
    } else if matches!(
        species,
        PetSpecies::MochiCat | PetSpecies::CloudBun | PetSpecies::EmberFox
    ) {
        base[1] = "..............OO........OO..............";
    }
    base
}

/// Spirit Memorial & Halo Ghost sprite when `pet.is_alive == false` (`PetMood::PassedOut`).
fn ghost_bitmap() -> [&'static str; CANVAS_H] {
    [
        "........................................",
        "..............GGGGGGGGGGGG..............",
        ".............G............G.............",
        "..............GGGGGGGGGGGG..............",
        "........................................",
        "...............OOOOOOOOOO...............",
        ".............OOWWWWWWWWWWOO.............",
        "............OWWWWWWWWWWWWWWO............",
        "...........OWWWKKWWWWWWKKWWWO...........",
        "...........OWWWKKWWWWWWKKWWWO...........",
        "...........OWWWPPWWWWWWPPWWWO...........",
        "...........OWWWWWWWOOOWWWWWWO...........",
        "...........OWWWWWWWWWWWWWWWWO...........",
        "............OWWWWWWWWWWWWWWO............",
        ".............OWW.OWWWWWO.WWO............",
        ".............OO...OOOOO...OO............",
        "........................................",
        "........................................",
        "........................................",
        "........................................",
        "........................................",
        "........................................",
    ]
}

/// Build the 40×22 character grid for any `(species, stage, mood, frame_idx, walk_offset, facing_right)`.
fn build_grid_with_motion(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
    walk_offset: isize,
    facing_right: bool,
) -> [[char; CANVAS_W]; CANVAS_H] {
    let raw = if mood == PetMood::PassedOut {
        ghost_bitmap()
    } else {
        match stage {
            EvolutionStage::Egg => egg_bitmap(species, frame_idx),
            EvolutionStage::Baby => baby_bitmap(species),
            EvolutionStage::Teen => teen_bitmap(species),
            EvolutionStage::Adult => adult_bitmap(species),
            EvolutionStage::CyberElder => elder_bitmap(species),
        }
    };

    let mut grid = [['.'; CANVAS_W]; CANVAS_H];
    for (y, row) in raw.iter().enumerate() {
        for (x, ch) in row.chars().take(CANVAS_W).enumerate() {
            grid[y][x] = ch;
        }
    }

    // Flip horizontally when strolling right!
    if facing_right && stage != EvolutionStage::Egg && mood != PetMood::PassedOut {
        for row in &mut grid {
            row.reverse();
        }
    }

    // Apply horizontal walking offset (-3..=3 columns) clamped so non-empty pixels never clip!
    if walk_offset != 0 && stage != EvolutionStage::Egg {
        let mut min_x = CANVAS_W;
        let mut max_x = 0usize;
        for row in &grid {
            for (x, &c) in row.iter().enumerate() {
                if c != '.' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                }
            }
        }
        if min_x <= max_x {
            let safe_shift = walk_offset
                .max(-(min_x as isize))
                .min((CANVAS_W - 1 - max_x) as isize);
            if safe_shift != 0 {
                let mut shifted_h = [['.'; CANVAS_W]; CANVAS_H];
                for (src_row, dst_row) in grid.iter().zip(shifted_h.iter_mut()) {
                    for (x, &cell) in src_row.iter().enumerate() {
                        let nx = x as isize + safe_shift;
                        if (0..CANVAS_W as isize).contains(&nx) {
                            dst_row[nx as usize] = cell;
                        }
                    }
                }
                grid = shifted_h;
            }
        }
    }

    // Ground-align the sprite so its feet rest directly on the bottom row (above the pedestal!)
    let mut last_non_empty = 0usize;
    for (y, row) in grid.iter().enumerate() {
        if row.iter().any(|&c| c != '.') {
            last_non_empty = y;
        }
    }
    let shift_down = (CANVAS_H - 1).saturating_sub(last_non_empty);
    if shift_down > 0 {
        let mut aligned = [['.'; CANVAS_W]; CANVAS_H];
        aligned[shift_down..(last_non_empty + shift_down + 1)]
            .copy_from_slice(&grid[..(last_non_empty + 1)]);
        grid = aligned;
    }

    // Dynamic eye expressions based on rich Emotions (`PetMood`)!
    let blink = frame_idx % 2 == 1 || matches!(mood, PetMood::Sleeping | PetMood::Sleepy);
    if stage != EvolutionStage::Egg && mood != PetMood::PassedOut {
        if blink {
            for row in &mut grid {
                for cell in row.iter_mut() {
                    if *cell == 'E' || *cell == 'K' || (mood == PetMood::Sleeping && *cell == 'R') {
                        *cell = 'O';
                    }
                }
            }
        } else if mood == PetMood::Angry {
            for row in &mut grid {
                for cell in row.iter_mut() {
                    if *cell == 'E' {
                        *cell = 'R';
                    }
                }
            }
        } else if matches!(mood, PetMood::Sick | PetMood::Overstuffed) {
            for row in &mut grid {
                for cell in row.iter_mut() {
                    if *cell == 'E' {
                        *cell = 'P';
                    }
                }
            }
        }
    }

    // Gentle 1-pixel vertical breathing hop on frame 1 when awake & hatched
    if stage != EvolutionStage::Egg
        && !matches!(mood, PetMood::Sleeping | PetMood::PassedOut)
        && frame_idx % 2 == 1
    {
        let mut shifted = [['.'; CANVAS_W]; CANVAS_H];
        shifted[..CANVAS_H - 1].copy_from_slice(&grid[1..CANVAS_H]);
        grid = shifted;
    }

    grid
}

fn build_grid(
    species: PetSpecies,
    stage: EvolutionStage,
    mood: PetMood,
    frame_idx: usize,
) -> [[char; CANVAS_W]; CANVAS_H] {
    build_grid_with_motion(species, stage, mood, frame_idx, 0, false)
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
        'O' | '0' => Some(Color::Rgb(27, 22, 44)),
        'D' => Some(scale_rgb(primary, 0.68)),
        'B' | 'b' => Some(primary),
        'L' => Some(scale_rgb(primary, 1.22)),
        'S' | 's' => Some(secondary),
        'A' | 'a' => Some(accent),
        'R' | 'r' => Some(Color::Rgb(255, 55, 75)),
        'W' | 'E' => Some(Color::Rgb(250, 252, 255)),
        'K' => Some(Color::Rgb(18, 14, 30)),
        'P' => Some(Color::Rgb(255, 110, 165)),
        'G' | 'g' => Some(Color::Rgb(255, 220, 70)),
        _ => Some(primary),
    }
}

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

pub fn get_sprite_frames(species: PetSpecies, stage: EvolutionStage, mood: PetMood) -> Vec<String> {
    let g0 = build_grid(species, stage, mood, 0);
    let g1 = build_grid(species, stage, mood, 1);
    vec![render_grid_plain(&g0), render_grid_plain(&g1)]
}

/// Ambient retro status particles above the pixel sprite for all 13 rich emotions!
pub fn get_mood_particles(mood: PetMood, tick: usize) -> &'static str {
    let frames: &[&str] = match mood {
        PetMood::Idle => &["·   ✦   ·   ★   ·   ✦   ·   ★   ·", "✦   ·   ★   ·   ✦   ·   ★   ·   ✦"],
        PetMood::Eating => &["♪   ♥   ★   ♥   ♪   ★   ♪   ♥   ♪", "★   ♪   ♥   ♪   ★   ♥   ★   ♪   ★"],
        PetMood::Sleeping => &["☾   z   Z   zZ  ·   ✦   ☾   z   Z", "✦   Z   zZ  z   ☾   ·   ✦   Z   z"],
        PetMood::Sleepy => &["·   z   ·   ☾   ·   z   ·   ☾   ·", "☾   ·   z   ·   ☾   ·   z   ·   ☾"],
        PetMood::Happy | PetMood::Playful => &[
            "✦   ♥   ★   ♥   ✦   ♥   ✦   ★   ♥",
            "♥   ✦   ♥   ✦   ★   ✦   ♥   ✦   ★",
        ],
        PetMood::Excited | PetMood::Proud => &[
            "★   ✦   ★   ♦   ★   ✦   ★   ♦   ★",
            "♦   ★   ✦   ★   ♦   ★   ✦   ★   ♦",
        ],
        PetMood::Loved => &["♥   ♥   ✦   ♥   ♥   ✦   ♥   ♥   ✦", "✦   ♥   ♥   ✦   ♥   ♥   ✦   ♥   ♥"],
        PetMood::Lonely => &[".   ·   💧  ·   .   ·   💧  ·   .", "·   .   ·   💧  ·   .   ·   💧  ·"],
        PetMood::Angry => &["💢  ·   ⚡  ·   💢  ·   ⚡  ·   💢", "⚡  ·   💢  ·   ⚡  ·   💢  ·   ⚡"],
        PetMood::Overstuffed => &[
            "~   o   O   ~   o   O   ~   o   O",
            "O   ~   o   O   ~   o   O   ~   o",
        ],
        PetMood::Sick => &[".   ~   x   ~   .   x   .   ~   x", "~   .   x   .   ~   .   x   .   ~"],
        PetMood::Training => &["★   ♦   ▲   ♦   ★   ▲   ★   ♦   ▲", "♦   ★   ▲   ★   ♦   ★   ♦   ▲   ★"],
        PetMood::PassedOut => &[
            "✝   ·   👻  ·   ✦   ·   👻  ·   ✝",
            "·   👻  ·   ✦   ·   👻  ·   ✦   ·",
        ],
    };
    frames[(tick / 4) % frames.len()]
}

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

/// Autonomous horizontal strolling motion (`walk_offset`, `facing_right`) so your
/// companion physically walks around the habitat when awake!
fn autonomous_motion(pet: &Pet, tick: usize) -> (isize, bool) {
    if !pet.is_alive
        || pet.stage == EvolutionStage::Egg
        || pet.is_sleeping
        || pet.is_sick
        || pet.has_toothache
    {
        return (0, false);
    }
    let phase = (tick / 8) % 12;
    let waypoints: [(isize, bool); 12] = [
        (0, false),
        (-1, false),
        (-2, false),
        (-2, false),
        (-1, true),
        (0, true),
        (1, true),
        (2, true),
        (2, true),
        (1, false),
        (0, false),
        (0, false),
    ];
    waypoints[phase]
}

/// TrueColor 40×22 Half-Block Pixel Art [`Line`]s for the habitat LCD screen.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 8 == 7);
    let (walk_offset, facing_right) = autonomous_motion(pet, tick);
    let grid = build_grid_with_motion(
        pet.species,
        pet.stage,
        pet.mood,
        frame_idx,
        walk_offset,
        facing_right,
    );
    let (_, secondary, accent) = species_palette(pet.species);

    let mut out = Vec::with_capacity(14);
    out.push(Line::from(Span::styled(
        get_mood_particles(pet.mood, tick).to_string(),
        Style::default().fg(accent),
    )));
    out.extend(render_grid_colored(&grid, pet.species));
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
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

/// TrueColor 40×22 Half-Block Pixel Art [`Line`]s for the Step 1 Companion Selector.
pub fn render_colored_preview(
    species: PetSpecies,
    stage: EvolutionStage,
    tick: usize,
) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 8 == 7);
    let grid = build_grid(species, stage, PetMood::Happy, frame_idx);
    let (_, secondary, _) = species_palette(species);
    let mut out = render_grid_colored(&grid, species);
    out.push(Line::from(Span::styled(
        "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        Style::default().fg(secondary),
    )));
    out
}

pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// What your companion is saying out loud right now (`💬`).
/// Immediately displays direct first-person feedback (`pet.active_reaction_speech()`)
/// when you interact or spam an action!
pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if let Some(ref reaction) = pet.last_pet_speech {
        return reaction.clone();
    }
    if !pet.is_alive || pet.mood == PetMood::PassedOut {
        return format!(
            "💬 \"👻 {} has passed out... Press [H] for Phoenix Revival!\"",
            pet.name
        );
    }
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 80 {
            "💬 \"*crack* Almost ready to hatch!\"".to_string()
        } else {
            "💬 \"A warm heartbeat waits inside...\"".to_string()
        };
    }
    if pet.has_toothache {
        return "💬 \"ADUH!🦷 Gigiku ngilu! Kasih obat [H] dong!\"".to_string();
    }
    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return "💬 \"Zzz... mimpi indah makan camilan...\"".to_string();
    }
    if pet.is_sick || pet.mood == PetMood::Sick {
        return "💬 \"Uhuk...🤒 Badan lemes, butuh obat [H]...\"".to_string();
    }
    if pet.hunger == 0 {
        return "💬 \"PERUTKU KERONCONGAN PARAH!🍙 Tolong suapin [F]!\"".to_string();
    }
    if pet.poop_count > 0 {
        return "💬 \"Ih bau!🚽 Mandiin aku pakai [C] dong!\"".to_string();
    }
    if pet.hunger <= 30 {
        return format!("💬 \"Laper nih... pengen {} [F]!\"", pet.species.favorite_snack());
    }
    if pet.energy <= 25 {
        return "💬 \"*Hoaaam*🥱 Ngantuk berat... bobo yuk [S]?\"".to_string();
    }

    let chatter = [
        format!("💬 \"{} seneng banget lihat kamu! ([Y] Ngobrol)\"", pet.name),
        format!("💬 \"Hmm... pengen nyemil {} deh~\"", pet.species.favorite_snack()),
        "💬 \"Main 3-in-1 Retro Arcade [G] bareng yuk!\"".to_string(),
        "💬 \"Elus kepalaku [T] atau ngobrol [Y] yuk!\"".to_string(),
    ];
    chatter[(tick / 48) % chatter.len()].clone()
}

/// Inner thought bubble (`💭`) revealing what your companion is thinking or doing!
pub fn thought_bubble(pet: &Pet, tick: usize) -> String {
    if let Some(ref thought) = pet.last_pet_thought {
        return thought.clone();
    }
    if !pet.is_alive || pet.mood == PetMood::PassedOut {
        return "💭 (Spirit: Menunggu keajaiban Phoenix Tear [H] atau telur baru [N]...)".to_string();
    }
    if pet.stage == EvolutionStage::Egg {
        return "💭 (Thinking: Hangat banget di dalam cangkang ini~)".to_string();
    }
    if pet.has_toothache {
        return "💭 (Thinking: Kapok makan 3 permen manis berturut-turut!)".to_string();
    }
    if pet.hunger == 0 {
        return "💭 (Starving: Lemes banget... kalau nggak makan bisa pingsan!)".to_string();
    }
    if pet.is_sleeping {
        return format!(
            "💭 (Dreaming of a mountain of {}...)",
            pet.species.favorite_snack()
        );
    }
    let species_thoughts = match pet.species {
        PetSpecies::MochiCat => [
            "💭 (Activity: 🐾 Jalan-jalan santai sambil mengibaskan ekor~)",
            "💭 (Thinking: Sebentar lagi wujud Byakko Tiger-ku bangkit!)",
            "💭 (Activity: 👀 Menempelkan hidung ke kaca layar ngelihatin kamu!)",
        ],
        PetSpecies::BobaPup => [
            "💭 (Activity: 🐾 Mengejar kupu-kupu pixel di pojok kamar!)",
            "💭 (Thinking: Aku bakal tumbuh jadi Fenrir Dire Wolf yang gagah!)",
            "💭 (Activity: 🎵 Menggoyangkan ekor mengikuti ketukan keyboard-mu!)",
        ],
        PetSpecies::CloudBun => [
            "💭 (Activity: ☁️ Melompat-lompat ringan di atas awan lembut~)",
            "💭 (Thinking: Di wujud Elder nanti, aku penguasa badai petir!)",
            "💭 (Activity: 👀 Mengintip ke arahmu sambil menggoyang telinga!)",
        ],
        PetSpecies::StarPengu => [
            "💭 (Activity: ❄️ Meluncur santai di lantai es sanctuary~)",
            "💭 (Thinking: Berlatih keras demi mahkota trisula Frost Kaiser!)",
            "💭 (Activity: 🎵 Bersenandung lagu chiptune 8-bit sendirian~)",
        ],
        PetSpecies::IgnisDragon => [
            "💭 (Activity: 🔥 Latihan menyemburkan cincin api kecil di udara!)",
            "💭 (Thinking: Sayap nagaku makin kekar — Bahamut menanti!)",
            "💭 (Activity: 🐾 Berpatroli gagah menjaga koin emas sanctuary!)",
        ],
        PetSpecies::AzureWyrm => [
            "💭 (Activity: 🫧 Memutar mutiara laut bercahaya di antara cakarnya!)",
            "💭 (Thinking: Kekuatan Leviathan Samudra mengalir di sisikku...)",
            "💭 (Activity: 🌊 Melayang anggun mengikuti arus angin sanctuary~)",
        ],
        PetSpecies::EmberFox => [
            "💭 (Activity: 🦊 Menyalakan lentera api biru foxfire di sekelilingnya!)",
            "💭 (Thinking: Sebentar lagi ke-9 ekor Kyuubi-ku mekar sempurna!)",
            "💭 (Activity: 👀 Tersenyum jahil sambil melirik ke arahmu~)",
        ],
        PetSpecies::PhoenixChick => [
            "💭 (Activity: 🌞 Merentangkan sayap api emasnya biar tetap hangat!)",
            "💭 (Thinking: Bulu apiku makin membara menuju wujud Solar Garuda!)",
            "💭 (Activity: 🎵 Berkicau merdu dengan nada chiptune pagi hari!)",
        ],
        PetSpecies::CrystalUni => [
            "💭 (Activity: ✨ Tanduk kristalnya membiaskan pelangi ke dinding!)",
            "💭 (Thinking: Wujud Celestial Kirin Pegasus sudah makin dekat!)",
            "💭 (Activity: 🐾 Berderap anggun mengelilingi taman sanctuary~)",
        ],
        PetSpecies::SlimeKing => [
            "💭 (Activity: 👑 Membenarkan posisi mahkota emasnya sambil membal!)",
            "💭 (Thinking: Sebentar lagi aku jadi Titan Slime Emperor raksasa!)",
            "💭 (Activity: 💧 Berkilau bening memantulkan cahaya layar!)",
        ],
    };
    species_thoughts[(tick / 48) % species_thoughts.len()].to_string()
}

pub fn mood_dialogue(pet: &Pet) -> String {
    speech_bubble(pet, 0)
}

pub fn mood_color(mood: PetMood) -> Color {
    match mood {
        PetMood::Idle => Color::Cyan,
        PetMood::Eating => Color::LightGreen,
        PetMood::Sleeping | PetMood::Sleepy => Color::LightBlue,
        PetMood::Happy | PetMood::Playful | PetMood::Loved => Color::LightMagenta,
        PetMood::Excited | PetMood::Proud | PetMood::Training => Color::LightYellow,
        PetMood::Lonely | PetMood::Overstuffed => Color::Yellow,
        PetMood::Angry | PetMood::Sick => Color::LightRed,
        PetMood::PassedOut => Color::Gray,
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
    fn test_all_species_stages_and_moods_render_40x22_half_block_pixel_sprites() {
        for species in PetSpecies::all() {
            let baby = get_sprite_frames(*species, EvolutionStage::Baby, PetMood::Idle);
            let teen = get_sprite_frames(*species, EvolutionStage::Teen, PetMood::Idle);
            let adult = get_sprite_frames(*species, EvolutionStage::Adult, PetMood::Idle);
            let elder = get_sprite_frames(*species, EvolutionStage::CyberElder, PetMood::Idle);
            assert_ne!(baby[0], teen[0]);
            assert_ne!(teen[0], adult[0]);
            assert_ne!(adult[0], elder[0]);

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
        assert_eq!(w_count_0, w_count_1);
    }

    #[test]
    fn test_autonomous_strolling_and_slow_readable_thoughts() {
        let mut pet = Pet::new("Mochi");
        pet.stage = EvolutionStage::Adult;
        let s0 = render_colored_sprite(&pet, 0);
        let s16 = render_colored_sprite(&pet, 16);
        assert!(!s0.is_empty());
        assert!(!s16.is_empty());
        assert_eq!(thought_bubble(&pet, 0), thought_bubble(&pet, 30));
        assert_eq!(speech_bubble(&pet, 0), speech_bubble(&pet, 30));
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
        }
        for species in PetSpecies::all() {
            let preview = render_species_preview(*species);
            assert_eq!(preview.len(), CANVAS_H / 2);
            let colored_preview = render_colored_preview(*species, EvolutionStage::CyberElder, 0);
            assert!(!colored_preview.is_empty());
        }
        let _ = sky_scene(500);
    }
}
