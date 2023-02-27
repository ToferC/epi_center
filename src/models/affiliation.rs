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
#[table_name = "affiliations"]
pub struct Affiliation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// Non Graphql
impl Affiliation {
    pub fn create(conn: &PgConnection, affiliation: &NewAffiliation) -> FieldResult<Affiliation> {
        let res = diesel::insert_into(affiliations::table)
        .values(affiliation)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, affiliation: &NewAffiliation) -> FieldResult<Affiliation> {
        let res = affiliations::table
        .filter(affiliations::family_name.eq(&affiliation.family_name))
        .distinct()
        .first(conn);
        
        let affiliation = match res {
            Ok(p) => p,
            Err(e) => {
                // Affiliation not found
                println!("{:?}", e);
                let p = Affiliation::create(conn, affiliation).expect("Unable to create affiliation");
                p
            }
        };
        Ok(affiliation)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let affiliations = affiliations::table.load::<Affiliation>(&conn)?;
        Ok(affiliations)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let affiliation = affiliations::table.filter(affiliations::id.eq(id)).first(&conn)?;
        Ok(affiliation)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(affiliations::table)
        .filter(affiliations::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "affiliations"]
pub struct NewAffiliation {
    pub organization_id: Uuid,
    pub role: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewAffiliation {

    pub fn new(
        organization_id: Uuid,
        role: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewAffiliation {
            organization_id,
            role,
            start_date,
            end_date,
            created_at,
            updated_at,
        }
    }
}
