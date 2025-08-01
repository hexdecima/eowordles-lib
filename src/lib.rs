use std::{cmp::Ordering, fmt::Display};
use serde::{Deserialize, Serialize};

pub fn hello() -> String {
    String::from("meow")
}

// TODO: move data to a rust file to make it nyoom fast.
pub const ENEMIES: &str = include_str!("./enemies");

pub fn list_enemies() -> Box<[Enemy]> {
    ENEMIES.lines().skip(1).map(Enemy::from).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Environment {
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
    Dungeon,
    Event,
    Day,
    Night,
    Graveyard,
    Goblin,
    Pirate,
    Rain,
    Martian,
    Eclipse,
    OldOnes,
    Blood,
    Lunar,
    Unknown,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Environment::*;

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
            Dungeon => "Dungeon",
            Event => "Event",
            Day => "Day",
            Night => "Night",
            Graveyard => "Graveyard",
            Goblin => "Goblin Army",
            Pirate => "Pirate Invasion",
            Rain => "Rain",
            Martian => "Martian Madness",
            Eclipse => "Solar Eclipse",
            OldOnes => "Old One's Army",
            Blood => "Blood Moon",
            Lunar => "Lunar Events",
            Unknown => "Unknown",
        };

        write!(f, "{text}")
    }
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        use Environment::*;

        match value {
            "Any" => Any,
            "Forest" => Forest,
            "Snow" => Snow,
            "Jungle" => Jungle,
            "Desert" => Desert,
            "Ocean" => Ocean,
            "Corruption" => Corruption,
            "Crimson" => Crimson,
            "Hallow" => Hallow,
            "Mushroom" => Mushroom,
            "Dungeon" => Dungeon,
            "Graveyard" => Graveyard,
            "Day" => Day,
            "Goblin" => Goblin,
            "Pirate" => Pirate,
            "Night" => Night,
            "Event" => Event,
            "OldOnes" => OldOnes,
            "Blood" => Blood,
            "Martian" => Martian,
            "Eclipse" => Eclipse,
            "Lunar" => Lunar,
            "Rain" => Rain,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EnvironmentDiff {
    pub right: Vec<Environment>,
    pub wrong: Vec<Environment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Layer {
    Any,
    Space,
    Surface,
    Underground,
    Caverns,
    Underworld,
    Unknown,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Layer::*;

        let text = match self {
            Any => "Any",
            Space => "Space",
            Surface => "Surface",
            Underground => "Underground",
            Caverns => "Caverns",
            Underworld => "Underworld",
            Unknown => "Unknown",
        };

        write!(f, "{text}")
    }
}

impl From<&str> for Layer {
    fn from(value: &str) -> Layer {
        use Layer::*;

        match value {
            "Any" => Any,
            "Space" => Space,
            "Surface" => Surface,
            "Underground" => Underground,
            "Caverns" => Caverns,
            "Underworld" => Underworld,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LayerDiff {
    pub right: Vec<Layer>,
    pub wrong: Vec<Layer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Coins {
    pub gold: u8,
    pub silver: u8,
    pub copper: u8,
}

impl Coins {
    pub fn new<T: AsRef<str>>(g: T, s: T, c: T) -> Self {
        Self {
            gold: g.as_ref().parse::<_>().unwrap(),
            silver: s.as_ref().parse::<_>().unwrap(),
            copper: c.as_ref().parse::<_>().unwrap(),
        }
    }
    pub fn as_copper(&self) -> usize {
        (self.copper as usize
            + (self.silver as usize * 100usize)
            + (self.gold as usize * 1000usize)) as usize
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
    Unknown,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Unknown => "Unknown",
        };

        write!(f, "{text}")
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
                Unknown => Greater,
            },
            Unknown => match other {
                _ => Less,
            },
        }
    }
}

impl From<&str> for Rarity {
    fn from(value: &str) -> Rarity {
        use Rarity::*;

        match value {
            "Common" => Common,
            "Uncommon" => Uncommon,
            "Rare" => Rare,
            _ => Unknown,
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
    pub environments: Vec<Environment>,
    pub layers: Vec<Layer>,
    pub rarity: Rarity,
}
impl Enemy {
    pub fn diff(&self, other: &Enemy) -> EnemyDiff {
        let name = self.name == other.name;
        let life = self.life.cmp(&other.life).into();
        let defence = self.defence.cmp(&other.defence).into();
        let coins = self.coins.as_copper().cmp(&other.coins.as_copper()).into();
        let environments = self.diff_env(&other.environments);
        let layers = self.diff_layer(&other.layers);
        let rarity = self.rarity.cmp(&other.rarity).into();

        EnemyDiff {
            name,
            life,
            defence,
            coins,
            environments,
            layers,
            rarity,
        }
    }
    pub fn diff_env(&self, other: &[Environment]) -> EnvironmentDiff {
        let (right, wrong): (Vec<Environment>, Vec<Environment>) = self
            .environments
            .iter()
            .cloned()
            .partition(|env| other.contains(env));
        EnvironmentDiff { right, wrong }
    }
    pub fn diff_layer(&self, other: &[Layer]) -> LayerDiff {
        let (right, wrong): (Vec<Layer>, Vec<Layer>) = self
            .layers
            .iter()
            .cloned()
            .partition(|lay| other.contains(lay));
        LayerDiff { right, wrong }
    }
}

impl From<&str> for Enemy {
    fn from(value: &str) -> Self {
        let mut chunks = value.split(',');
        let id = chunks.next().unwrap();
        let name = chunks.next().unwrap();
        let life = chunks.next().unwrap();
        let defence = chunks.next().unwrap();
        let g = chunks.next().unwrap();
        let s = chunks.next().unwrap();
        let c = chunks.next().unwrap();
        let environment = chunks.next().unwrap();
        let layer = chunks.next().unwrap();
        let rarity = chunks.next().expect(&format!("{id}"));

        Self {
            id: id.parse::<_>().unwrap(),
            name: name.to_owned(),
            life: life.parse::<_>().unwrap(),
            defence: defence.parse::<_>().unwrap(),
            coins: Coins::new(g, s, c),
            environments: environment.split('/').map(Environment::from).collect(),
            layers: layer.split('/').map(Layer::from).collect(),
            rarity: rarity.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EnemyDiff {
    pub name: bool,
    pub life: OrderingText,
    pub defence: OrderingText,
    pub coins: OrderingText,
    pub environments: EnvironmentDiff,
    pub layers: LayerDiff,
    pub rarity: OrderingText,
}

impl EnemyDiff {
    /// Whether or not this diff was the result of two of the same enemy.
    pub fn is_same(&self) -> bool {
        self.name
            && self.life.is_eq()
            && self.defence.is_eq()
            && self.coins.is_eq()
            && self.environments.wrong.is_empty()
            && self.layers.wrong.is_empty()
    }
}
