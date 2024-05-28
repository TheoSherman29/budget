-- Your SQL goes here

CREATE TABLE transactions (
	id INTEGER NOT NULL,
	date TEXT NOT NULL,
	tpe TEXT NOT NULL,
	amount REAL NOT NULL,
	source TEXT,
	destination TEXT,
	category TEXT NOT NULL,
	description TEXT,
	earmark TEXT,
	CONSTRAINT transactions_pkey PRIMARY KEY (id)
);
