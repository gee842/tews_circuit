use std::fmt::Display;

use super::*;
use crate::db::Database;

use poise::serenity_prelude::User;
use sqlx::Error as SqlxError;

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

    pub async fn add(&mut self, points: u16, db: &Database) -> Result<u16, SqlxError> {
        self.points += points;
        self.update_rank(points, db).await?;

        Ok(self.points)
    }

    pub async fn minus(&mut self, points: u16, db: &Database) -> Result<u16, SqlxError> {
        self.points -= points;
        self.update_rank(points, db).await?;

        Ok(self.points)
    }

    async fn update_rank(&mut self, points: u16, db: &Database) -> Result<(), SqlxError> {
        let new_rank = Rank::from(points);
        if self.rank != new_rank {
            return Ok(());
        }

        self.rank = new_rank;
        db.update_rank(self).await?;
        Ok(())
    }

    pub fn id(&self) -> f64 {
        self.user().id.0 as f64
    }

    pub async fn mark_loss(&self, db: &Database) -> Result<(), SqlxError> {
        db.mark_loss(self).await?;
        Ok(())
    }

    pub async fn mark_win(&self, db: &Database) -> Result<(), SqlxError> {
        db.mark_win(self).await?;

        Ok(())
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
