CREATE TABLE IF NOT EXISTS "History" (
	"Challenger"	TEXT,
	"Challenged"	TEXT,
	"Date"	DATE,
	"Finished"	INTEGER,
	"Winner"	INTEGER,
	FOREIGN KEY("Challenger") REFERENCES "Players"("UID")
);

CREATE TABLE IF NOT EXISTS "Players" (
	"UID"	TEXT,
	"Win"	INTEGER,
	"Loss"	INTEGER,
	"Disqualifications"	INTEGER,
	"Rank"	TEXT,
	"Points"	INTEGER,
	"WinStreak"	INTEGER,
	"LoseStreak"	INTEGER,
	PRIMARY KEY("UID")
);

CREATE TABLE IF NOT EXISTS "Ranks" (
	"Name"	TEXT,
	PRIMARY KEY("Name")
);
