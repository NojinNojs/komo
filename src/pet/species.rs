use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PetSpecies {
    #[default]
    MochiCat,
    BobaPup,
    CloudBun,
    StarPengu,
    IgnisDragon,
    AzureWyrm,
    EmberFox,
    PhoenixChick,
    CrystalUni,
    SlimeKing,
}

impl PetSpecies {
    pub fn all() -> &'static [PetSpecies] {
        &[
            Self::MochiCat,
            Self::BobaPup,
            Self::CloudBun,
            Self::StarPengu,
            Self::IgnisDragon,
            Self::AzureWyrm,
            Self::EmberFox,
            Self::PhoenixChick,
            Self::CrystalUni,
            Self::SlimeKing,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "🐱 Neko Mochi",
            Self::BobaPup => "🐶 Boba Pup",
            Self::CloudBun => "🐰 Cloud Bun",
            Self::StarPengu => "🐧 Star Pengu",
            Self::IgnisDragon => "🐉 Ignis Drake",
            Self::AzureWyrm => "🐲 Azure Wyrm",
            Self::EmberFox => "🦊 Kitsune Nine",
            Self::PhoenixChick => "🦅 Sol Phoenix",
            Self::CrystalUni => "🦄 Astra Unicorn",
            Self::SlimeKing => "👑 Royal Jelly",
        }
    }

    pub fn family(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun | Self::StarPengu => "Cozy Classics",
            _ => "Mythical & Fantasy",
        }
    }

    pub fn short_family(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun | Self::StarPengu => "Cozy",
            _ => "Mythical",
        }
    }

    pub fn rarity(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun => "Common ★",
            Self::StarPengu | Self::SlimeKing => "Rare ★★",
            Self::EmberFox | Self::CrystalUni => "Epic ★★★",
            Self::AzureWyrm => "Mythic ★★★★",
            Self::IgnisDragon | Self::PhoenixChick => "Legendary ★★★★★",
        }
    }

    pub fn star_bar(&self) -> &'static str {
        match self {
            Self::MochiCat | Self::BobaPup | Self::CloudBun => "★☆☆☆☆",
            Self::StarPengu | Self::SlimeKing => "★★☆☆☆",
            Self::EmberFox | Self::CrystalUni => "★★★☆☆",
            Self::AzureWyrm => "★★★★☆",
            Self::IgnisDragon | Self::PhoenixChick => "★★★★★",
        }
    }

    pub fn element_badge(&self) -> &'static str {
        match self {
            Self::MochiCat => "🍡 Sweet",
            Self::BobaPup => "🧋 Tapioca",
            Self::CloudBun => "🌈 Sky",
            Self::StarPengu => "🌌 Aurora",
            Self::IgnisDragon => "🔥 Dragonfire",
            Self::AzureWyrm => "🌊 Celestial Tide",
            Self::EmberFox => "🦊 Spirit Foxfire",
            Self::PhoenixChick => "🌞 Sunburst",
            Self::CrystalUni => "🌟 Starlight",
            Self::SlimeKing => "💧 Mystic Slime",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::MochiCat => {
                "A squishy sweet rice-cake kitty who awakens into Byakko Tiger Shogun."
            }
            Self::BobaPup => "A loyal milk-tea puppy who grows into the mighty Fenrir Dire Wolf.",
            Self::CloudBun => {
                "A cotton-soft sky bunny who commands storm clouds as Raijin Valkyrie."
            }
            Self::StarPengu => {
                "A cosmic snow penguin who earns the golden trident of Frost Kaiser."
            }
            Self::IgnisDragon => {
                "A crimson wyvern whelp who evolves into Inferno Bahamut Sovereign."
            }
            Self::AzureWyrm => "An ancient sea-and-sky dragon guarding a luminous tidal pearl.",
            Self::EmberFox => "A shrine fox wreathed in nine tails of dancing cyan foxfire.",
            Self::PhoenixChick => "An immortal solar firebird whose wings blaze like Garuda.",
            Self::CrystalUni => "A prism-horned celestial foal evolving into Kirin Pegasus.",
            Self::SlimeKing => "A royal gelatinous monarch who grows into Titan Slime Emperor.",
        }
    }

    pub fn default_name(&self) -> &'static str {
        match self {
            Self::MochiCat => "Mochi",
            Self::BobaPup => "Boba",
            Self::CloudBun => "Cloudy",
            Self::StarPengu => "Pebble",
            Self::IgnisDragon => "Ignis",
            Self::AzureWyrm => "Ao",
            Self::EmberFox => "Kitsune",
            Self::PhoenixChick => "Sol",
            Self::CrystalUni => "Astra",
            Self::SlimeKing => "Puni",
        }
    }

    pub fn favorite_snack(&self) -> &'static str {
        match self {
            Self::MochiCat => "Taiyaki Fish 🐟",
            Self::BobaPup => "Honey Biscuit 🦴",
            Self::CloudBun => "Sweet Carrot 🥕",
            Self::StarPengu => "Snowberry 🫐",
            Self::IgnisDragon => "Flame Chili 🔥",
            Self::AzureWyrm => "Moon Pearl 🫧",
            Self::EmberFox => "Inari Tofu 🦊",
            Self::PhoenixChick => "Sunfire Berry 🌞",
            Self::CrystalUni => "Starlight Sugar 🌟",
            Self::SlimeKing => "Crystal Dew 💧",
        }
    }

    pub fn is_mythical(&self) -> bool {
        self.family() == "Mythical & Fantasy"
    }
}
