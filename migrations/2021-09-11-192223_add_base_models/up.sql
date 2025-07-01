-- Your SQL goes here

CREATE TYPE access_level_enum AS ENUM (
    'adminstrator',
    'analyst',
    'employee',
    'research',
    'open'
);

CREATE TABLE IF NOT EXISTS valid_roles (
   role VARCHAR(64) PRIMARY KEY
);

INSERT INTO valid_roles (role) VALUES
    ('ADMIN'),
    ('USER'),
    ('ANALYST'),
    ('OPERATOR');

CREATE TABLE IF NOT EXISTS users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    hash VARCHAR(255) NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL UNIQUE,
    role VARCHAR(64) REFERENCES valid_roles (role) ON UPDATE CASCADE DEFAULT 'USER' NOT NULL,
    name VARCHAR(256) NOT NULL,
    access_level VARCHAR(64) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    access_key VARCHAR(256) NOT NULL,
    approved_by_user_uid UUID
);

CREATE UNIQUE INDEX users__email_idx ON users(email);

-- Your SQL goes here

CREATE TABLE IF NOT EXISTS organizations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    acronym_en VARCHAR(16) UNIQUE NOT NULL,
    acronym_fr VARCHAR(16) UNIQUE NOT NULL,
    org_type VARCHAR(32) NOT NULL,
    url VARCHAR(256) NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS persons (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID UNIQUE NOT NULL,
    family_name VARCHAR NOT NULL,
    given_name VARCHAR NOT NULL,

    email VARCHAR(128) UNIQUE NOT NULL,
    phone VARCHAR(32) UNIQUE NOT NULL,
    work_address VARCHAR(256) NOT NULL,
    city VARCHAR(128) NOT NULL,
    province VARCHAR(128) NOT NULL,
    postal_code VARCHAR(16) NOT NULL,
    country VARCHAR(128) NOT NULL,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    peoplesoft_id VARCHAR NOT NULL,
    orcid_id VARCHAR NOT NULL,
    
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP
);

CREATE TYPE skill_domain as ENUM ('combat', 'strategy', 'intelligence', 'information_technology', 
    'human_resources', 'finance', 'communications', 'administration', 'engineering', 'medical', 
    'management', 'leadership', 'joint_operations');

CREATE TABLE IF NOT EXISTS org_tiers (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,
    
    tier_level INT NOT NULL,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,

    primary_domain skill_domain NOT NULL,

    parent_tier UUID,
    FOREIGN KEY(parent_tier)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS org_tier_ownerships (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    owner_id UUID NOT NULL,
    FOREIGN KEY(owner_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    org_tier_id UUID NOT NULL,
    FOREIGN KEY(org_tier_id)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS teams (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    org_tier_id UUID NOT NULL,
    FOREIGN KEY(org_tier_id)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    primary_domain skill_domain NOT NULL,

    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    description_en TEXT NOT NULL,
    description_fr TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL

);

CREATE TYPE hr_group AS ENUM (
    'ec',
    'as',
    'pm',
    'cr',
    'pe',
    'is',
    'fi',
    'res',
    'ex',
    'dm'
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    team_id UUID NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE RESTRICT,

    title_en VARCHAR(256) NOT NULL,
    title_fr VARCHAR(256) NOT NULL,
    effort FLOAT NOT NULL,
    active bool NOT NULL,

    hr_group hr_group NOT NULL,
    hr_level INT NOT NULL,

    start_datestamp TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS team_ownerships (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    team_id UUID NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE RESTRICT,
        
    start_datestamp TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS skills (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    description_en TEXT NOT NULL,
    description_fr TEXT NOT NULL,
    domain skill_domain NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TYPE capability_level AS ENUM ('desired', 'novice', 'experienced', 'expert', 'specialist');

CREATE TABLE IF NOT EXISTS capabilities (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    name_en VARCHAR(256) NOT NULL,
    name_fr VARCHAR(256) NOT NULL,

    domain skill_domain NOT NULL,

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    skill_id UUID NOT NULL,
    FOREIGN KEY(skill_id)
        REFERENCES skills(id) ON DELETE RESTRICT,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    self_identified_level capability_level NOT NULL,
    validated_level capability_level,


    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL,

    validation_values BIGINT[] NOT NULL
);

CREATE TABLE IF NOT EXISTS affiliations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    home_org_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    affiliation_role VARCHAR(256) NOT NULL,

    start_datestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE work_status AS ENUM (
    'backlog',
    'planning',
    'in_progress',
    'completed',
    'blocked',
    'cancelled'
);

CREATE TABLE IF NOT EXISTS tasks (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    created_by_role_id UUID NOT NULL,
    FOREIGN KEY(created_by_role_id)
        REFERENCES roles(id) ON DELETE RESTRICT,

    title VARCHAR(144) NOT NULL,
    domain skill_domain NOT NULL,
    intended_outcome VARCHAR(1256) NOT NULL,
    final_outcome VARCHAR(1256),

    approval_tier INT NOT NULL DEFAULT 4,
    url VARCHAR(256) NOT NULL,

    start_datestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    target_completion_date TIMESTAMP NOT NULL,

    task_status work_status NOT NULL DEFAULT 'planning',

    completed_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE language_name AS ENUM (
    'english',
    'french',
    'arabic',
    'chinese',
    'spanish',
    'german',
    'japanese',
    'korean',
    'italian',
    'other'
);

CREATE TYPE language_level AS ENUM (
    'a',
    'b',
    'c',
    'e',
    'x'
);

CREATE TABLE IF NOT EXISTS language_datas (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    language_name language_name NOT NULL,
    reading language_level,
    writing language_level,
    speaking language_level,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

