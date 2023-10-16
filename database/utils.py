import aiosqlite
from aiosqlite import Cursor

async def verify_database():
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

if __name__ == "__main__":
    pass
