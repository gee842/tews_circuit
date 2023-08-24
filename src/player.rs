use std::fmt::Display;

use poise::serenity_prelude::User;

use super::*;

pub struct Player {
    user: User,
    pub rank: rank::Rank,
    pub points: u16,
}

impl Player {
    pub async fn new(user: User, points: u16) -> Self {
        let rank = rank::Rank::from(points);
        Self { user, rank, points }
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn name(&self) -> String {
        self.user.name.clone()
    }

    pub fn add(&mut self, points: u16) -> u16 {
        self.points += points;
        self.points
    }

    pub fn minus(&mut self, points: u16) -> u16 {
        self.points -= points;
        self.points
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.user.id;
        let name = &self.user.name;

        write!(
            f,
            "Name: {} ({})\nRank: {}\nPoints: {}",
            name, id, self.rank, self.points
        )
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.user.id == other.user.id
    }
}
