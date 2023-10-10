import sqlite3

class Database:
    def __init__(self):
        self.con = sqlite3.connect("database.db")
        self.cur = self.con.cursor()

        self.__verify_database()

    def __verify_database(self):
        res = self.cur.execute("SELECT name FROM sqlite_master")
        tables = res.fetchall()
        num_tables = len(tables)

        if num_tables == 0 or num_tables < 5:
            self.__execute_script("creation.sql")
        elif num_tables == 5:
            ranks = self.cur.execute("SELECT * FROM Ranks")
            ranks = ranks.fetchall()

            if len(ranks) < 4:
                self.__execute_script("insert_ranks.sql")

    def __execute_script(self, name: str):
        with open(name) as f:
            lines = "".join(f.readlines())
            self.cur.executescript(lines)

if __name__ == "__main__":
    Database()
