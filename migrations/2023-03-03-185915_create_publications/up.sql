-- Your SQL goes here

CREATE TYPE publication_status AS ENUM (
    'planning',
    'in_progress',
    'draft',
    'submitted',
    'published',
    'rejected',
    'cancelled'
);

CREATE TABLE IF NOT EXISTS publications (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    publishing_organization_id UUID NOT NULL,
    FOREIGN KEY(publishing_organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    lead_author_id UUID NOT NULL,
    FOREIGN KEY(lead_author_id)
        REFERENCES persons(id) ON DELETE RESTRICT,
    
    title VARCHAR(256) NOT NULL,
    subject_text VARCHAR(256) NOT NULL,

    publication_status publication_status NOT NULL,

    url_string VARCHAR(256),

    publishing_id VARCHAR(256),

    submitted_date TIMESTAMP,
    published_datestamp TIMESTAMP,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS publication_contributors (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    publication_id UUID NOT NULL,
    FOREIGN KEY(publication_id)
        REFERENCES publications(id) ON DELETE RESTRICT,

    contributor_id UUID NOT NULL,
    FOREIGN KEY(contributor_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    contributor_role VARCHAR(256) NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS works (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    task_id UUID NOT NULL,
    FOREIGN KEY(task_id)
        REFERENCES tasks(id) ON DELETE RESTRICT,

    role_id UUID NOT NULL,
    FOREIGN KEY(role_id)
        REFERENCES roles(id) ON DELETE RESTRICT,

    work_description VARCHAR(256) NOT NULL,
    url VARCHAR(256),
    domain skill_domain NOT NULL,
    capability_level capability_level NOT NULL,
    effort INT NOT NULL,

    work_status work_status NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS validations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    validator_id UUID NOT NULL,
    FOREIGN KEY(validator_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    capability_id UUID NOT NULL,
    FOREIGN KEY(capability_id)
        REFERENCES capabilities(id) ON DELETE RESTRICT,

    validated_level capability_level NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS requirements (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    name_en VARCHAR(256) NOT NULL,
    name_fr VARCHAR(256) NOT NULL,

    domain skill_domain NOT NULL,

    role_id UUID NOT NULL,
    FOREIGN KEY(role_id)
        REFERENCES roles(id) ON DELETE RESTRICT,

    skill_id UUID NOT NULL,
    FOREIGN KEY(skill_id)
        REFERENCES skills(id) ON DELETE RESTRICT,

    required_level capability_level NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);