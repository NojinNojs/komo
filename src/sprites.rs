//! 40×24 TrueColor Half-Block Pixel Art Engine (`▀`, `▄`, `█`) for
//! **Mochigotchi v1.0.0**.
//!
//! Renders a genuine **40×24 pixel canvas** (40 columns × 12 terminal rows of
//! `1:1` square half-block pixels) with a dramatic **4-Stage Silhouette
//! Evolution** from **Cute Chibi (`Baby`) -> Agile (`Teen`) -> Fierce (`Adult`)
//! -> Badass Apex Sovereign (`CyberElder`)**:
//! - `.` = Transparent background
//! - `O` = Dark pixel outline (`#1b162a`)
//! - `D` = Deep armor/muscle shadow (`0.68 × primary`)
//! - `B` = Primary species body color
//! - `L` = Light highlight (`1.22 × primary`)
//! - `S` = Secondary / chest armor / belly plates
//! - `A` = Elemental accent (flames, tidal fins, foxfire, lightning)
//! - `R` = Fierce crimson/ember glow (`#ff3b4e`) for apex eyes & fire
//! - `G` = Royal Gold (horns, crowns, shoulder armor, claws)
//! - `W` = Pure white fangs, claws, pearl & fur (isolated from blinking!)
//! - `E` = Eye shine / sclera (turns into `O` when blinking/sleeping)
//! - `K` = Eye pupil (turns into `O` when blinking/sleeping)
//! - `P` = Rosy cheek / heart pink

use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub const CANVAS_W: usize = 40;
pub const CANVAS_H: usize = 24;

/// Badass, muscular, full-canvas (40×24) **APEX ELDER (`CyberElder`)** bitmaps!
/// Every species transforms from its cute baby origins into a fierce sovereign beast.
fn elder_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        // 🐉 05/10 Ignis Drake (Elder — Inferno Bahamut Sovereign)
        // Giant 40-col magma wings, twin golden horns, fierce crimson eyes, fangs, muscular claws & fire breath
        PetSpecies::IgnisDragon => [
            "......GG........................GG......",
            ".....OGGO......AA......AA......OGGO.....",
            ".OO...OGGO....OAOOOOOOOOAO....OGGO...OO.",
            "OAOO...OGGOOOBBLLLLLLLLLLBBOOGGO...OOAO",
            "OAAOO...OOOOBBBBBBBBBBBBBBBBOOOO...OOAAO",
            "OAGAAOOOO..ODBBREKBBBBKERBBDO..OOOOAAGAO",
            "OAGGAAAAOO.ODBBROOBBBBOORBBDO.OOAAAAGGAO",
            "OAGGGDBBBOOODBBBBWOOOOOWBBBBDOOOBBBDGGGA",
            "OAGGDBBBBBBBBBBBOWWWWWWOABBBBBBBBBBDGGAO",
            "OAGDBBBOOOODBBBSSSSSSSSSSBBBDOOOOBBBDGAO",
            "OADBBBO...ODBBGGGSSSSSSGGGBBDO...OBBBDAO",
            ".OABBBO..ODBBBSSSSSSSSSSSSBBBDO..OBBBAO.",
            "..OABBO.ODBBBBGGGSSSSSSGGGBBBBDO.OBBAO..",
            "...OABO.ODBBBBBSSSSSSSSSSBBBBBDO.OBAO...",
            "....OO..OWWDBBBBSSSSSSSSBBBBDBWWO.OO..AA",
            "........OWWODBBBBBBBBBBBBBBDOOWWO....AGA",
            "...........ODBBBBBBBBBBBBBBDO.......AGGA",
            "..........ODBBBBDOOOOOODBBBBDO.....OAGAO",
            ".........ODBBBBDO......ODBBBBDO...OAAOO.",
            "........OOWWWOO........OOWWWOO..OOAOO...",
            "........OOOOOOO..........OOOOOOOOOO.....",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐲 06/10 Azure Wyrm (Elder — Tidal Leviathan Sovereign)
        // Towering S-coiled Sea-and-Sky Dragon with grand antler horns, fanged dragon jaws,
        // spiked tsunami wing-fins, and serpentine coils guarding a blazing Moon Pearl!
        PetSpecies::AzureWyrm => [
            "....GG............................GG....",
            "...OGGO....AA..............AA....OGGO...",
            "....OGGO..OAAOOOOOOOOOOOOOOAAO..OGGO....",
            ".....OGGOOOBBLLLLLLLLLLLLLLBBOOGGO.....",
            "..AA..OOOOBBBBBBBBBBBBBBBBBBBBOOOO..AA..",
            ".OAAO...ODBBBREKBBBBBBBBKERBBBDO...OAAO.",
            "OAAAAOO.ODBBBROOBBSSSSBBOORBBBDO.OOAAAAO",
            "OAGGAAAOODBBBBWWOOOOOOOOOWWBBBBDOOAAAGGA",
            "OAGBBBBBBBBBBOWWWWWWWWWWWWOABBBBBBBBBGAO",
            ".OADBBBBOOOODBBOAAWWWWAAOBBDOOOOBBBBDAO.",
            "..OOOBBBO..ODBBAAWWWWWWAAABBDO..OBBBOOO.",
            "....OABBO.ODBBBAWWWWWWWWABBBBDO.OBBAO...",
            "...OABBBO.ODBBBBAAWWWWAAABBBBDO.OBBBAO..",
            "..OADBBBO..ODBBBBOOOOOOOBBBBDO..OBBBDAO.",
            ".OADBBBDO...OOODBBBBBBBBDOOOO...ODBBBDAO",
            ".OABBBDO..OOOBBBSSSSSSSSBBBOOO..ODBBBAO.",
            "..OOOOO..ODBBBBBSSSSSSSSBBBBBDO..OOOOO..",
            "........ODBBBBDDDDDDDDDDDDBBBBDO........",
            ".......ODBBBDOOOOOOOOOOOOOODBBBDO.......",
            "......OOWWWOO..............OOWWWOO......",
            "......OOOOOOO..............OOOOOOO......",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🦊 07/10 Kitsune Nine (Elder — Nine-Tailed Kyuubi Spirit Sovereign)
        // Massive 9-tailed foxfire halo spanning the full width, fierce kitsune mask & golden armor
        PetSpecies::EmberFox => [
            "..AA....AA....................AA....AA..",
            ".AWWA..AWWA...OOO......OOO...AWWA..AWWA.",
            "..AA...OSSO..OBWBO....OBWBO..OSSO...AA..",
            ".OSSO.OSSSSOOBWWWBOOOOBWWWBOOSSSSO.OSSO.",
            "OSSSSOOSSSSBOBLLLLBBBBLLLLBOBSSSSOOSSSSO",
            "OSSSSBBOSSBBOOBBBBBBBBBBBBOOBBSOBBSSSSO",
            ".OSSSSBBOBBBOODBBREKKERBBDOOBBBOBBSSSSO.",
            "..OSSSBBOBBBOODBBROOOORBBDOOBBBOBBSSSO..",
            ".OOOSSBBOBBBOOOODBBWWBBDOOOOBBBOBBSSOOO.",
            "OAAWSSBBOBBBGGGGBBSSSSBBGGGGBBBOBBSSWAAO",
            "OAWWSSBBOBBBGGGGSSSSSSSSGGGGBBBOBBSSWWAO",
            ".OOOSSBBOODBBBBSSSSSSSSSSBBBBDOOBBSSOOO.",
            "...OSSSBBODBBBBBSSSSSSSSBBBBBDOBBSO.....",
            "....OOOOOODBBBBBBBBBBBBBBBBBBDOOOOO.....",
            "........OWWDBBBBBBBBBBBBBBBBDBWWO.......",
            "........OWWODBBBDOOOOOODBBBDOOWWO.......",
            "...........ODBBBDO....ODBBBDO...........",
            "..........OOWWWOO......OOWWWOO..........",
            "..........OOOOOOO......OOOOOOO..........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🦅 08/10 Sol Phoenix (Elder — Solar Garuda Deity)
        // 40-column wingspan of golden & crimson solar fire, 5-plume sun crown & armored talons
        PetSpecies::PhoenixChick => [
            ".............GG..GG..GG..GG.............",
            ".AA.........OGGAAGGGGGGAAGGO.........AA.",
            "OAAO..AA.....OOBBLLLLLLBBOO.....AA..OAAO",
            "OAGAOOAAO...ODBBBBBBBBBBBBDO...OAAOOAGAO",
            "OAGGAAAAOO.ODBBREKGGGGKERBBDO.OOAAAAGGAO",
            "OAGGGGGGAAOODBBROOGGGGOORBBDOOAAAGGGGGAO",
            "OAGBBBBGGGAOODBBBBGGGGGBBBBDOOAGGGBBBGAO",
            "OADBBBBBBGGAAODBBGGGGGGGBBDOAAGGBBBBBDAO",
            ".OADBBBBBBBBBBBBSSSSSSSSBBBBBBBBBBBBDAO.",
            "..OADBBBBBBBOOOSSSSSSSSSSOOOBBBBBBBDAO..",
            "...OADBBBBBO..ODSSSSSSSSDO..OBBBBBDAO...",
            "....OOOOOOO..ODBBSSSSSSBBDO..OOOOOOO....",
            ".............ODBBBBBBBBBBDO.............",
            "............ODBBBBDDDDDBBBBDO...........",
            "...........OOOGGGGOOOOOGGGGOOO..........",
            "..........OAAAGGGGO...OGGGGAAAO.........",
            ".........OAGAGAGAGAO.OAGAGAGAGAO........",
            "..........OOOOOOOOO...OOOOOOOOO.........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🦄 09/10 Astra Unicorn (Elder — Celestial Kirin Pegasus)
        // Long spiral golden lance-horn, giant astral feathered wings, armored chest & lightning hooves
        PetSpecies::CrystalUni => [
            "...................GG...................",
            "..................OGGO..................",
            ".................OGGGGO.................",
            "..OO.........OO..OGGGGO..OO.........OO..",
            ".OWWOO......OBBOOOGGGGOOOBBO......OOWWO.",
            "OWWWWOO....OBLLLLBBBBBBLLLLBO....OOWWWWO",
            "OWWSSWWOO.ODBBREKBBBBBBKERBBDO.OOWWSSWWO",
            "OWWSSSSWWOODBBROOBBBBBBOORBBDOOWWSSSSWWO",
            ".OWWSSSSWWOOODBBBBWWWWBBBBDOOOWWSSSSWWO.",
            "..OWWSSSSBBOODBBGGGSSGGGBBDOOBBSSSSWWO..",
            "...OOOSSSBBOODBBSSSSSSSSBBDOOBBSSSOOO...",
            ".....OOOOOOOODBBBSSSSSSBBBDOOOOOOO..AA..",
            "...........ODBBBBBSSSSBBBBBDO......ASSA.",
            "..........ODBBBBBBBBBBBBBBBBDO.....ASSA.",
            ".........ODBBBBDDDDDDDDDDBBBBDO....OAA..",
            "........ODBBBDOOOOOOOOOOOODBBBDO........",
            ".......ODBBBDO............ODBBBDO.......",
            "......OOGGGGO..............OGGGGOO......",
            "......OOOOOOO..............OOOOOOO......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 👑 10/10 Royal Jelly (Elder — Titan Slime Emperor)
        // Colossal armored slime titan with grand jeweled crown, fierce glowing war-eyes & spiked gauntlets
        PetSpecies::SlimeKing => [
            "...........GG......GG......GG...........",
            "..........OGGO...OGRRGO...OGGO..........",
            ".........OGGGGOOOGGRRGGOOOGGGGO.........",
            ".........OGGGGGGGGGGGGGGGGGGGGO.........",
            ".........OOOOOOOOOOOOOOOOOOOOOO.........",
            "......OOOBLLLLLLLLLLLLLLLLLLLLBBOOO.....",
            "....OOBBBWWWWBBBBBBBBBBBBBBBBBBBBBOO....",
            "..OO..OBBWWBBBBBBBBBBBBBBBBBBBBBBBO..OO.",
            ".OGGO.ODBBBBBREKBBBBBBBBKERBBBBBDO.OGGO.",
            "OGBBGOODBBBBBROOBBBBBBBBOORBBBBBDOOGBBGO",
            "OBBBBGOODBBBBBBWWOOOOOOWWBBBBBBDOOGBBBBO",
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
            "........................................",
            "........................................",
        ],
        // 🐱 01/10 Neko Mochi (Elder — Byakko Saber Tiger Shogun)
        // Muscular armored tiger beast with saber fangs, golden shogun shoulder-armor & twin tails
        PetSpecies::MochiCat => [
            ".....OOOO......................OOOO.....",
            "....OBLLBO....................OBLLBO....",
            "...OBBPPPBO.....OOOOOOOO.....OBPPPBBO...",
            "...OBBPPPBOOOOOBBLLLLLLBBOOOOOBPPPBBO...",
            "....OBBBOOBBBBBBBBBBBBBBBBBBBBOOBBBO....",
            "..OO.OO..ODBBREKBBBBBBBBKERBBDO..OO.OO..",
            ".OGGO....ODBBROOBBAAAABBOORBBDO....OGGO.",
            "OGGGGGOO.ODBBBBWWOOOOOOWWBBBBDO.OOGGGGGO",
            "OGGGGGGGODBBBBBBOWWWWWWOABBBBBBDOGAGGGGO",
            ".OOOOOOODBBBBGGGSSSSSSSSGGGbbbBDOOOOOOO.",
            "......ODBBBBBSSSSSSSSSSSSSSBBBBBDO...OO.",
            ".....OWWDBBBBSSSSSSSSSSSSSSBBBBDBWWOOBBO",
            ".....OWWODBBBBSSSSSSSSSSSSBBBBDOOWWOBBAO",
            "........ODBBBBBBBBBBBBBBBBBBBBDO...OBAO.",
            "........ODBBBBDDDDDDDDDDDDBBBBDO..OBAO..",
            ".......ODBBBDOOOOOOOOOOOOOODBBBDOOOAO...",
            "......OOWWWOO..............OOWWWOOOO....",
            "......OOOOOOO..............OOOOOOO......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐶 02/10 Boba Pup (Elder — Fenrir Dire Wolf Guardian)
        // Broad-shouldered Dire Wolf with spiked golden mane, fierce eyes & saber fangs
        PetSpecies::BobaPup => [
            "....OOOO........................OOOO....",
            "...OSSSSOO....................OOSSSSO...",
            "..OSSSSSSBBO...OOOOOOOOOO...OBBSSSSSSO..",
            "..OSSSSSSBBBOOBBLLLLLLLLBBOOBBBSSSSSSO..",
            "...OSSSSBBBBBBBBBBBBBBBBBBBBBBBBSSSSO...",
            "..OO.OOODBBBBREKBBBBBBBBKERBBBBDOOO.OO..",
            ".OGGO..ODBBBBROOBBSSSSBBOORBBBBDO..OGGO.",
            "OGGGGOODBBBBBBWWOOOOOOOOWWBBBBBBDOOGGGGO",
            ".OOGGGGDBBBBBBBOAAWWWWAAOBBBBBBBAGGGGOO.",
            "...OOOODBBBBGGGSSSSSSSSSSGGGbbbbDOOOO...",
            ".....ODBBBBBSSSSSSSSSSSSSSSSBBBBBDO..OO.",
            "....OWWDBBBBSSOSOSOSOSOSOSOSBBBBDBWWOSSO",
            "....OWWODBBBSSSSSSSSSSSSSSSSBBBDOOWWOBSO",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBDO..OBAO.",
            ".......ODBBBBDDDDDDDDDDDDDDBBBBDO.OOAO..",
            "......ODBBBDOOOOOOOOOOOOOOOODBBBDOOOO...",
            ".....OOWWWOO................OOWWWOO.....",
            ".....OOOOOOO................OOOOOOO.....",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐰 03/10 Cloud Bun (Elder — Raijin Storm Valkyrie)
        // Swept-back wing-ears, thunder-cloud shoulder armor, fierce eyes & storm gauntlets
        PetSpecies::CloudBun => [
            "..OOOO............................OOOO..",
            ".OBLLBOO........................OOBLLBO.",
            "OBBPPPBBOO....................OOBBPPPBBO",
            ".OOBBPPPBBBO....OOOOOOOO....OBBBPPPBBOO.",
            "...OOBBBBBBBBOOBBLLLLLLBBOOBBBBBBBBOO...",
            ".....OOOBBBBBBBBBBBBBBBBBBBBBBBBOOO.....",
            "..WWW..ODBBBBREKBBBBBBBBKERBBBBDO..WWW..",
            ".OWWWO.ODBBBBROOBBPPPPBBOORBBBBDO.OWWWO.",
            "OWGGGWOODBBBBBBBOOOOOOOOBBBBBBBDOOWGGGWO",
            "OWWWWWWWDBBBBGGGSSSSSSSSGGGbbbbBDWWWWWWWO",
            ".OWWWWWWDBBBBSSSSSSSSSSSSSSBBBBDBWWWWWO.",
            "..OOOOOODBBBBSSSSSSSSSSSSSSBBBBDOOOOOO..",
            ".......ODBBBBBSSSSSSSSSSSSBBBBBDO.......",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBDO.......",
            "......ODBBBBDDDDDDDDDDDDDDDDBBBBDO......",
            ".....ODBBBDOOOOOOOOOOOOOOOOOODBBBDO.....",
            "....OOWWWOO..................OOWWWOO....",
            "....OOOOOOO..................OOOOOOO....",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        // 🐧 04/10 Star Pengu (Elder — Empoleon Frost Kaiser)
        // Golden trident crown, broad ice-armor wings, razor flippers & glowing visor eyes
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
            "........................................",
            "........................................",
        ],
    }
}

/// Fierce, full-stature **ADULT (`Adult`)** bitmaps (40×24 canvas).
/// Stronger, taller, with spread wings, horns, fangs, and muscular posture!
fn adult_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::IgnisDragon => [
            "........................................",
            "........GG....................GG........",
            ".......OGGO.....OOOOOOOO.....OGGO.......",
            "....OO..OAOOOOOBBLLLLLLBBOOOOAO..OO.....",
            "...OAAO..OOOBBBBBBBBBBBBBBBBOOO.OAAO....",
            "..OABBBO...ODBBEEKKBBBBEEKKBBDO.OBBBAO..",
            ".OABBBBBO..ODBBKKKKBBBBKKKKBBDOOBBBBBAO.",
            ".OADBBBBBOOODBBBBBWAAAWBBBBBDOOBBBBBDAO.",
            "..OADBBBBBBBBBBBSSSSSSSSSSBBBBBBBBBBDAO.",
            "...OOODBBBBBBBBBSSSSSSSSSSBBBBBBBBBDOOO.",
            ".....ODBBBBBBBBBSSSSSSSSSSBBBBBBBBBDO...",
            ".....ODBBBBBBBBBSSSSSSSSSSBBBBBBBBBDO.AA",
            "......ODBBBBBBBBBSSSSSSSSBBBBBBBBBDO.AGA",
            "......ODBBBBBBBBBBBBBBBBBBBBBBBBBBDOOAA.",
            ".......OOODBBBBBDOOOOOOOODBBBBBDOOOOO...",
            ".........ODBBBBDO........ODBBBBDO.......",
            "........OOWWWOO..........OOWWWOO........",
            "........OOOOOOO..........OOOOOOO........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::AzureWyrm => [
            "........................................",
            ".........AA..................AA.........",
            "........OAAO....OOOOOOOO....OAAO........",
            ".........OOAOOOBBLLLLLLBBOOOAOO.........",
            "..........OOOBBBBBBBBBBBBBBOOO..........",
            "....OO...ODBBBEEKKBBBBEEKKBBBDO...OO....",
            "...OAAO..ODBBBKKKKBBBBKKKKBBBDO..OAAO...",
            "..OABBBO.OOODBBBBBBSSSSBBBBBBDOO.OBBBAO.",
            ".OABBBBBBOOOBBBBBWWWWWWWWBBBBBOOOBBBBBAO",
            "..OADBBBBBBBBBBOWWWWWWWWWWOBBBBBBBBBBDAO",
            "...OOODBBBBSSSSOOWWWWWWWWOOSSSSBBBBDOOO.",
            ".....OOOODBBSSSSSBBBBBBBBSSSSSBBDOOOO...",
            "....OABBBBBBBBBSSSSSSSSSSSSBBBBBBBBBAO..",
            "...OADBBBBBDDDDDDDDDDDDDDDDDDDDBBBBBDAO.",
            "....OOOOOOO....................OOOOOOO..",
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
        PetSpecies::EmberFox => [
            "........................................",
            ".......OOOO..................OOOO.......",
            "......OBWWBO................OBWWBO......",
            "......OBWWWBO....OOOOOO....OBWWWBO......",
            ".......OBBBBBOOBBLLLLLLBBOOBBBBBO.......",
            "........OBBBBBBBBBBBBBBBBBBBBBBO........",
            "...AA..ODBBBEEKKBBBBBBBBEEKKBBBDO..AA...",
            "..AWWA.ODBBBKKKKBBBBBBBBKKKKBBBDO.AWWA..",
            "...AA..ODBBBPPPBBBWWWWBBBPPPBBBDO..AA...",
            "........OOODBBBBBBWAAWBBBBBBDOOO..OOOO..",
            "..........OOOOBBBSSSSSSBBBOOOO...OSSSSO.",
            "............ODBBBSSSSSSSSBBBDO..OSSSSWSO",
            "...........ODBBBBSSSSSSSSBBBBDOOOSSSSSO.",
            "..........ODBBBBBBBBBBBBBBBBBBDOOSSSSO..",
            ".........OOODBBBBOOOOOOOOBBBBDOOOOOOO...",
            "...........OOOOOO........OOOOOO.........",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::PhoenixChick => [
            "........................................",
            "...............GG..GG..GG...............",
            "..............OGGAAGGAAGGO..............",
            "....OOO......OBBLLLLLLLLBBO......OOO....",
            "...OAAAO....OBBBBBBBBBBBBBBO....OAAAO...",
            "..OAGGGGOO.OBBBEEKKWWEEKKBBBO.OOGGGGAO..",
            ".OAGBBBBBBOOBBBKKKKGGKKKKBBBOOBBBBBBGAO.",
            ".OABBBBBBBBOODBBBBGGGGBBBBDOOBBBBBBBBAO.",
            "..OADBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBDAO..",
            "...OOODBBBBBBBSSSSSSSSSSSSBBBBBBBDOOO...",
            ".....OOOODBBBBSSSSSSSSSSSSBBBBDOOOO.....",
            "........ODBBBBBSSSSSSSSSSBBBBBDO........",
            "........ODBBBBBBBBBBBBBBBBBBBBDO........",
            ".......OOOGGGGGOOOOOOOOOOGGGGGOOO.......",
            "......OAGAGAGAGAO......OAGAGAGAGAO......",
            ".......OOOOOOOOO........OOOOOOOOO.......",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::CrystalUni => [
            "........................................",
            "..................GGGG..................",
            ".................OGGGGO.................",
            "............OO...OGGGGO...OO............",
            "...........OBBOOOOGGGGOOOBBO............",
            "..........OBLLLLBBBBBBBBLLLLBO..........",
            "....OO...OBBBBBBBBBBBBBBBBBBBBO.........",
            "...OWWO.ODBBEEKKBBBBBBBBEEKKBBDO........",
            "..OWWWWOODBBKKKKBBBBBBBBKKKKBBDO...AA...",
            "..OWWSSWODBBPPPBBBBBBBBBBPPPBBDO..ASSA..",
            "...OOOSSSOOODBBBBBBBBBBBBBBDOOO..OASSA..",
            ".....OOOOOODBBBBSSSSSSSSBBBBDOOOOASSA...",
            ".........OABBBBBSSSSSSSSBBBBBBBOOOAA....",
            ".........OABBBBBBBBBBBBBBBBBBBBBO.......",
            "..........OOODBBBDDDDDDDDDDBBBDOOO......",
            "...........OOOGGGGOOOOOOOOGGGGOOO.......",
            ".............OOOOO........OOOOO.........",
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
            "........................................",
            "........................................",
        ],
        PetSpecies::MochiCat => [
            "........................................",
            ".........OOO................OOO.........",
            "........OBLBO..............OBLBO........",
            ".......OBPPPBO............OBPPPBO.......",
            ".......OBPPPBOOOOOOOOOOOOOBPPPBO........",
            "......OBLLLLBBBBBBBBBBBBBBBBLLLBO.......",
            "......OBBBBBBBBBBBBBBBBBBBBBBBBBO.......",
            ".....ODBBEEKKBBBBBBBBBBBBEEKKBBDO.......",
            ".....ODBBKKKKBBBBBBBBBBBBKKKKBBDO.......",
            ".....ODBBPPPBBBBBBAAABBBBBPPPBBDO.......",
            "......ODBBBBBBBBOOGGGOOBBBBBBBDO........",
            ".......OOOOOOOBBSSSSSSSSBBOOOOO.........",
            ".......ODBBBBSSSSSSSSSSSSSSBBBDO..OO....",
            "......ODBBBBBSSSSSSSSSSSSSSBBBBDOOBA....",
            "......ODBBBBBSSSSSSSSSSSSSSBBBBDOBBA....",
            ".......ODBBBBBBSSSSSSSSSSBBBBBBDOBA.....",
            ".......OODBBBBBBBBBBBBBBBBBBBBDOOO......",
            "........OOODBBBOOOOOOOOOBBBDOOO.........",
            "..........OOOOO........OOOOO............",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "........................................",
        ],
        PetSpecies::BobaPup => [
            "........................................",
            "........OOOOOO............OOOOOO........",
            ".......OSSSSSSOOOOOOOOOOOOSSSSSSO.......",
            "......OSSSSSSLLLBBBBBBBBLLLSSSSSSO......",
            ".....OSSSSSSBBBBBBBBBBBBBBBBSSSSSSO.....",
            ".....OSSSSSBBBBBBBBBBBBBBBBBBSSSSSO.....",
            ".....OSSSSBBEEKKBBBBBBBBEEKKBBSSSSO.....",
            "......OOOOBBKKKKBBBBBBBBKKKKBBOOOO......",
            ".......ODBBBBPPPSSSSSSSSPPPBBBBDO.......",
            ".......ODBBBBSSSSOOOOOOSSSSBBBBDO.......",
            "........OOOOOBBBBOOAAOOBBBBOOOOO........",
            ".......ODBBBBBSSSSSSSSSSSSBBBBBDO.......",
            "......ODBBBBBBSSOSOSOSOSSBBBBBBDO.OO....",
            "......ODBBBBBBSSSSSSSSSSSSBBBBBBDOBS....",
            ".......ODBBBBBBSSOSSOSSOSBBBBBBDOBBA....",
            ".......ODBBBBBBBBBBBBBBBBBBBBBBDOOO.....",
            "........OOODBBBOOOOOOOOOBBBDOOOO........",
            "..........OOOOO........OOOOO............",
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
            ".........OBBBBO..........OBBBBO.........",
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
            "........................................",
            "........................................",
        ],
    }
}

/// Agile intermediate **TEEN (`Teen`)** bitmaps (medium proportions, budding wings/horns).
fn teen_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick => [
            "........................................",
            "........................................",
            "...........AA..............AA...........",
            "..........OAAO...OOOOOO...OAAO..........",
            "...........OOOBBBLLLLLLBBBOOO...........",
            "........OO..OBBBEEKKBBEEKKBBBO..OO......",
            ".......OAAO.OBBBKKKKBBKKKKBBBO.OAAO.....",
            ".......OABBBBBBBBBAAABBBBBBBBBBAO.......",
            "........OADBBBBSSSSSSSSSSBBBBDAO........",
            ".........OOODBBSSSSSSSSSSBBDOOO.........",
            "..........ODBBBSSSSSSSSSSBBBDO..........",
            "..........ODBBBBBBBBBBBBBBBBDO..AA......",
            "...........OOODBBBOOOOBBBDOOOOAA........",
            ".............OOOOO....OOOOO.OO..........",
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
        PetSpecies::AzureWyrm | PetSpecies::CrystalUni => [
            "........................................",
            "........................................",
            ".............AA..........AA.............",
            "............OAOOOOOOOOOOOOAO............",
            "...........OBBLLLLLLLLLLLLBBO...........",
            "..........OBBBEEKKBBBBEEKKBBBO..........",
            "..........OBBBKKKKBBBBKKKKBBBO..........",
            "..........OBBBPPPBBBBBBPPPBBBO..........",
            "...........OOOBBSSSSSSSSBBOOO...........",
            "..........OABBBBSSSSSSSSBBBBAO..........",
            ".........OADBBBBSSSSSSSSBBBBDAO.........",
            "..........OOODBBBBBBBBBBBBDOOO..........",
            "...........OOODBBBOOOOBBBDOOO...........",
            ".............OOOOO....OOOOO.............",
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
        PetSpecies::SlimeKing => [
            "........................................",
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
            "........................................",
        ],
        _ => [
            "........................................",
            "........................................",
            "..........OOOO............OOOO..........",
            ".........OBLPBOOOOOOOOOOOOBPLBO.........",
            "........OBLLLBBBBBBBBBBBBBBLLLBO........",
            "........OBBBEEKKBBBBBBBBEEKKBBBO........",
            "........OBBBKKKKBBBBBBBBKKKKBBBO........",
            "........OBBBPPPBBBAAAABBBPPPBBBO........",
            ".........OOODBBBBSSSSSSBBBBDOOO.........",
            "........OOODBBBBBSSSSSSBBBBBDOOO........",
            "........ODBBBBBBBSSSSSSBBBBBBBDO........",
            ".........ODBBBBBBBBBBBBBBBBBBDO.........",
            "..........OOODBBBOOOOOOBBBDOOO..........",
            "............OOOOO......OOOOO............",
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

/// Super-cute, compact **BABY (`Baby`)** chibi bitmaps (small & adorable!).
fn baby_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
    match species {
        PetSpecies::MochiCat | PetSpecies::EmberFox => [
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "............OOO..........OOO............",
            "...........OBPBOOOOOOOOOOBPBO...........",
            "..........OBLLLBBBBBBBBBBLLLBO..........",
            ".........OBBBEEKKBBBBBBEEKKBBBO.........",
            ".........OBBBKKKKBBBBBBKKKKBBBO.........",
            ".........ODBBPPPBBBBBBBBPPPBBDO.........",
            "..........ODBBBBBSSSSSSBBBBBDO..........",
            "...........OODBBBBBBBBBBBBDOO...........",
            "............OOODBBBOOOOBBDOO............",
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
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::AzureWyrm => [
            "........................................",
            "........................................",
            "........................................",
            ".............AA..........AA.............",
            "............OAAOOOOOOOOOOAAO............",
            "...........OBLLLBBBBBBBBLLLBO...........",
            ".........OOOBBEEKKBBBBEEKKBBOOO.........",
            "........OAAOBBKKKKBBBBKKKKBBOAAO........",
            ".........OOODBBPPPBBBBPPPBBDOOO.........",
            "...........ODBBBBSSSSSSBBBBDO...........",
            "............OODBBBBBBBBBBDOO............",
            ".............OODBBBOOOOBBDO.............",
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
            "........................................",
        ],
        _ => [
            "........................................",
            "........................................",
            "........................................",
            "........................................",
            "............OOOO........OOOO............",
            "...........OSSSSOOOOOOOOSSSSO...........",
            "..........OBLLLBBBBBBBBBBLLLBO..........",
            ".........OBBBEEKKBBBBBBEEKKBBBO.........",
            ".........OBBBKKKKBBBBBBKKKKBBBO.........",
            ".........ODBBPPPBBBBBBBBPPPBBDO.........",
            "..........ODBBBBBSSSSSSBBBBBDO..........",
            "...........OODBBBBBBBBBBBBDOO...........",
            "............OOODBBBOOOOBBDOO............",
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

/// Species-customized 40×24 Egg stage bitmap with rocking animation.
fn egg_bitmap(species: PetSpecies, frame_idx: usize) -> [&'static str; CANVAS_H] {
    let mut base = if frame_idx.is_multiple_of(2) {
        [
            "........................................",
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
            "........................................",
        ]
    } else {
        [
            "........................................",
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
            "........................................",
        ]
    };

    if matches!(
        species,
        PetSpecies::IgnisDragon | PetSpecies::PhoenixChick | PetSpecies::SlimeKing
    ) {
        base[2] = "................GG....GG................";
    } else if matches!(
        species,
        PetSpecies::MochiCat | PetSpecies::CloudBun | PetSpecies::EmberFox
    ) {
        base[2] = "..............OO........OO..............";
    }
    base
}

/// Build the 40×24 character grid for any `(species, stage, mood, frame_idx)`.
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
        EvolutionStage::Adult => adult_bitmap(species),
        EvolutionStage::CyberElder => elder_bitmap(species),
    };

    let mut grid = [['.'; CANVAS_W]; CANVAS_H];
    for (y, row) in raw.iter().enumerate() {
        for (x, ch) in row.chars().take(CANVAS_W).enumerate() {
            grid[y][x] = ch;
        }
    }

    // Blink / sleeping eye transform: ONLY turn `E` and `K` eye pixels into closed eyelids `O`!
    // White body/fangs/claws/pearl pixels (`W`) and fierce red iris (`R`) remain untouched on wake!
    let blink = frame_idx % 2 == 1 || mood == PetMood::Sleeping;
    if stage != EvolutionStage::Egg && blink {
        for row in &mut grid {
            for cell in row.iter_mut() {
                if *cell == 'E' || *cell == 'K' || (mood == PetMood::Sleeping && *cell == 'R') {
                    *cell = 'O';
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

/// Convert a 40×24 pixel grid into 12 half-block terminal [`Line`]s.
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

/// Convert a 40×24 pixel grid into 12 monochrome half-block String lines.
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
        PetMood::Idle => &["·   ✦   ·   ★   ·   ✦   ·   ★   ·", "✦   ·   ★   ·   ✦   ·   ★   ·   ✦"],
        PetMood::Eating => &["♪   ♥   ★   ♥   ♪   ★   ♪   ♥   ♪", "★   ♪   ♥   ♪   ★   ♥   ★   ♪   ★"],
        PetMood::Sleeping => &["☾   z   Z   zZ  ·   ✦   ☾   z   Z", "✦   Z   zZ  z   ☾   ·   ✦   Z   z"],
        PetMood::Happy => &["✦   ♥   ★   ♥   ✦   ♥   ✦   ★   ♥", "♥   ✦   ♥   ✦   ★   ✦   ♥   ✦   ★"],
        PetMood::Sick => &[".   ~   x   ~   .   x   .   ~   x", "~   .   x   .   ~   .   x   .   ~"],
        PetMood::Training => &["★   ♦   ▲   ♦   ★   ▲   ★   ♦   ▲", "♦   ★   ▲   ★   ♦   ★   ♦   ▲   ★"],
    };
    frames[(tick / 4) % frames.len()]
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

/// TrueColor 40×24 Half-Block Pixel Art [`Line`]s for the habitat LCD screen.
pub fn render_colored_sprite(pet: &Pet, tick: usize) -> Vec<Line<'static>> {
    let frame_idx = usize::from(tick % 8 == 7);
    let grid = build_grid(pet.species, pet.stage, pet.mood, frame_idx);
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

/// TrueColor 40×24 Half-Block Pixel Art [`Line`]s for the Step 1 Companion Selector.
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

/// A friendly preview of a species (adult form, idle) as plain strings.
pub fn render_species_preview(species: PetSpecies) -> Vec<String> {
    let frames = get_sprite_frames(species, EvolutionStage::Adult, PetMood::Idle);
    frames[0].lines().map(|l| l.to_string()).collect()
}

/// What your companion is saying out loud right now (`💬`).
/// Rotates slowly every 48 ticks (~12 seconds) so you can easily read every word!
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
    chatter[(tick / 48) % chatter.len()].clone()
}

/// Inner thought bubble (`💭`) revealing what your companion is secretly thinking!
/// Rotates slowly every 48 ticks (~12 seconds) so you have plenty of time to read it.
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
            "💭 (Thinking: Soon I'll awaken my Byakko Tiger Shogun form!)",
            "💭 (Thinking: My keeper's keyboard clicks sound like a lullaby!)",
        ],
        PetSpecies::BobaPup => [
            "💭 (Thinking: Is it playtime yet?! My tail won't stop wagging!)",
            "💭 (Thinking: One day I'll grow into a mighty Fenrir Dire Wolf!)",
            "💭 (Thinking: I'll guard the Sanctuary while my keeper works!)",
        ],
        PetSpecies::CloudBun => [
            "💭 (Thinking: My ears feel extra fluffy today~)",
            "💭 (Thinking: In my Elder form, I'll command the storm clouds!)",
            "💭 (Thinking: Press [Y] to chat with me anytime!)",
        ],
        PetSpecies::StarPengu => [
            "💭 (Thinking: I counted 42 shooting stars in the aurora last night!)",
            "💭 (Thinking: Training hard to earn my Frost Kaiser trident crown!)",
            "💭 (Thinking: Snowberries taste best when shared with a friend!)",
        ],
        PetSpecies::IgnisDragon => [
            "💭 (Thinking: My flame breath is getting hotter! Soon I'll be Bahamut!)",
            "💭 (Thinking: Every true dragon hoards shiny coins and spicy chilies!)",
            "💭 (Thinking: I love when my keeper polishes my golden horns.)",
        ],
        PetSpecies::AzureWyrm => [
            "💭 (Thinking: The tides and stars are awakening my Leviathan power...)",
            "💭 (Thinking: My moon pearl glows brighter whenever you visit!)",
            "💭 (Thinking: Ancient dragons value patience, wisdom, and snacks.)",
        ],
        PetSpecies::EmberFox => [
            "💭 (Thinking: Soon I'll unleash all nine tails of cyan foxfire!)",
            "💭 (Thinking: Inari tofu is the secret to Kyuubi spirit power~)",
            "💭 (Thinking: Should I play a tiny harmless prank, or behave?)",
        ],
        PetSpecies::PhoenixChick => [
            "💭 (Thinking: My wings burn like sunrise — Solar Garuda awaits!)",
            "💭 (Thinking: Even a legendary solar firebird loves headpats.)",
            "💭 (Thinking: Let's set a new high score in Hurdle Dash [G]!)",
        ],
        PetSpecies::CrystalUni => [
            "💭 (Thinking: My prism horn is evolving into a Celestial Kirin lance!)",
            "💭 (Thinking: Starlight sugar makes my hooves sparkle~)",
            "💭 (Thinking: Our bond feels stronger every single day!)",
        ],
        PetSpecies::SlimeKing => [
            "💭 (Thinking: Soon I'll grow into a colossal Titan Slime Emperor!)",
            "💭 (Thinking: Make sure my golden crown isn't tilted!)",
            "💭 (Thinking: Crystal dew keeps my royal gel 100% powerful!)",
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
    fn test_all_species_stages_and_moods_render_40x24_half_block_pixel_sprites() {
        for species in PetSpecies::all() {
            let baby = get_sprite_frames(*species, EvolutionStage::Baby, PetMood::Idle);
            let adult = get_sprite_frames(*species, EvolutionStage::Adult, PetMood::Idle);
            let elder = get_sprite_frames(*species, EvolutionStage::CyberElder, PetMood::Idle);
            assert_ne!(
                baby[0], adult[0],
                "{species:?} Baby and Adult must have distinct silhouettes"
            );
            assert_ne!(
                adult[0], elder[0],
                "{species:?} Adult and Apex Elder must have distinct silhouettes"
            );

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
    fn test_render_animated_colored_sprite_and_slow_readable_thoughts() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let colored = render_colored_sprite(&pet, 0);
        assert!(!colored.is_empty());

        pet.stage = EvolutionStage::Adult;
        // Verify thought & speech stay identical across 30 consecutive ticks (~7.5s) so user can read comfortably!
        assert_eq!(thought_bubble(&pet, 0), thought_bubble(&pet, 30));
        assert_eq!(speech_bubble(&pet, 0), speech_bubble(&pet, 30));

        pet.poop_count = 2;
        let dirty_lines = render_animated_sprite(&pet, 0);
        assert!(dirty_lines.iter().any(|l| l.contains('💩')));
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
            assert_ne!(get_mood_particles(*mood, 0), get_mood_particles(*mood, 4));
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
