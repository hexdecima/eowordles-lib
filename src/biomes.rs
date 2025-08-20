use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Biome {
    Any,
    Forest,
    Snow,
    Jungle,
    Desert,
    Ocean,
    Corruption,
    Crimson,
    Hallow,
    Mushroom,
    Graveyard,
    Dungeon,
    Margranite, // Granite or Marble
    Day,
    Night
}

impl Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Biome::*;

        let text = match self {
            Any => "Any",
            Forest => "Forest",
            Snow => "Snow",
            Jungle => "Jungle",
            Desert => "Desert",
            Ocean => "Ocean",
            Corruption => "Corruption",
            Crimson => "Crimson",
            Hallow => "Hallow",
            Mushroom => "Glowing Mushrooms",
            Graveyard => "Graveyard",
            Dungeon => "Dungeon",
            Margranite => "Marble or Granite",
            Day => "Day",
            Night => "Night",
        };

        write!(f, "{text}")
    }
}
