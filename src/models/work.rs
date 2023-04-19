use std::fmt::Debug;

use chrono::{prelude::*};
use diesel_derive_enum::DbEnum;
use rand::Rng;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::models::{SkillDomain, Role, Task, CapabilityLevel};
use crate::database::connection;

/// Data structure for a relationship between a person and work
/// This is a many to many relationship as multiple people may be 
/// assigned to a specific piece of work and a person may be assigned
/// to multiple pieces of work
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[table_name = "works"]
pub struct Work {
    pub id: Uuid,
    #[graphql(skip)]
    pub task_id: Uuid,
    #[graphql(skip)]
    pub role_id: Uuid,
    pub work_description: String,
    pub url: Option<String>,
    pub domain: SkillDomain,
    pub capability_level: CapabilityLevel,
    pub effort: i32,
    pub work_status: WorkStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Work {
    pub async fn task(&self) -> Result<Task> {
        Task::get_by_id(&self.task_id)
    }

    pub async fn role(&self) -> Result<Role> {
        Role::get_by_id(&self.role_id)
    }
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

    pub fn batch_create(works: &Vec<NewWork>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(works::table)
            .values(works)
            .execute(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(work: &NewWork) -> Result<Work> {
        let mut conn = connection()?;

        let res = works::table
        .filter(works::task_id.eq(&work.task_id)
            .and(works::role_id.eq(&work.role_id))
            .and(works::work_description.eq(&work.work_description)))
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
        let persons = works::table.load::<Work>(&mut conn)?;
        Ok(persons)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let persons = works::table
            .limit(count)
            .load::<Work>(&mut conn)?;
        Ok(persons)
    }

    pub fn get_worker_ids(task_id: &Uuid) -> Result<Vec<Uuid>> {

        let mut conn = connection()?;
        let res: Vec<Uuid> = works::table
            .filter(works::task_id.eq(task_id))
            .select(works::role_id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let person = works::table
            .filter(works::id.eq(id))
            .first(&mut conn)?;
        Ok(person)
    }

    pub fn get_by_role_id(role_id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::role_id.eq(role_id))
            .order_by(works::created_at)
            .load::<Work>(&mut conn)?;

        Ok(res)
    }

    /// Return the numeric indicator of the total effort allocated to a person.
    pub fn sum_role_effort(role_id: &Uuid) -> Result<i32> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::role_id.eq(role_id))
            .filter(works::work_status.ne_all(vec![WorkStatus::Cancelled, WorkStatus::Completed]))
            .select(works::effort)
            .load::<i32>(&mut conn)?;

        let total_effort = res.into_iter()
            .sum();

        Ok(total_effort)
    }

    /// Return the numeric indicator of the total effort allocated to a task.
    pub fn sum_task_effort(task_id: &Uuid) -> Result<i32> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::task_id.eq(task_id))
            .select(works::effort)
            .load::<i32>(&mut conn)?;

        let total_effort = res.into_iter()
            .sum();

        Ok(total_effort)
    }

    pub fn get_by_task_id(task_id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::task_id.eq(task_id))
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

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[table_name = "works"]
pub struct NewWork {
    pub task_id: Uuid,
    pub role_id: Uuid,
    pub work_description: String,
    pub url: Option<String>,
    pub domain: SkillDomain,
    pub capability_level: CapabilityLevel,
    pub effort: i32,
    pub work_status: WorkStatus,
}

impl NewWork {

    pub fn new(
        task_id: Uuid,
        role_id: Uuid,
        work_description: String,
        url: Option<String>,
        domain: SkillDomain,
        capability_level: CapabilityLevel,
        effort: i32,
        work_status: WorkStatus,
    ) -> Self {
        NewWork {
            task_id,
            role_id,
            work_description,
            url,
            domain,
            capability_level,
            effort,
            work_status,
        }
    }
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

impl Distribution<WorkStatus> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WorkStatus {
        match rng.gen_range(0..=10) {
            0..=1 => WorkStatus::Planning,
            2..=7 => WorkStatus::InProgress,
            8 => WorkStatus::Completed,
            9 => WorkStatus::Cancelled,
            10 => WorkStatus::Blocked,
            _ => WorkStatus::Blocked,
        }
    }
}
