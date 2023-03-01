use std::fmt::Debug;

use chrono::{prelude::*};
use rand::{distributions::{Distribution,Standard}, Rng};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::dsl::count;
use diesel_derive_enum::{DbEnum};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::database::connection;

use crate::{schema::*, database};

use super::{Person, Skill};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[diesel(table_name = capabilities)]
#[graphql(complex)]
pub struct Capability {
    pub id: Uuid,

    #[graphql(visible = false)]
    pub person_id: Uuid, // Person

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

    pub fn get_by_skill_ids(ids: Vec<Uuid>) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::skill_id.eq_any(ids))
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

    pub fn get_level_counts_by_name(name: String) -> Result<Vec<(String, i64)>> {
        let mut conn = connection()?;

        let skill_id = Skill::get_top_skill_id_by_name(name)?;

        let res: Vec<(String, i64)> = capabilities::table
            .filter(capabilities::skill_id.eq(skill_id))
            .group_by(capabilities::self_identified_level)
            .select((capabilities::self_identified_level, count(capabilities::id)))
            .load::<(String, i64)>(&mut conn)?;

        Ok(res)
           
           
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

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "capabilities"]
pub struct NewCapability {
    pub person_id: Uuid, // Person
    pub skill_id: Uuid, // Skill
    pub self_identified_level: CapabilityLevel,
    pub validated_level: Option<CapabilityLevel>,
}

impl NewCapability {

    pub fn new(
        person_id: Uuid, // Person
        skill_id: Uuid, // Skill
        self_identified_level: CapabilityLevel,
    ) -> Self {
        NewCapability {
            person_id,
            skill_id,
            self_identified_level,
            validated_level: Some(self_identified_level),
        }
    }
}
