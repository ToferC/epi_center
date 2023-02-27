// @generated automatically by Diesel CLI.

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
        organization_id -> Uuid,
        peoplesoft_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        person_id -> Uuid,
        team_id -> Uuid,
        title_en -> Varchar,
        title_fr -> Varchar,
        effort -> Float8,
        active -> Bool,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
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
        access_key -> Varchar,
        approved_by_user_uid -> Nullable<Uuid>,
    }
}

diesel::table! {
    valid_roles (role) {
        role -> Varchar,
    }
}

diesel::joinable!(org_tier_ownerships -> org_tiers (org_tier_id));
diesel::joinable!(org_tier_ownerships -> persons (owner_id));
diesel::joinable!(org_tiers -> organizations (organization_id));
diesel::joinable!(persons -> organizations (organization_id));
diesel::joinable!(roles -> persons (person_id));
diesel::joinable!(roles -> teams (team_id));
diesel::joinable!(team_ownerships -> persons (person_id));
diesel::joinable!(team_ownerships -> teams (team_id));
diesel::joinable!(teams -> org_tiers (org_tier_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(users -> valid_roles (role));

diesel::allow_tables_to_appear_in_same_query!(
    org_tier_ownerships,
    org_tiers,
    organizations,
    persons,
    roles,
    team_ownerships,
    teams,
    users,
    valid_roles,
);
