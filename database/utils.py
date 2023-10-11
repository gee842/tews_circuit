import aiosqlite
from aiosqlite import Cursor

async def verify_database():
    async with aiosqlite.connect("database.db") as db:
        async with db.execute("SELECT name FROM sqlite_master") as cursor:
            tables = await cursor.fetchall()
            num_tables = len(tables) # type: ignore

            if num_tables == 0 or num_tables < 5:
                await execute_from_file(cursor, "sql/creation.sql")
            elif num_tables == 5:
                ranks = await cursor.execute("SELECT * FROM Ranks")
                ranks = await ranks.fetchall()

                if len(ranks) < 4: # type: ignore
                    await execute_from_file(cursor, "sql/insert_ranks.sql")


async def execute_from_file(cursor: Cursor, name: str):
    with open(name) as f:
        lines = "".join(f.readlines())
        await cursor.executescript(lines)

if __name__ == "__main__":
    pass
