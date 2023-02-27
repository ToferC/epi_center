use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::graphql::graphql_translate;

use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "capabilities"]
pub struct Capability {
    pub id: Uuid,
    pub person_id: Uuid, // Person
    pub skill_id: Uuid, // Skill
    pub self_identified_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Enums for Capability -> shift to 0 - 4
pub enum CapabilityLevel {
    Desired,
    Novice,
    Experienced,
    Expert,
    Specialist,
}

// Non Graphql
impl Capability {
    pub fn create(conn: &PgConnection, capability: &NewCapability) -> FieldResult<Capability> {
        let res = diesel::insert_into(capabilities::table)
        .values(capability)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, capability: &NewCapability) -> FieldResult<Capability> {
        let res = capabilities::table
        .filter(capabilities::family_name.eq(&capability.family_name))
        .distinct()
        .first(conn);
        
        let capability = match res {
            Ok(p) => p,
            Err(e) => {
                // Capability not found
                println!("{:?}", e);
                let p = Capability::create(conn, capability).expect("Unable to create capability");
                p
            }
        };
        Ok(capability)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let capabilities = capabilities::table.load::<Capability>(&conn)?;
        Ok(capabilities)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let capability = capabilities::table.filter(capabilities::id.eq(id)).first(&conn)?;
        Ok(capability)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(capabilities::table)
        .filter(capabilities::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "capabilities"]
pub struct NewCapability {
    pub person_id: Uuid, // Person
    pub skill_id: Uuid, // Skill
    pub self_identified_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewCapability {

    pub fn new(
        person_id: Uuid, // Person
        skill_id: Uuid, // Skill
        self_identified_level: u32,
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewCapability {
            person_id,
            skill_id,
            self_identified_level,
            created_at,
            updated_at,
        }
    }
}
