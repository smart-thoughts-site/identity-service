CREATE TABLE users (
    username varchar(32) PRIMARY KEY NOT NULL,
	salt     char(88) NOT NULL,
    password char(88) NOT NULL,
	email    varchar(32) NOT NULL,
    UNIQUE(email)
);