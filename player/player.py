from .rank import Rank, determine_rank


class Player:
    def __init__(self, uid: int, points: int, streak: int):
        self.uid = uid
        self.points = points
        self.streak = streak
        self.rank = determine_rank(points)

    def changed(self, points: int):
        other = determine_rank(points)
        if self.rank < other:
            return f"Promotion: {other.name} -> {self.rank.name}"
        elif self.rank > other:
            return f"Demotion:  {other.name} -> {self.rank.name}."
        else:
            return """"""

    def calculate_points(self, win: bool, other_rank: Rank):
        # for ppl not in the same group basic points are:
        # higher than u +25 and -15
        # lower than u +10 and -30

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
        else:
            streak_bonus = 10

        self.points += streak_bonus

        if self.points <= 750:
            self.points = 750

        return self.points
