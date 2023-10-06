use std::fmt::Display;

use super::*;
use crate::db::Database;

use poise::serenity_prelude::User;
use sqlx::Error as SqlxError;
use tracing::info;

pub enum Streak {
    Amount(u16),
    Neither,
}

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

    pub async fn streak_bonus(&mut self, db: &Database) -> Result<u16, SqlxError> {
        let streak = match db.streak_info(self).await? {
            Streak::Amount(amt) => amt,
            Streak::Neither => return Ok(0),
        };

        let streak = if streak == 2 { 5 } else { 10 };

        Ok(streak)
    }

    // OK OK OK OK
    // OK OK OK OK
    // OK OK OK OK
    pub async fn add(&mut self, points: u16, db: &Database) -> Result<u16, SqlxError> {
        let streak = self.streak_bonus(db).await?;
        self.points += points + streak;

        self.update_rank(self.points, db).await?;

        Ok(self.points)
    }

    pub async fn minus(&mut self, points: u16, db: &Database) -> Result<u16, SqlxError> {
        if self.points <= 750 {
            self.points = 750;
        } else {
            let streak = self.streak_bonus(db).await?;
            self.points -= points + streak;
        }

        self.update_rank(self.points, db).await?;
        Ok(self.points)
    }

    async fn update_rank(&mut self, points: u16, db: &Database) -> Result<(), SqlxError> {
        let new_rank = Rank::from(points);
        if self.rank == new_rank {
            return Ok(());
        }

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
