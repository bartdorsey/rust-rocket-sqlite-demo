-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks
(
   id INTEGER PRIMARY KEY NOT NULL,
   name TEXT NOT NULL,
   description TEXT NOT NULL
);
