-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS requirements;
DROP TABLE IF EXISTS validations;
DROP TABLE IF EXISTS works;
DROP TABLE IF EXISTS publication_contributors;
DROP TABLE IF EXISTS publications;
DROP TYPE IF EXISTS publication_status;