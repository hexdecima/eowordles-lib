use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::Diff;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Layer {
    Any,
    Surface,
    Underground,
    Caverns,
    Underworld,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Layer::*;

        let text = match self {
            Any => "Any",
            Surface => "Surface",
            Underground => "Underground",
            Caverns => "Caverns",
            Underworld => "Underworld",
        };

        write!(f, "{text}")
    }
}

pub type LayerDiff = Diff<Layer>;
