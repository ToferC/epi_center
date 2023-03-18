use std::fmt::Debug;

use chrono::{prelude::*};
use diesel::dsl::sum;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::models::{TaskStatus, SkillDomain, Person, Task, CapabilityLevel};
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
    pub person_id: Uuid,
    pub work_description: String,
    pub domain: SkillDomain,
    pub capability_level: CapabilityLevel,
    pub effort: i32,
    pub work_status: TaskStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Work {
    pub async fn task(&self) -> Result<Task> {
        Task::get_by_id(&self.task_id)
    }

    pub async fn person(&self) -> Result<Person> {
        Person::get_by_id(&self.person_id)
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
    
    pub fn get_or_create(work: &NewWork) -> Result<Work> {
        let mut conn = connection()?;

        let res = works::table
        .filter(works::task_id.eq(&work.task_id)
            .and(works::person_id.eq(&work.person_id))
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

    pub fn get_worker_ids(task_id: &Uuid) -> Result<Vec<Uuid>> {

        let mut conn = connection()?;
        let res: Vec<Uuid> = works::table
            .filter(works::task_id.eq(task_id))
            .select((works::person_id))
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

    pub fn get_by_person_id(person_id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::person_id.eq(person_id))
            .load::<Work>(&mut conn)?;

        Ok(res)
    }

    /// Return the numeric indicator of the total effort allocated to a person.
    pub fn sum_person_effort(person_id: &Uuid) -> Result<i32> {
        let mut conn = connection()?;

        let res = works::table
            .filter(works::person_id.eq(person_id))
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
    pub person_id: Uuid,
    pub work_description: String,
    pub domain: SkillDomain,
    pub capability_level: CapabilityLevel,
    pub effort: i32,
    pub work_status: TaskStatus,
}

impl NewWork {

    pub fn new(
        task_id: Uuid,
        person_id: Uuid,
        work_description: String,
        domain: SkillDomain,
        capability_level: CapabilityLevel,
        effort: i32,
        work_status: TaskStatus,
    ) -> Self {
        NewWork {
            task_id,
            person_id,
            work_description,
            domain,
            capability_level,
            effort,
            work_status,
        }
    }
}
