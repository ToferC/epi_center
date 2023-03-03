use std::fmt::Debug;

use chrono::{prelude::*};
use rand::{distributions::{Distribution,Standard}, Rng};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods, PgTextExpressionMethods};
use diesel::dsl::count;
use diesel_derive_enum::{DbEnum};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::{database::connection};

use crate::{schema::*, database};

use crate::models::{Person, Skill, Organization, SkillDomain};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, SimpleObject, Associations)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Skill))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = capabilities)]
#[graphql(complex)]
pub struct Capability {
    pub id: Uuid,

    pub name_en: String,
    pub name_fr: String,

    pub domain: SkillDomain,

    #[graphql(visible = false)]
    pub person_id: Uuid, // Person
    pub organization_id: Uuid, // Organization

    #[graphql(visible = false)]
    pub skill_id: Uuid, // Skill
    pub self_identified_level: CapabilityLevel,
    pub validated_level: Option<CapabilityLevel>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum, PartialOrd, Ord, Display)]
#[ExistingTypePath = "crate::schema::sql_types::CapabilityLevel"]
/// Enums for Capability -> shift to 0 - 4
pub enum CapabilityLevel {
    Desired,
    Novice,
    Experienced,
    Expert,
    Specialist,
}

impl Distribution<CapabilityLevel> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CapabilityLevel {
        match rng.gen_range(0..11) {
            0 => CapabilityLevel::Desired,
            1 => CapabilityLevel::Novice,
            2..=7 => CapabilityLevel::Experienced,
            8..=9 => CapabilityLevel::Expert,
            10 => CapabilityLevel::Specialist,
            _ => CapabilityLevel::Desired,
        }
    }
}

impl CapabilityLevel {
    pub fn step_down(&self) -> CapabilityLevel {
        match self {
            CapabilityLevel::Desired => CapabilityLevel::Desired,
            CapabilityLevel::Novice => CapabilityLevel::Desired,
            CapabilityLevel::Experienced => CapabilityLevel::Novice,
            CapabilityLevel::Expert => CapabilityLevel::Experienced,
            CapabilityLevel::Specialist => CapabilityLevel::Expert,
        }
    }
}

// Graphql
#[ComplexObject]
impl Capability {
    pub async fn person(&self) -> Result<Person> {
        Person::get_by_id(&self.person_id)
    }

    pub async fn skill_name(&self) -> Result<String> {
        Skill::get_name_by_id(&self.skill_id)
    }

    pub async fn skill(&self) -> Result<Skill> {
        Skill::get_by_id(&self.skill_id)
    }
}

// Non Graphql
impl Capability {
    pub fn create(capability: &NewCapability) -> Result<Capability> {
        let mut conn = connection()?;

        let res = diesel::insert_into(capabilities::table)
            .values(capability)
            .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(capability: &NewCapability) -> Result<Capability> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::person_id.eq(&capability.person_id)
            .and(capabilities::skill_id.eq(&capability.skill_id)))
            .distinct()
            .first(&mut conn);
        
        let capability = match res {
            Ok(p) => p,
            Err(e) => {
                // Capability not found
                println!("{:?}", e);
                let p = Capability::create(capability).expect("Unable to create capability");
                p
            }
        };
        Ok(capability)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = capabilities::table.load::<Capability>(&mut conn)?;
        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = database::connection()?;
        let res = capabilities::table.limit(count).load::<Capability>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self>{
        let mut conn = database::connection()?;
        let res = capabilities::table.filter(capabilities::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_single_by_skill_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::skill_id.eq(id))
            .load::<Capability>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::name_en.ilike(format!("%{}%", name)).or(capabilities::name_fr.ilike(format!("%{}%", name))))
            .load::<Capability>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name_and_level(name: &String, level: CapabilityLevel) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::name_en.ilike(format!("%{}%", name)).or(capabilities::name_fr.ilike(format!("%{}%", name))))
            .filter(capabilities::self_identified_level.eq(level))
            .load::<Capability>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_person_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::person_id.eq(id))
            .load::<Capability>(&mut conn)?;

        Ok(res)
    }

    pub fn get_level_counts_by_name(name: String) -> Result<Vec<CapabilityCount>> {
        let mut conn = connection()?;

        let skill_id = Skill::get_top_skill_id_by_name(name)?;

        let res: Vec<(String, SkillDomain, CapabilityLevel, i64)> = capabilities::table
            .filter(capabilities::skill_id.eq(skill_id))
            .group_by((capabilities::domain, capabilities::self_identified_level, capabilities::name_en))
            .select((capabilities::name_en, capabilities::domain, capabilities::self_identified_level, count(capabilities::id)))
            .order_by((capabilities::name_en, capabilities::self_identified_level))
            .load::<(String, SkillDomain, CapabilityLevel, i64)>(&mut conn)?;

        // Convert res into CapabilityCountStruct
        let mut counts: Vec<CapabilityCount> = Vec::new();

        for r in res {
            let count = CapabilityCount::from(r);
            counts.push(count);
        }

        Ok(counts)
    }

    pub fn get_level_counts_by_domain(domain: SkillDomain) -> Result<Vec<CapabilityCount>> {
        let mut conn = connection()?;

        let res: Vec<(String, SkillDomain, CapabilityLevel, i64)> = capabilities::table
            .filter(capabilities::domain.eq(domain))
            .group_by((capabilities::domain, capabilities::self_identified_level, capabilities::name_en))
            .select((capabilities::name_en, capabilities::domain, capabilities::self_identified_level, count(capabilities::id)))
            .order_by((capabilities::name_en, capabilities::self_identified_level))
            .load::<(String, SkillDomain, CapabilityLevel, i64)>(&mut conn)?;

        // Convert res into CapabilityCountStruct
        let mut counts: Vec<CapabilityCount> = Vec::new();

        for r in res {
            let count = CapabilityCount::from(r);
            counts.push(count);
        }

        Ok(counts)
    }
    
    pub fn update(&self) -> Result<Self> {

        let mut conn = database::connection()?;

        let res = diesel::update(capabilities::table)
        .filter(capabilities::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[table_name = "capabilities"]
pub struct NewCapability {
    pub name_en: String,
    pub name_fr: String,
    pub domain: SkillDomain,
    pub person_id: Uuid, // Person
    pub skill_id: Uuid, // Skill
    pub organization_id: Uuid,
    pub self_identified_level: CapabilityLevel,
    pub validated_level: Option<CapabilityLevel>,
}

impl NewCapability {

    pub fn new(
        person_id: Uuid, // Person
        skill_id: Uuid, // Skill
        organization_id: Uuid,
        self_identified_level: CapabilityLevel,
    ) -> Self {

        let skill = Skill::get_by_id(&skill_id).expect("Unable to get skill");

        NewCapability {
            name_en: skill.name_en,
            name_fr: skill.name_fr,
            domain: skill.domain,
            person_id,
            skill_id,
            organization_id,
            self_identified_level,
            validated_level: Some(self_identified_level),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub struct CapabilityCount {
    pub name: String,
    pub domain: SkillDomain,
    pub level: String,
    pub counts: i64,
}

impl From<(String, SkillDomain, CapabilityLevel, i64)> for CapabilityCount {
    fn from((name, domain, level, counts): (String, SkillDomain, CapabilityLevel, i64)) -> Self {
        CapabilityCount {
            name,
            domain,
            level: level.to_string(),
            counts,
        }
    }
}

impl CapabilityCount {
    pub fn new(name: String, domain: SkillDomain, level: String, counts: i64) -> Self {
        CapabilityCount {
            name,
            domain,
            level,
            counts,
        }
    }
}
