from .rank import Rank

class Player:
    def __init__(self, uid: int, points: int):
        self.uid = uid

        if points >= 1500:
            self.rank = Rank.Diamond
        elif points >= 1300:
            self.rank = Rank.Emerald
        elif points >= 1100:
            self.rank = Rank.Gold
        else:
            self.rank = Rank.Unrated

    def changed(self, points: int):
        other = Player(0, points).rank
        if self.rank < other:
            return f"Promotion: {self} to {other}"
        elif self.rank > other:
            return f"Demotion: {self} -> {other}."
        else:
            return """"""
