use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Event {
    Goblin,
    Pirate,
    Rain,
    Slime,
    Wind,
    Martian,
    Eclipse,
    OldOnes,
    Blood,
    Lunar,
    Solar,
    Stardust,
    Vortex,
    Nebula,
    Legion,
    Frost,
    Pumpkin,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Event::*;

        let text = match self {
            Goblin => "Goblin Invasion",
            Pirate => "Pirate Invasion",
            Rain => "Rain",
            Slime => "Slime Rain",
            Wind => "Windy Day",
            Martian => "Martian Invasion",
            Eclipse => "Solar Eclipse",
            OldOnes => "Old One's Army",
            Blood => "Blood Moon",
            Lunar => "Lunar Events",
            Solar => "Solar Pillar",
            Stardust => "Stardust Pillar",
            Vortex => "Vortex Pillar",
            Nebula => "Nebula Pillar",
            Legion => "Frost Legion",
            Frost => "Frost Moon",
            Pumpkin => "Pumpkin Moon",
        };

        write!(f, "{text}")
    }
}
