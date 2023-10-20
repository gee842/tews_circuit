from typing import Tuple, Any

from player import Player
from database.utils import get_player_data
from . import insert_new_player, does_player_exist

import asqlite

from discord import User


async def new_challenge(date: str, challenger: User, challenged: User):
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            if not await does_player_exist(cursor, challenger.id):
                await insert_new_player(cursor, challenger.id)

            if not await does_player_exist(cursor, challenged.id):
                await insert_new_player(cursor, challenged.id)

            await cursor.execute(
                "INSERT INTO History VALUES (?, ?, ?, ?)",
                (challenger.id, challenged.id, date, 0),
            )


async def has_match_with_player(user1: int, user2: int) -> Tuple[Any, Any, Any] | None:
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            sql = """
            SELECT * 
            FROM History 
            WHERE 
                Finished = 0 AND 
                ((Challenger = ? AND Challenged = ?)
                OR 
                (Challenger = ? AND Challenged = ?))
            ORDER BY Date ASC 
            LIMIT 1;
            """

            result = await cursor.execute(sql, (user1, user2, user2, user1))
            result = await result.fetchone()
            if result is None:
                return None

            return result[:3]  # type: ignore


async def finish_match(challenger: int, challenged: int, date: str):
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            sql = """
            UPDATE History
            SET Finished = 1 
            WHERE 
                Challenger = ? AND Challenged = ? AND Date = ?;
            """

            await cursor.execute(sql, (challenger, challenged, date))


def calculate_points(points: int, streak: int, win: bool):
    if win:
        points = points + 25
    else:
        points = points - 25

    if streak == 0:
        bonus = 0
    elif streak == 1:
        bonus = 5
    else:
        bonus = 10

    points += bonus

    if points <= 750:
        points = 750

    return points


def create_sql(win: bool, new_points: int, rank_update: str, player: Player):
    if win:
        streak_update = """
           WinStreak = WinStreak + 1, 
           LoseStreak = 0,
       """
    else:
        streak_update = """
           LoseStreak = LoseStreak + 1,
           WinStreak = 0,
       """

    set_query = f"""
        {'Win = Win + 1' if win else 'Loss = Loss + 1'},
        {streak_update}
        Points = {new_points}
    """

    if rank_update != "":
        set_query += f",\nRank = '{player.rank.name}'"

    return set_query


async def update_player_info(user: int, win: bool):
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            data = await get_player_data(cursor, user)
            (ori_points, win_streak, lose_streak) = data[5::]

            streaks = max(int(win_streak), int(lose_streak))
            new_points = calculate_points(ori_points, streaks, win)

            player = Player(user, new_points)

            point_update = f"{ori_points} -> {new_points}"
            rank_update = player.changed(int(ori_points))
            set_query = create_sql(win, new_points, rank_update, player)

            sql = f"""
            UPDATE Players
            SET 
                {set_query}
            WHERE
                UID = ?
            """

            await cursor.execute(sql, user)

            return f"{point_update}\n{rank_update}"


async def disqualifications():
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            condition = "Date < Date('now') AND Finished = 0"
            sql = f"""
            SELECT
                Challenger, Challenged, Date
            FROM 
                History
            WHERE {condition}
            """

            result = await cursor.execute(sql)
            matches = await result.fetchall()

            for data in matches:
                players = [data[0], data[1]]  # p1 & p2

                for player in players:
                    sql = "SELECT Points FROM Players WHERE UID = ?"
                    result = await cursor.execute(sql, player)
                    point = await result.fetchone()
                    if point is None:
                        return

                    points = point[0]
                    # Points can't be lower than 750.
                    if points == 750:
                        continue

                    sql = """
                    UPDATE
                        Players
                    SET
                        Disqualifications = Disqualifications + 1,
                        Points = Points - 10
                    WHERE
                        UID = ?
                    """

                    await cursor.execute(sql, player)

            sql = f"""
            UPDATE 
                History
            SET
                Finished = 1
            WHERE {condition}
            """

            await cursor.execute(sql)
