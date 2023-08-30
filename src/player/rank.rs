use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Rank {
    Unrated = 0,
    Gold = 1,
    Emerald = 2,
    Diamond = 3,
}

impl From<u16> for Rank {
    fn from(value: u16) -> Self {
        if value >= 1500 {
            Rank::Diamond
        } else if value >= 1300 {
            Rank::Emerald
        } else if value >= 1100 {
            Rank::Gold
        } else {
            Rank::Unrated
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Unrated => write!(f, "Unrated"),
            Rank::Gold => write!(f, "Gold"),
            Rank::Emerald => write!(f, "Emerald"),
            Rank::Diamond => write!(f, "Diamond"),
        }
    }
}

impl Rank {
    pub fn current_status(&self, new: &Self) -> String {
        if self < new {
            format!("Promotion: {self} -> {new}.")
        } else if self > &new {
            format!("Demotion: {self} -> {new}.")
        } else {
            "".to_string()
        }
        .to_string()
    }
}
