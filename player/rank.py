from enum import Enum


class Rank(Enum):
    Diamond = 1
    Emerald = 2
    Gold = 3
    Unrated = 4

    def __repr__(self):
        return f"{self._name_}"

    def __ge__(self, other):
        if self.__class__ is other.__class__:
            return self.value >= other.value
        return NotImplemented

    def __gt__(self, other):
        if self.__class__ is other.__class__:
            return self.value > other.value
        return NotImplemented

    def __le__(self, other):
        if self.__class__ is other.__class__:
            return self.value <= other.value
        return NotImplemented

    def __lt__(self, other):
        if self.__class__ is other.__class__:
            return self.value < other.value
        return NotImplemented


def determine_rank(points: int) -> Rank:
    if points >= 1500:
        return Rank.Diamond
    elif points >= 1300:
        return Rank.Emerald
    elif points >= 1100:
        return Rank.Gold
    else:
        return Rank.Unrated
