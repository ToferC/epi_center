pub struct EmployeeData {
    pub id: Uuid,
    pub person_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub group: String,
    pub level: u32,
    pub hr_state: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    // salary from a separate API call
}

pub struct ContactData {
    pub id: Uuid,
    pub person_id: Uuid,
    pub email: String,
    pub phone: String,
    pub work_address: String,
    pub city: String,
    pub territory: String,
    pub country: String,
}

pub struct DataAccess {
    pub id: Uuid,
    pub person_id: Uuid,
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
}

pub struct IntersectionalData {
    pub id: Uuid,
    pub person_id: Uuid,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub sexuality: String,
    pub disability: String,
    pub ethnicity: String,
    pub family_status: String,
    pub education_level: String,
    pub economic_background: String,
    pub linguistic_background: String,
    pub nationality: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub struct WorkSkillRequirement {
    pub id: Uuid,
    pub work_id: Uuid, // Work
    pub skill_id: Uuid, // Skill
    pub required_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Assessment of a persons work in a role
pub struct Assessment {
    pub id: Uiud,
    pub role_id: Uuid,
    pub assessor_id: Uuid,
    pub assessed_level: u32,
    pub narrative_en: Option<String>,
    pub narrative_fr: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

/// Other people's validations of an individuals Capability
pub struct Validations {
    pub id: Uuid,
    pub validator_id: Uuid, // Person
    pub capability_id: Uuid, // Capability
    pub validated_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}