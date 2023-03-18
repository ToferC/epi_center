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

use super::SkillDomain;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Uuid,
    pub created_by_person_id: Uuid, // Person
    pub title: String,
    pub domain: SkillDomain,
    pub intended_outcome: String,
    pub final_outcome: Option<String>,
    pub approval_tier: i32,
    pub start_datestamp: NaiveDateTime,
    pub target_completion_date: NaiveDateTime,
    pub task_status: TaskStatus,
    pub effort: f64,
    pub completed_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, Enum)]
#[ExistingTypePath = "crate::schema::sql_types::TaskStatus"]
pub enum TaskStatus {
    Planning,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

// Non Graphql
impl Task {
    pub fn create(task: &NewTask) -> Result<Task> {
        let mut conn = connection()?;

        let res = diesel::insert_into(tasks::table)
        .values(task)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(task: &NewTask) -> Result<Task> {
        let mut conn = connection()?;

        let res = tasks::table
            .filter(tasks::created_by_person_id.eq(&task.created_by_person_id)
                .and(tasks::title.eq(&task.title))
                .and(tasks::target_completion_date.eq(&task.target_completion_date))
            )
            .distinct()
            .first(&mut conn);
        
        let task = match res {
            Ok(p) => p,
            Err(e) => {
                // Task not found
                println!("{:?}", e);
                let p = Task::create(task).expect("Unable to create task");
                p
            }
        };
        Ok(task)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = tasks::table.load::<Task>(&mut conn)?;
        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let res = tasks::table
            .limit(count)
            .load::<Task>(&mut conn)?;
        
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let res = tasks::table.filter(tasks::id.eq(id)).first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_assigning_person_id(id: Uuid) -> Result<Vec<Task>> {
        let mut conn = connection()?;

        let res = tasks::table
            .filter(tasks::created_by_person_id.eq(id))
            .load::<Task>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(tasks::table)
        .filter(tasks::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject, InputObject)]
#[table_name = "tasks"]
pub struct NewTask {
    pub created_by_person_id: Uuid, // Person
    pub title: String,
    pub domain: SkillDomain,
    pub intended_outcome: String,
    pub approval_tier: i32,
    pub start_datestamp: NaiveDateTime,
    pub target_completion_date: NaiveDateTime,
    pub task_status: TaskStatus,
    pub effort: f64,
}

impl NewTask {

    pub fn new(
        created_by_person_id: Uuid, // Person
        title: String,
        domain: SkillDomain,
        intended_outcome: String,
        approval_tier: i32,
        start_datestamp: NaiveDateTime,
        target_completion_date: NaiveDateTime,
        task_status: TaskStatus,
        effort: f64,

    ) -> Self {
        NewTask {
            created_by_person_id,
            title,
            domain,
            intended_outcome,
            approval_tier,
            start_datestamp,
            target_completion_date,
            task_status,
            effort,
        }
    }
}
