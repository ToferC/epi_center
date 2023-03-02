use std::fmt::Debug;

use chrono::{prelude::*};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::schema::*;
use crate::database::connection;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[table_name = "works"]
pub struct Work {
    pub id: Uuid,
    pub assigned_by_person_id: Uuid, // Person
    pub assigned_to_person_id: Option<Uuid>, // Person
    pub team_id: Uuid, // Team
    pub title_en: String,
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_datestamp: NaiveDateTime,
    pub target_completion_date: NaiveDateTime,
    pub work_status: WorkStatus,
    pub effort: f64,
    pub completed_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::WorkStatus"]
pub enum WorkStatus {
    Planning,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

// Non Graphql
impl Work {
    pub fn create(work: &NewWork) -> Result<Work> {
        let mut conn = connection()?;

        let res = diesel::insert_into(works::table)
        .values(work)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(work: &NewWork) -> Result<Work> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::assigned_by_person_id.eq(&work.assigned_by_person_id)
                .and(works::title_en.eq(&work.title_en))
                .and(works::assigned_to_person_id.eq(&work.assigned_to_person_id))
                .and(works::target_completion_date.eq(&work.target_completion_date))
            )
            .distinct()
            .first(&mut conn);
        
        let work = match res {
            Ok(p) => p,
            Err(e) => {
                // Work not found
                println!("{:?}", e);
                let p = Work::create(work).expect("Unable to create work");
                p
            }
        };
        Ok(work)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = works::table.load::<Work>(&mut conn)?;
        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = works::table
            .limit(count)
            .load::<Work>(&mut conn)?;
        
        Ok(res)
    }

    pub fn get_by_id(id: Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let res = works::table.filter(works::id.eq(id)).first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_team_id(id: Uuid) -> Result<Vec<Work>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::team_id.eq(id))
            .load::<Work>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_assigning_person_id(id: Uuid) -> Result<Vec<Work>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::assigned_by_person_id.eq(id))
            .load::<Work>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_assigned_person_id(id: Uuid) -> Result<Vec<Work>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::assigned_to_person_id.eq(id))
            .load::<Work>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(works::table)
        .filter(works::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject, InputObject)]
#[table_name = "works"]
pub struct NewWork {
    pub assigned_by_person_id: Uuid, // Person
    pub assigned_to_person_id: Option<Uuid>, // Person
    pub team_id: Uuid, // Team
    pub title_en: String,
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_datestamp: NaiveDateTime,
    pub target_completion_date: NaiveDateTime,
    pub work_status: WorkStatus,
    pub effort: f64,
}

impl NewWork {

    pub fn new(
        assigned_by_person_id: Uuid, // Person
        assigned_to_person_id: Option<Uuid>, // Person
        team_id: Uuid, // Work
        title_en: String,
        outcome_en: String,
        outcome_fr: String,
        start_datestamp: NaiveDateTime,
        target_completion_date: NaiveDateTime,
        work_status: WorkStatus,
        effort: f64,

    ) -> Self {
        NewWork {
            assigned_by_person_id,
            assigned_to_person_id,
            team_id,
            title_en,
            outcome_en,
            outcome_fr,
            start_datestamp,
            target_completion_date,
            work_status,
            effort,
        }
    }
}
