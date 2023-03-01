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
#[table_name = "reporting_relationships"]
/// Data structure connecting persons in heirarchical relationship
pub struct ReportingRelationship {
    pub id: Uuid,
    pub reporter: Uuid, // Person
    pub reporting_to: Uuid, // Person
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}


// Non Graphql
impl ReportingRelationship {
    pub fn create(conn: &PgConnection, reporting_relationship: &NewReportingRelationship) -> Result<ReportingRelationship> {
        let res = diesel::insert_into(reporting_relationships::table)
        .values(reporting_relationship)
        .get_result(conn);
        
        Ok(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, reporting_relationship: &NewReportingRelationship) -> Result<ReportingRelationship> {
        let res = reporting_relationships::table
        .filter(reporting_relationships::family_name.eq(&reporting_relationship.family_name))
        .distinct()
        .first(conn);
        
        let reporting_relationship = match res {
            Ok(p) => p,
            Err(e) => {
                // ReportingRelationship not found
                println!("{:?}", e);
                let p = ReportingRelationship::create(conn, reporting_relationship).expect("Unable to create reporting_relationship");
                p
            }
        };
        Ok(reporting_relationship)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let persons = reporting_relationships::table.load::<ReportingRelationship>(&conn)?;
        Ok(persons)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let person = reporting_relationships::table.filter(reporting_relationships::id.eq(id)).first(&conn)?;
        Ok(person)
    }
    
    pub fn update(&self, conn: &PgConnection) -> Result<Self> {
        let res = diesel::update(reporting_relationships::table)
        .filter(reporting_relationships::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "reporting_relationships"]
pub struct NewReportingRelationship {
    pub reporter: Uuid, // Person
    pub reporting_to: Uuid, // Person
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

impl NewReportingRelationship {

    pub fn new(
        reporter: Uuid, // Person
        reporting_to: Uuid, // Person
        description: String,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Self {
        NewReportingRelationship {
            reporter,
            reporting_to,
            description,
            start_date,
            end_date,
        }
    }
}
