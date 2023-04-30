# %%

class Player:
    def __init__(self, name):
        self.name = name
        self.points = 900
        self.win_streak = 0
        self.lose_streak = 0
        self.group = 'unrated'

class Circuit:
    def __init__(self):
        self.players = {}
        self.groups = {'unrated': [], 'gold': [], 'emerald': [], 'diamond': []}
        
    def add_player(self, name):
        self.players[name] = Player(name)
        self.groups['unrated'].append(name)
        self.update_groups(self.players[name])

    def inter_group_points(self, winner, loser):
        win_bonus = 25
        lose_penalty = -25
        streak_bonus = 5
        higher_group_win_bonus = 0
        lower_group_lose_penalty = 5

        winner_streak = self.players[winner].streak
        loser_streak = self.players[loser].streak

        if self.players[winner].group != self.players[loser].group:
            if self.players[winner].group > self.players[loser].group:
                win_bonus += higher_group_win_bonus
                lose_penalty += lower_group_lose_penalty
            else:
                win_bonus -= higher_group_win_bonus
                lose_penalty -= lower_group_lose_penalty

            if winner_streak > 0 and winner_streak < 3:
                win_bonus += streak_bonus
            if loser_streak < 0 and loser_streak > -3:
                lose_penalty += streak_bonus

        self.players[winner].points += win_bonus
        self.players[loser].points += lose_penalty

        if self.players[loser].points < 750:
            self.players[loser].points = 750

        self.update_groups(self.players[winner])
        self.update_groups(self.players[loser])

    def update_groups(self, player):
        if player.points >= 1500:
            group = 'diamond'
        elif player.points >= 1300:
            group = 'emerald'
        elif player.points >= 1100:
            group = 'gold'
        else:
            group = 'unrated'

        if player.group != group:
            if player.group != 'unrated':
                self.groups[player.group].remove(player.name)

            if group != 'unrated':
                if len(self.groups[group]) < 8:
                    self.groups[group].append(player.name)
                else:
                    # Re-add the player to their original group if they cannot enter the new group
                    if player.group != 'unrated':
                        self.groups[player.group].append(player.name)
                    return False
            else:
                self.groups[group].append(player.name)
            player.group = group

        return True



    def fight(self, winner, loser, winner_stocks_lost_match1, winner_stocks_lost_match2, winner_stocks_lost_match3=None):
        win_bonus = 25
        lose_penalty = -25
        streak_bonus = 5
        performance_bonus = 5

        winner_stocks = [winner_stocks_lost_match1, winner_stocks_lost_match2]
        loser_stocks = [3 - winner_stocks_lost_match1, 3 - winner_stocks_lost_match2]

        if winner_stocks_lost_match3 is not None:
            winner_stocks.append(winner_stocks_lost_match3)
            loser_stocks.append(3 - winner_stocks_lost_match3)

        winner_stocks.sort(reverse=True)
        loser_stocks.sort()

        winner_bonus_tally = 0
        loser_penalty_tally = 0

        for i in range(len(winner_stocks)):
            winner_bonus_tally += win_bonus
            loser_penalty_tally += lose_penalty

            if winner_stocks[i] == 0:
                winner_bonus_tally += performance_bonus

            if i > 0:
                winner_bonus_tally += streak_bonus
                loser_penalty_tally -= streak_bonus

        self.players[winner].points += winner_bonus_tally
        self.players[loser].points += loser_penalty_tally

        if self.players[loser].points < 750:
            self.players[loser].points = 750

        self.update_groups(self.players[winner])
        self.update_groups(self.players[loser])


    

# %%
