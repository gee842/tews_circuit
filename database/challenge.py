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


async def finish_match(user1: int, user2: int):
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
            (challenger, challenged, date, _) = await result.fetchone()

            sql = """
            UPDATE History
            SET Finished = 1 
            WHERE 
                Challenger = ? AND Challenged = ? AND Date = ?;
            """

            await cursor.execute(sql, (challenger, challenged, date))
