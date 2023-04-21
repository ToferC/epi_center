-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS language_datas;
DROP TYPE IF EXISTS language_level;
DROP TYPE IF EXISTS language_name;

DROP TABLE IF EXISTS tasks;
DROP TYPE IF EXISTS work_status;

DROP TABLE IF EXISTS affiliations;
DROP TABLE IF EXISTS capabilities;
DROP TYPE IF EXISTS capability_level;

DROP TABLE IF EXISTS skills;

DROP TABLE IF EXISTS team_ownerships;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS teams;

DROP TABLE IF EXISTS org_tier_ownerships;
DROP TABLE IF EXISTS org_tiers;

DROP TABLE IF EXISTS persons;
DROP TYPE IF EXISTS hr_group;

DROP TABLE IF EXISTS organizations;

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS valid_roles;

DROP TYPE IF EXISTS access_level_enum;
DROP TYPE IF EXISTS skill_domain;


