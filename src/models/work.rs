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
#[table_name = "works"]
pub struct Work {
    pub id: Uuid,
    pub person_id: Option<Uuid>, // Person
    pub work_id: Option<Uuid>, // Work
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_date: NaiveDate,
    pub target_completion_data: NaiveDate,
    pub work_status: usize,
    pub completed_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub enum WorkStatus {
    Planning, // 0
    InProgress, // 1
    Complete, // 2
    Blocked, // 3
}

// Non Graphql
impl Work {
    pub fn create(conn: &PgConnection, work: &NewWork) -> FieldResult<Work> {
        let res = diesel::insert_into(works::table)
        .values(work)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, work: &NewWork) -> FieldResult<Work> {
        let res = works::table
        .filter(works::family_name.eq(&work.family_name))
        .distinct()
        .first(conn);
        
        let work = match res {
            Ok(p) => p,
            Err(e) => {
                // Work not found
                println!("{:?}", e);
                let p = Work::create(conn, work).expect("Unable to create work");
                p
            }
        };
        Ok(work)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let works = works::table.load::<Work>(&conn)?;
        Ok(works)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let work = works::table.filter(works::id.eq(id)).first(&conn)?;
        Ok(work)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(works::table)
        .filter(works::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "works"]
pub struct NewWork {
    pub person_id: Option<Uuid>, // Person
    pub work_id: Option<Uuid>, // Work
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_date: NaiveDate,
    pub target_completion_data: NaiveDate,
    pub work_status: usize,
    pub completed_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewWork {

    pub fn new(
        person_id: Option<Uuid>, // Person
        work_id: Option<Uuid>, // Work
        outcome_en: String,
        outcome_fr: String,
        start_date: NaiveDate,
        target_completion_data: NaiveDate,
        work_status: usize,
        completed_date: Option<NaiveDate>,
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewWork {
            person_id,
            work_id,
            outcome_en,
            outcome_fr,
            start_date,
            target_completion_data,
            work_status,
            completed_date,
            created_at,
            updated_at,
        }
    }
}
