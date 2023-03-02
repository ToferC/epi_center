use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::schema::*;
use crate::database::connection;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset, SimpleObject)]
#[diesel(table_name = team_ownerships)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Team))]
// Represents ownership of a team by a person
pub struct TeamOwnership {
    pub id: Uuid,
    pub person_id: Uuid,
    pub team_id: Uuid,

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    // pub milestones: Uuid // Refers to Github Milestones
}

// Non Graphql
impl TeamOwnership {
    pub fn create(team_ownership: &NewTeamOwnership) -> Result<TeamOwnership> {

        let mut conn = connection()?;

        let res = diesel::insert_into(team_ownerships::table)
        .values(team_ownership)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(team_ownership: &NewTeamOwnership) -> Result<TeamOwnership> {

        let mut conn = connection()?;

        let res = team_ownerships::table
        .filter(team_ownerships::person_id.eq(&team_ownership.person_id))
        .filter(team_ownerships::team_id.eq(&team_ownership.team_id))
        .distinct()
        .first(&mut conn);
        
        let team_ownership = match res {
            Ok(p) => p,
            Err(e) => {
                // TeamOwnership not found
                println!("{:?}", e);
                let p = TeamOwnership::create(team_ownership).expect("Unable to create team_ownership");
                p
            }
        };
        Ok(team_ownership)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = team_ownerships::table
            .filter(team_ownerships::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = team_ownerships::table
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = team_ownerships::table
            .limit(count)
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_team_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = team_ownerships::table
            .filter(team_ownerships::team_id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_team_ids_by_owner_id(id: &Uuid) -> Result<Vec<Uuid>> {
        let mut conn = connection()?;

        let res = team_ownerships::table
            .filter(team_ownerships::person_id.eq(id))
            .select(team_ownerships::team_id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {

        let mut conn = connection()?;

        let res = diesel::update(team_ownerships::table)
        .filter(team_ownerships::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Linked from HealthProfile
/// Linked to Trip
#[diesel(table_name = team_ownerships)]
pub struct NewTeamOwnership {
    pub person_id: Uuid,
    pub team_id: Uuid,

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

impl NewTeamOwnership {

    pub fn new(
        person_id: Uuid,
        team_id: Uuid,
        start_datestamp: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        NewTeamOwnership {
            person_id,
            team_id,
            start_datestamp,
            end_date,
        }
    }
}
