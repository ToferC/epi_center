// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "capability_level"))]
    pub struct CapabilityLevel;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hr_group"))]
    pub struct HrGroup;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language_level"))]
    pub struct LanguageLevel;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language_name"))]
    pub struct LanguageName;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "publication_status"))]
    pub struct PublicationStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "skill_domain"))]
    pub struct SkillDomain;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "work_status"))]
    pub struct WorkStatus;
}

diesel::table! {
    affiliations (id) {
        id -> Uuid,
        person_id -> Uuid,
        organization_id -> Uuid,
        affiliation_role -> Varchar,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SkillDomain;
    use super::sql_types::CapabilityLevel;

    capabilities (id) {
        id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        domain -> SkillDomain,
        person_id -> Uuid,
        skill_id -> Uuid,
        organization_id -> Uuid,
        self_identified_level -> CapabilityLevel,
        validated_level -> Nullable<CapabilityLevel>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
        validation_values -> Array<Nullable<Int8>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LanguageName;
    use super::sql_types::LanguageLevel;

    language_datas (id) {
        id -> Uuid,
        person_id -> Uuid,
        language_name -> LanguageName,
        reading -> Nullable<LanguageLevel>,
        writing -> Nullable<LanguageLevel>,
        speaking -> Nullable<LanguageLevel>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    org_tier_ownerships (id) {
        id -> Uuid,
        owner_id -> Uuid,
        org_tier_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    org_tiers (id) {
        id -> Uuid,
        organization_id -> Uuid,
        tier_level -> Int4,
        name_en -> Varchar,
        name_fr -> Varchar,
        parent_tier -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        acronym_en -> Varchar,
        acronym_fr -> Varchar,
        org_type -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    persons (id) {
        id -> Uuid,
        user_id -> Uuid,
        family_name -> Varchar,
        given_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        work_address -> Varchar,
        city -> Varchar,
        province -> Varchar,
        postal_code -> Varchar,
        country -> Varchar,
        organization_id -> Uuid,
        peoplesoft_id -> Varchar,
        orcid_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    publication_contributors (id) {
        id -> Uuid,
        publication_id -> Uuid,
        contributor_id -> Uuid,
        contributor_role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PublicationStatus;

    publications (id) {
        id -> Uuid,
        publishing_organization_id -> Uuid,
        lead_author_id -> Uuid,
        title -> Varchar,
        subject_text -> Varchar,
        publication_status -> PublicationStatus,
        url_string -> Nullable<Varchar>,
        publishing_id -> Nullable<Varchar>,
        submitted_date -> Nullable<Timestamp>,
        published_datestamp -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HrGroup;

    roles (id) {
        id -> Uuid,
        person_id -> Nullable<Uuid>,
        team_id -> Uuid,
        title_en -> Varchar,
        title_fr -> Varchar,
        effort -> Float8,
        active -> Bool,
        hr_group -> HrGroup,
        hr_level -> Int4,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SkillDomain;

    skills (id) {
        id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        description_en -> Text,
        description_fr -> Text,
        domain -> SkillDomain,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SkillDomain;
    use super::sql_types::WorkStatus;

    tasks (id) {
        id -> Uuid,
        created_by_role_id -> Uuid,
        title -> Varchar,
        domain -> SkillDomain,
        intended_outcome -> Varchar,
        final_outcome -> Nullable<Varchar>,
        approval_tier -> Int4,
        url -> Varchar,
        start_datestamp -> Timestamp,
        target_completion_date -> Timestamp,
        task_status -> WorkStatus,
        completed_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    team_ownerships (id) {
        id -> Uuid,
        person_id -> Uuid,
        team_id -> Uuid,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        organization_id -> Uuid,
        org_tier_id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        description_en -> Text,
        description_fr -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        hash -> Varchar,
        email -> Varchar,
        role -> Varchar,
        name -> Varchar,
        access_level -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        access_key -> Varchar,
        approved_by_user_uid -> Nullable<Uuid>,
    }
}

diesel::table! {
    valid_roles (role) {
        role -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CapabilityLevel;

    validations (id) {
        id -> Uuid,
        validator_id -> Uuid,
        capability_id -> Uuid,
        validated_level -> CapabilityLevel,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SkillDomain;
    use super::sql_types::CapabilityLevel;
    use super::sql_types::WorkStatus;

    works (id) {
        id -> Uuid,
        task_id -> Uuid,
        role_id -> Uuid,
        work_description -> Varchar,
        url -> Nullable<Varchar>,
        domain -> SkillDomain,
        capability_level -> CapabilityLevel,
        effort -> Int4,
        work_status -> WorkStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(affiliations -> organizations (organization_id));
diesel::joinable!(affiliations -> persons (person_id));
diesel::joinable!(capabilities -> organizations (organization_id));
diesel::joinable!(capabilities -> persons (person_id));
diesel::joinable!(capabilities -> skills (skill_id));
diesel::joinable!(language_datas -> persons (person_id));
diesel::joinable!(org_tier_ownerships -> org_tiers (org_tier_id));
diesel::joinable!(org_tier_ownerships -> persons (owner_id));
diesel::joinable!(org_tiers -> organizations (organization_id));
diesel::joinable!(persons -> organizations (organization_id));
diesel::joinable!(publication_contributors -> persons (contributor_id));
diesel::joinable!(publication_contributors -> publications (publication_id));
diesel::joinable!(publications -> organizations (publishing_organization_id));
diesel::joinable!(publications -> persons (lead_author_id));
diesel::joinable!(roles -> persons (person_id));
diesel::joinable!(roles -> teams (team_id));
diesel::joinable!(tasks -> roles (created_by_role_id));
diesel::joinable!(team_ownerships -> persons (person_id));
diesel::joinable!(team_ownerships -> teams (team_id));
diesel::joinable!(teams -> org_tiers (org_tier_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(users -> valid_roles (role));
diesel::joinable!(validations -> capabilities (capability_id));
diesel::joinable!(validations -> persons (validator_id));
diesel::joinable!(works -> roles (role_id));
diesel::joinable!(works -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    affiliations,
    capabilities,
    language_datas,
    org_tier_ownerships,
    org_tiers,
    organizations,
    persons,
    publication_contributors,
    publications,
    roles,
    skills,
    tasks,
    team_ownerships,
    teams,
    users,
    valid_roles,
    validations,
    works,
);
