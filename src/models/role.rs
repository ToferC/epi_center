use std::{fmt::Debug, collections::HashMap};

use chrono::{prelude::*};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods};
use rand::{distributions::{Distribution, Standard}, Rng};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::config_variables::DATE_FORMAT;

use crate::schema::*;
use crate::database::connection;

use super::{Person, Team, Work, Requirement, Capability};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = roles)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Team))]
/// Intermediary data structure between Person and team
/// Referenced by Person
pub struct Role {
    pub id: Uuid,
    pub person_id: Option<Uuid>, // You can have an empty role on a team
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f64,
    pub active: bool,
    // HR info - this will be another module - just here for expediency
    pub hr_group: HrGroup,
    pub hr_level: i32,

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl Role {

    pub async fn id(&self) -> Uuid {
        self.id
    }

    pub async fn person(&self) -> Option<Person> {

        match self.person_id {
            Some(p) => Some(Person::get_by_id(&p).unwrap()),
            None => None
        }
    }

    pub async fn team(&self) -> Result<Team> {
        Team::get_by_id(&self.team_id)
    }

    pub async fn title_english(&self) -> Result<String> {
        Ok(self.title_en.to_owned())
    }

    pub async fn title_french(&self) -> Result<String> {
        Ok(self.title_fr.to_owned())
    }

    /// Returns the sum effort of all active work underway
    /// Maximum work should be around 10
    pub async fn effort(&self) -> Result<i32> {
        Work::sum_role_effort(&self.id)
    }

    /// Returns a vector of the work undertaken by this role
    pub async fn work(&self) -> Result<Vec<Work>> {
        Work::get_by_role_id(&self.id)
    }

    pub async fn active(&self) -> Result<String> {
        if self.active {
            Ok("Active".to_string())
        } else {
            Ok("INACTIVE".to_string())
        }
    }

    pub async fn requirements(&self) -> Result<Vec<Requirement>> {
        Requirement::get_by_role_id(self.id)
    }

    pub async fn hr_group(&self) -> Result<String> {
        Ok(self.hr_group.to_string())
    }

    pub async fn hr_level(&self) -> Result<i32> {
        Ok(self.hr_level)
    }

    pub async fn start_date(&self) -> Result<String> {
        Ok(self.start_datestamp.format(DATE_FORMAT).to_string())
    }

    pub async fn end_date(&self) -> Result<String> {
        match self.end_date {
            Some(d) => Ok(d.format(DATE_FORMAT).to_string()),
            None => Ok("Still Active".to_string())
        }
    }

    pub async fn created_at(&self) -> Result<String> {
        Ok(self.created_at.format(DATE_FORMAT).to_string())
    }

    pub async fn updated_at(&self) -> Result<String> {
        Ok(self.updated_at.format(DATE_FORMAT).to_string())
    }

    pub async fn find_matches(&self) -> Result<Vec<Person>> {

        let requirements = Requirement::get_by_role_id(self.id)?;

        find_people_by_requirements_met(requirements)
    }
}


// Non Graphql
impl Role {
    pub fn create(role: &NewRole) -> Result<Role> {
        let mut conn = connection()?;

        let res = diesel::insert_into(roles::table)
        .values(role)
        .get_result(&mut conn)?;
        
        Ok(res)
    }

    pub fn batch_create(roles: Vec<NewRole>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(roles::table)
            .values(roles)
            .execute(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(role: &NewRole) -> Result<Role> {
        let mut conn = connection()?;

        let res = roles::table
        .filter(roles::person_id.eq(&role.person_id))
        .distinct()
        .first(&mut conn);
        
        let role = match res {
            Ok(p) => p,
            Err(e) => {
                // Role not found
                println!("{:?}", e);
                let p = Role::create(role).expect("Unable to create role");
                p
            }
        };
        Ok(role)
    }

    pub fn get_all_active() -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let roles = roles::table
            .filter(roles::active.eq(true))
            .load::<Role>(&mut conn)?;
        Ok(roles)
    }

    pub fn get_active(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let roles = roles::table
            .filter(roles::active.eq(true))
            .limit(count)
            .load::<Role>(&mut conn)?;
        
        Ok(roles)
    }

    pub fn count() -> Result<i64> {
        let mut conn = connection()?;

        let res = roles::table
            .count()
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;
        let role = roles::table.filter(roles::id.eq(id)).first(&mut conn)?;
        Ok(role)
    }

    pub fn get_active_vacant_by_ids(ids: &Vec<Uuid>) -> Result<Vec<Self>> {
        let mut conn = connection()?;
        let roles = roles::table
            .filter(roles::id.eq_any(ids))
            .filter(roles::active.eq(true))
            .filter(roles::person_id.is_null())
            .load::<Self>(&mut conn)?;
        Ok(roles)
    }

    pub fn get_by_team_id(id: Uuid) -> Result<Vec<Role>> {
        let mut conn = connection()?;

        let res = roles::table
            .filter(roles::team_id.eq(id))
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    /// Returns active and occupied roles by a team_id
    pub fn get_occupied_by_team_id(id: Uuid) -> Result<Vec<Role>> {
        let mut conn = connection()?;

        let res = roles::table
            .filter(roles::team_id.eq(id))
            .filter(roles::active.eq(true))
            .filter(roles::person_id.is_not_null())
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    /// Returns vacant and active roles
    pub fn get_vacant(count: i64) -> Result<Vec<Role>> {
        let mut conn = connection()?;

        let res = roles::table
            .filter(roles::person_id.is_null())
            .filter(roles::active.eq(true))
            .limit(count)
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    /// Returns vacant and active roles by a team_id
    pub fn get_vacant_by_team_id(id: Uuid) -> Result<Vec<Role>> {
        let mut conn = connection()?;

        let res = roles::table
            .filter(roles::team_id.eq(id))
            .filter(roles::person_id.is_null())
            .filter(roles::active.eq(true))
            .load::<Role>(&mut conn)?;

        Ok(res)
    }

    /// Get roles by person ID. Can add a boolean to choose between active or inactive roles.
    pub fn get_by_person_id(id: Uuid, active: bool) -> Result<Vec<Role>> {
        let mut conn = connection()?;

        let res = roles::table
            .filter(roles::person_id.eq(id))
            .filter(roles::active.eq(active))
            .load::<Role>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&mut self) -> Result<Self> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(roles::table)
        .filter(roles::id.eq(&self.id))
        .set(self.clone())
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub person_id: Option<Uuid>,
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f64,
    pub active: bool,
    // HR info - this will be another module - just here for expediency
    pub hr_group: HrGroup,
    pub hr_level: i32,
    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

impl NewRole {

    pub fn new(
        person_id: Option<Uuid>,
        team_id: Uuid,
        title_en: String,
        title_fr: String,
        effort: f64,
        active: bool,
        hr_group: HrGroup,
        hr_level: i32,
        start_datestamp: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        NewRole {
            person_id,
            team_id,
            title_en,
            title_fr,
            effort,
            active,
            hr_group,
            hr_level,
            start_datestamp,
            end_date,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Enum, DbEnum, Copy, Display)]
#[ExistingTypePath = "crate::schema::sql_types::HrGroup"]
/// Represents a Government of Canada pay group
pub enum HrGroup {
    EC,
    AS,
    PM,
    CR,
    PE,
    IS,
    FI,
    RES,
    EX,
    DM,
    LotsMore,
}

impl Distribution<HrGroup> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HrGroup {
        match rng.gen_range(0..13) {
            0..=4 => HrGroup::EC,
            5 => HrGroup::AS,
            6 => HrGroup::PM,
            7 => HrGroup::CR,
            8 => HrGroup::PE,
            9 => HrGroup::IS,
            10 => HrGroup::FI,
            11..=12 => HrGroup::RES,
            13 => HrGroup::EX,
            _ => HrGroup::DM,
        }
    }
}

pub fn find_people_by_requirements_met(requirements: Vec<Requirement>) -> Result<Vec<Person>> {

    let mut people_ids = Vec::new();

    let num_matches_required = *&requirements.len() as i32;

    for req in requirements {

        let caps = Capability::get_by_skill_id_and_level(req.skill_id, req.required_level)?;

        for c in caps {
            people_ids.push(c.person_id);
        };
    }

    let id_counts: HashMap<Uuid, i32> =
        people_ids.iter()
            .fold(HashMap::new(), |mut map, id| {
                *map.entry(*id).or_insert(0) += 1;
                map
            });

    let mut validated_ids: Vec<Uuid> = Vec::new();

    for (k, v) in id_counts {
        if v >= num_matches_required {
            validated_ids.push(k);
        }
    };


    Person::get_by_ids(&validated_ids)
}
