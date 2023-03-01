use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel_derive_enum::{DbEnum};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::graphql::graphql_translate;
use crate::errors::error_handler::CustomError;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::CapabilityLevel"]
/// Enums for Capability -> shift to 0 - 4
pub enum CapabilityLevel {
    Desired,
    Novice,
    Experienced,
    Expert,
    Specialist,
}

// Graphql
#[ComplexObject]
impl Capability {
    pub async fn person(&self) -> Result<Person> {
        Person::get_by_id(&self.person_id)
    }

    pub async fn skill(&self) -> Result<Skill> {
        Skill::get_by_id(&self.skill_id)
    }
}

// Non Graphql
impl Capability {
    pub fn create(capability: &NewCapability) -> FieldResult<Capability> {
        let mut conn = connection()?;

        let res = diesel::insert_into(capabilities::table)
            .values(capability)
            .get_result(&mut conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(capability: &NewCapability) -> FieldResult<Capability> {
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

    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let capabilities = capabilities::table.load::<Capability>(&mut conn)?;
        Ok(capabilities)
    }

    pub fn get_by_id(id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;
        let capability = capabilities::table.filter(capabilities::id.eq(id)).first(&mut conn)?;
        Ok(capability)
    }

    pub fn get_by_skill_id(id: Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = capabilities::table
            .filter(capabilities::skill_id.eq(id))
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
    
    pub fn update(&self) -> FieldResult<Self> {

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
            validated_level: None,
        }
    }
}
