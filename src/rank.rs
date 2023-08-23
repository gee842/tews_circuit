use std::fmt::Display;

#[derive(PartialEq, PartialOrd)]
pub enum Rank {
    Unrated(u16),
    Gold(u16),
    Emerald(u16),
    Diamond(u16),
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Unrated(value) => write!(f, "Unrated: {}", value),
            Rank::Gold(value) => write!(f, "Gold: {}", value),
            Rank::Emerald(value) => write!(f, "Emerald: {}", value),
            Rank::Diamond(value) => write!(f, "Diamond: {}", value),
        }
    }
}

impl From<u16> for Rank {
    fn from(value: u16) -> Self {
        if value >= 1500 {
            Rank::Diamond(value)
        } else if value >= 1300 {
            Rank::Emerald(value)
        } else if value >= 1100 {
            Rank::Gold(value)
        } else {
            Rank::Unrated(value)
        }
    }
}
