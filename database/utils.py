import asqlite
from asqlite import Cursor

from player.player import Player


async def ensure_db_setup():
    """
    Setups the database. Includes creation of db and tables and
    inserting default values such as ranks.
    """
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            results = await cursor.execute("SELECT name FROM sqlite_master")
            tables = await results.fetchall()
            num_tables = len(tables)  # type: ignore

            # true if db doesn't exist or has missing tables.
            if num_tables == 0 or num_tables < 5:
                await execute_from_file(cursor, "sql/creation.sql")
                await execute_from_file(cursor, "sql/insert_ranks.sql")
            elif num_tables == 5:
                ranks = await cursor.execute("SELECT * FROM Ranks")
                ranks = await ranks.fetchall()

                if len(ranks) < 4:  # type: ignore
                    await execute_from_file(cursor, "sql/insert_ranks.sql")


async def execute_from_file(cursor: Cursor, name: str):
    with open(name) as f:
        lines = "".join(f.readlines())
        await cursor.executescript(lines)


async def insert_new_player(cursor: Cursor, uid: int):
    await cursor.execute(
        "INSERT INTO Players Values(?, ?, ?, ?, ?, ?, ?, ?)",
        (uid, 0, 0, 0, "Unrated", 900, 0, 0),
    )


async def does_player_exist(cursor: Cursor, uid: int) -> bool:
    results = await cursor.execute("SELECT UID FROM Players WHERE UID = ?", uid)
    results = await results.fetchone()
    if results is None:
        return False

    return True


async def get_player_data(cursor: Cursor, uid: int):
    sql = "SELECT * FROM Players WHERE UID = ?"
    result = await cursor.execute(sql, (uid))
    result = await result.fetchone()
    points, win_streak, lose_streak = result[5::]

    return Player(uid, points, max(win_streak, lose_streak))


async def get_pending_matches(uid: int):
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            sql = f"""
            SELECT
                Challenger, Challenged, Date
            FROM
                History
            WHERE
                (Challenger = '{uid}' OR Challenged = '{uid}')
                AND Finished = 0
            ORDER BY Date ASC
            """

            result = await cursor.execute(sql)
            return await result.fetchall()


async def player_has_match_at_time(uid: int, date: str) -> bool:
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            sql = f"""
            SELECT * FROM History
            WHERE 
                (Challenger = "{uid}" OR Challenged = "{uid}")
                AND Date = "{date}"
            """

            result = await cursor.execute(sql)
            if await result.fetchone() is None:
                return False

            return True


async def cancel_match(u1: int, u2: int, date: str):
    async with asqlite.connect("database.db") as db:
        async with db.cursor() as cursor:
            sql = f"""
            DELETE FROM History 
            WHERE
                ((Challenger = "{u1}" AND Challenged = "{u2}")
                OR 
                (Challenger = "{u2}" AND Challenged = "{u1}"))
                AND Date = "{date}";
            """

            await cursor.execute(sql)


if __name__ == "__main__":
    pass
