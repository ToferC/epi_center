use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods, PgTextExpressionMethods};
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{database::connection};

use crate::{schema::*, database};

use crate::models::{Role, Skill, CapabilityLevel, SkillDomain};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, SimpleObject, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Skill))]
#[diesel(table_name = requirements)]
#[graphql(complex)]
/// A representation of a roles ability to use a skill at a specific level
pub struct Requirement {
    pub id: Uuid,

    pub name_en: String,
    pub name_fr: String,

    pub domain: SkillDomain,

    #[graphql(visible = false)]
    pub role_id: Uuid, // Role
    pub skill_id: Uuid, // Skill

    pub required_level: CapabilityLevel,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

// Graphql
#[ComplexObject]
impl Requirement {
    pub async fn role(&self) -> Result<Role> {
        Role::get_by_id(&self.role_id)
    }

    pub async fn skill_name(&self) -> Result<String> {
        Skill::get_name_by_id(&self.skill_id)
    }

    pub async fn skill(&self) -> Result<Skill> {
        Skill::get_by_id(&self.skill_id)
    }
}

// Non Graphql
impl Requirement {
    pub fn create(requirement: &NewRequirement) -> Result<Requirement> {
        let mut conn = connection()?;

        let res = diesel::insert_into(requirements::table)
            .values(requirement)
            .get_result(&mut conn)?;
        
        Ok(res)
    }

    pub fn batch_create(requirements: &Vec<NewRequirement>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(requirements::table)
            .values(requirements)
            .execute(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(requirement: &NewRequirement) -> Result<Requirement> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::role_id.eq(&requirement.role_id)
            .and(requirements::skill_id.eq(&requirement.skill_id)))
            .distinct()
            .first(&mut conn);
        
        let requirement = match res {
            Ok(p) => p,
            Err(e) => {
                // Requirement not found
                println!("{:?}", e);
                let p = Requirement::create(requirement).expect("Unable to create requirement");
                p
            }
        };
        Ok(requirement)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = requirements::table.load::<Requirement>(&mut conn)?;
        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = requirements::table.limit(count).load::<Requirement>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self>{
        let mut conn = database::connection()?;
        let res = requirements::table.filter(requirements::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_single_by_skill_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::skill_id.eq(id))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::name_en.ilike(format!("%{}%", name)).or(requirements::name_fr.ilike(format!("%{}%", name))))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name_and_level(name: &String, level: CapabilityLevel) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::name_en.ilike(format!("%{}%", name)).or(requirements::name_fr.ilike(format!("%{}%", name))))
            .filter(requirements::required_level.eq(level))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_skill_id_and_level(id: Uuid, level: CapabilityLevel) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::skill_id.eq(id))
            // the required level must be less than or equal to the provided level
            .filter(requirements::required_level.le(level))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_domain_and_level(domain: &SkillDomain, level: CapabilityLevel) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::domain.eq(domain))
            .filter(requirements::required_level.eq(level))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_role_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = requirements::table
            .filter(requirements::role_id.eq(id))
            .load::<Requirement>(&mut conn)?;

        Ok(res)
    }

    pub fn get_level_counts_by_name(name: String) -> Result<Vec<RequirementCount>> {
        let mut conn = connection()?;

        let skill_id = Skill::get_top_skill_id_by_name(name)?;

        let res: Vec<(String, SkillDomain, CapabilityLevel, i64)> = requirements::table
            .filter(requirements::skill_id.eq(skill_id))
            .group_by((requirements::domain, requirements::required_level, requirements::name_en))
            .select((requirements::name_en, requirements::domain, requirements::required_level, count(requirements::id)))
            .order_by((requirements::name_en, requirements::required_level))
            .load::<(String, SkillDomain, CapabilityLevel, i64)>(&mut conn)?;

        // Convert res into RequirementCountStruct
        let mut counts: Vec<RequirementCount> = Vec::new();

        for r in res {
            let count = RequirementCount::from(r);
            counts.push(count);
        }

        Ok(counts)
    }

    pub fn get_level_counts_by_domain(domain: SkillDomain) -> Result<Vec<RequirementCount>> {
        let mut conn = connection()?;

        let res: Vec<(String, SkillDomain, CapabilityLevel, i64)> = requirements::table
            .filter(requirements::domain.eq(domain))
            .group_by((requirements::domain, requirements::required_level, requirements::name_en))
            .select((requirements::name_en, requirements::domain, requirements::required_level, count(requirements::id)))
            .order_by((requirements::name_en, requirements::required_level))
            .load::<(String, SkillDomain, CapabilityLevel, i64)>(&mut conn)?;

        // Convert res into RequirementCountStruct
        let mut counts: Vec<RequirementCount> = Vec::new();

        for r in res {
            let count = RequirementCount::from(r);
            counts.push(count);
        }

        Ok(counts)
    }
    
    /// Updates a Requirement based on changed data
    pub fn update(&self) -> Result<Self> {

        let mut conn = database::connection()?;

        let res = diesel::update(requirements::table)
            .filter(requirements::id.eq(&self.id))
            .set(self)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[table_name = "requirements"]
pub struct NewRequirement {
    pub name_en: String,
    pub name_fr: String,
    pub domain: SkillDomain,
    pub role_id: Uuid, // Role
    pub skill_id: Uuid, // Skill
    pub required_level: CapabilityLevel,
}

impl NewRequirement {

    pub fn new(
        role_id: Uuid, // Role
        skill_id: Uuid, // Skill
        required_level: CapabilityLevel,
    ) -> Self {

        let skill = Skill::get_by_id(&skill_id).expect("Unable to get skill");
        
        NewRequirement {
            name_en: skill.name_en,
            name_fr: skill.name_fr,
            domain: skill.domain,
            role_id: role_id,
            skill_id: skill.id,
            required_level,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct RequirementCount {
    pub name: String,
    pub domain: SkillDomain,
    pub level: CapabilityLevel,
    pub counts: i64,
}

impl From<(String, SkillDomain, CapabilityLevel, i64)> for RequirementCount {
    fn from((name, domain, level, counts): (String, SkillDomain, CapabilityLevel, i64)) -> Self {
        RequirementCount {
            name,
            domain,
            level,
            counts,
        }
    }
}

impl RequirementCount {
    pub fn new(name: String, domain: SkillDomain, level: CapabilityLevel, counts: i64) -> Self {
        RequirementCount {
            name,
            domain,
            level,
            counts,
        }
    }
}
