CREATE TABLE account (
	id serial NOT NULL PRIMARY KEY,
	hash char(4) NOT NULL UNIQUE,
	name varchar(70) NOT NULL UNIQUE,
	password char(70) NOT NULL,
	created timestamp NOT NULL default current_timestamp,
	updated timestamp NOT NULL default current_timestamp
);