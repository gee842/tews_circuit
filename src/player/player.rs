use std::fmt::Display;

use super::*;
use crate::db::Database;

use poise::serenity_prelude::User;
use sqlx::Error as SqlxError;
use tracing::info;

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
        self.update_rank(self.points, db).await?;

        Ok(self.points)
    }

    pub async fn streak_info(&mut self, db: &Database) -> Result<(bool, u8), SqlxError> {
        let (streak, times) = db.streak_info(self).await?;
        if streak {
            Ok((true, times))
        } else {
            Ok((false, times))
        }
    }

    pub async fn minus(&mut self, points: u16, db: &Database) -> Result<u16, SqlxError> {
        if self.points <= 750 {
            self.points = 750;
        } else {
            self.points -= points;
        }

        self.update_rank(self.points, db).await?;
        Ok(self.points)
    }

    async fn update_rank(&mut self, points: u16, db: &Database) -> Result<(), SqlxError> {
        let new_rank = Rank::from(points);
        if self.rank == new_rank {
            info!("{}'s rank remains unchanged.", self.user.name);
            return Ok(());
        }

        info!("{}'s rank remains changed.", self.user.name);
        self.rank = new_rank;
        db.update_rank(self).await?;
        Ok(())
    }

    pub fn id(&self) -> String {
        self.user.id.0.to_string()
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
