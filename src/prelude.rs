use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{biomes::Biome, events::Event, layers::Layer};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diff<T: Clone + PartialEq + Eq> {
    pub right: Box<[T]>,
    pub wrong: Box<[T]>,
    pub missing: bool,
}

impl<T: Clone + PartialEq + Eq> Diff<T> {
    pub fn with(lhs: &[T], rhs: &[T]) -> Self {
        let (right, wrong): (Vec<T>, Vec<T>) =
            lhs.iter().cloned().partition(|item| rhs.contains(item));
        let missing = !wrong.is_empty() || rhs.len() != lhs.len();

        Self {
            right: right.into_boxed_slice(),
            wrong: wrong.into_boxed_slice(),
            missing,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Coins {
    pub gold: u8,
    pub silver: u8,
    pub copper: u8,
}

impl Coins {
    pub fn new(gold: u8, silver: u8, copper: u8) -> Self {
        Self {
            gold,
            silver,
            copper,
        }
    }
    pub fn as_copper(&self) -> usize {
        (self.copper as usize
            + (self.silver as usize * 100usize)
            + (self.gold as usize * 10000usize)) as usize
    }
}

impl Display for Coins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.as_copper() == 0 {
            return write!(f, "Nothing");
        }

        let gold = if self.gold > 0 {
            format!("{} gold", self.gold)
        } else {
            String::new()
        };
        let silver = if self.silver > 0 {
            format!(" {} silver", self.silver)
        } else {
            String::new()
        };
        let copper = if self.copper > 0 {
            format!(" {} copper", self.copper)
        } else {
            String::new()
        };

        write!(f, "{gold}{silver}{copper}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Ord for Rarity {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Rarity::*;

        match self {
            Common => match other {
                Common => Equal,
                _ => Greater,
            },
            Uncommon => match other {
                Common => Less,
                Uncommon => Equal,
                _ => Greater,
            },
            Rare => match other {
                Common | Uncommon => Less,
                Rare => Equal,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OrderingText {
    Less,
    Equal,
    Greater,
}

impl OrderingText {
    pub fn is_eq(&self) -> bool {
        *self == OrderingText::Equal
    }
}

impl From<Ordering> for OrderingText {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => Self::Less,
            Ordering::Equal => Self::Equal,
            Ordering::Greater => Self::Greater,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Enemy {
    pub id: u16,
    pub name: String,
    pub life: u16,
    pub defence: u16,
    pub coins: Coins,
    pub biomes: Box<[Biome]>,
    pub events: Box<[Event]>,
    pub layers: Vec<Layer>,
    pub rarity: Rarity,
}
impl Enemy {
    pub fn new(
        id: u16,
        name: impl AsRef<str>,
        life: u16,
        defence: u16,
        coins: Coins,
        biomes: impl IntoIterator<Item = Biome>,
        events: impl IntoIterator<Item = Event>,
        layers: impl IntoIterator<Item = Layer>,
        rarity: Rarity,
    ) -> Self {
        let name = name.as_ref().to_string();
        let biomes = biomes.into_iter().collect();
        let events = events.into_iter().collect();
        let layers = layers.into_iter().collect();

        Self {
            id,
            name,
            life,
            defence,
            coins,
            biomes,
            events,
            layers,
            rarity,
        }
    }
    pub fn diff(&self, other: &Enemy) -> EnemyDiff {
        let name = self.name == other.name;
        let life = self.life.cmp(&other.life).into();
        let defence = self.defence.cmp(&other.defence).into();
        let coins = self.coins.as_copper().cmp(&other.coins.as_copper()).into();
        let biomes = Diff::with(&*self.biomes, &*other.biomes);
        let events = Diff::with(&*self.events, &*other.events);
        let layers = Diff::with(&*self.layers, &*other.layers);
        let rarity = self.rarity.cmp(&other.rarity).into();

        EnemyDiff {
            name,
            life,
            defence,
            coins,
            biomes,
            events,
            layers,
            rarity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EnemyDiff {
    pub name: bool,
    pub life: OrderingText,
    pub defence: OrderingText,
    pub coins: OrderingText,
    pub biomes: Diff<Biome>,
    pub events: Diff<Event>,
    pub layers: Diff<Layer>,
    pub rarity: OrderingText,
}

impl EnemyDiff {
    /// Whether or not this diff was the result of two of the same enemy.
    pub fn is_same(&self) -> bool {
        self.name
            && self.life.is_eq()
            && self.defence.is_eq()
            && self.coins.is_eq()
            && self.biomes.wrong.is_empty()
            && self.events.wrong.is_empty()
            && self.layers.wrong.is_empty()
    }
}
