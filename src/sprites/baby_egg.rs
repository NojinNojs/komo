//! Sprite submodule.
use super::*;

/// Cute Chibi **BABY (`Baby`)** bitmaps (40×22).
pub(super) fn baby_bitmap(species: PetSpecies) -> [&'static str; CANVAS_H] {
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
pub(super) fn egg_bitmap(species: PetSpecies, frame_idx: usize) -> [&'static str; CANVAS_H] {
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
pub(super) fn ghost_bitmap() -> [&'static str; CANVAS_H] {
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
