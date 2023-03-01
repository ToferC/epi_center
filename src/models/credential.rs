use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "credentials"]
// External certifications or credentials like degrees, professional certs, etc
pub struct Credential {
    pub id: Uuid,
    pub person_id: Uuid,
    pub provider: String,
    pub description: String,
    pub received_date: NaiveDate,
    pub validated: bool,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Non Graphql
impl Credential {
    pub fn create(conn: &PgConnection, credential: &NewCredential) -> Result<Credential> {
        let res = diesel::insert_into(credentials::table)
        .values(credential)
        .get_result(conn);
        
        Ok(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, credential: &NewCredential) -> Result<Credential> {
        let res = credentials::table
        .filter(credentials::family_name.eq(&credential.family_name))
        .distinct()
        .first(conn);
        
        let credential = match res {
            Ok(p) => p,
            Err(e) => {
                // Credential not found
                println!("{:?}", e);
                let p = Credential::create(conn, credential).expect("Unable to create credential");
                p
            }
        };
        Ok(credential)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let credentials = credentials::table.load::<Credential>(&conn)?;
        Ok(credentials)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let credential = credentials::table.filter(credentials::id.eq(id)).first(&conn)?;
        Ok(credential)
    }
    
    pub fn update(&self, conn: &PgConnection) -> Result<Self> {
        let res = diesel::update(credentials::table)
        .filter(credentials::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "credentials"]
pub struct NewCredential {
    pub person_id: Uuid,
    pub provider: String,
    pub description: String,
    pub received_date: NaiveDate,
    pub validated: bool,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewCredential {

    pub fn new(
        person_id: Uuid,
        provider: String,
        description: String,
        received_date: NaiveDate,
        validated: bool,
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewCredential {
            person_id,
            provider,
            description,
            received_date,
            validated,
            created_at,
            updated_at,
        }
    }
}
