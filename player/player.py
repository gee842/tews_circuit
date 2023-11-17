from .rank import Rank, determine_rank


class Player:
    """
    Represents a player. Mainly used for point calculation and for pretty printing
    player information to the user.
    """

    def __init__(self, uid: int, points: int, streak: int):
        self.uid = uid
        self.points = points
        self.streak = streak
        self.rank = determine_rank(points)

    def __repr__(self):
        return f"{self.uid}: {self.points} ({self.rank.name})"

    def has_rank_changed(self, points: int) -> str:
        """
        keyword arguments:
        points -- The player's points BEFORE point calculation.
        """

        # self.rank is the player's CURRENT rank.
        other = determine_rank(points)
        msg = f"{other.name} -> {self.rank.name}."
        if other < self.rank:
            return f"Promotion: {msg}"
        elif other > self.rank:
            return f"Demotion:  {msg}"
        else:
            return """"""

    def calculate_points(self, win: bool, other_rank: Rank):
        rank_bonus = 25
        if win:
            if self.rank > other_rank:
                rank_bonus = 10
            elif self.rank < other_rank:
                rank_bonus = 30

            self.points += rank_bonus
        else:
            if self.rank > other_rank:
                rank_bonus = 30
            elif self.rank < other_rank:
                rank_bonus = 15

            self.points -= rank_bonus

        streak_bonus = 0
        if self.streak == 1:
            streak_bonus = 5
        elif self.streak >= 2:
            streak_bonus = 10

        self.points += streak_bonus

        if self.points <= 750:
            self.points = 750

        self.rank = determine_rank(self.points)
        return self.points
