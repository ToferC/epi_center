use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use crate::models::{Organization, OrgTier};

use crate::config_variables::DATE_FORMAT;

use crate::schema::*;
use crate::database::connection;

use super::{Role, Person, TeamOwnership, SkillDomain};


#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = teams)]
#[diesel(belongs_to(Organization))]
#[diesel(belongs_to(OrgTier))]
/// Referenced by Role
pub struct Team {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub org_tier_id: Uuid,
    pub primary_domain: SkillDomain,

    pub name_en: String,
    pub name_fr: String,

    pub description_en: String,
    pub description_fr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,

    // pub milestones: Uuid // Refers to Github Milestones
}

// Non Graphql
impl Team {
    pub fn create(team: &NewTeam) -> Result<Team> {
        let mut conn = connection()?;

        let res = diesel::insert_into(teams::table)
        .values(team)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(team: &NewTeam) -> Result<Team> {
        let mut conn = connection()?;

        let res = teams::table
        .filter(teams::name_en.eq(&team.name_en))
        .filter(teams::name_fr.eq(&team.name_fr))
        .filter(teams::organization_id.eq(&team.organization_id))
        .distinct()
        .first(&mut conn);
        
        let team = match res {
            Ok(p) => p,
            Err(e) => {
                // Team not found
                if e.to_string() == "NotFound" {
                    println!("{:?}", e);
                }
                let p = Team::create(team).expect("Unable to create team");
                p
            }
        };
        Ok(team)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = teams::table
            .filter(teams::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = teams::table.load::<Team>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_name(name: String) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = teams::table
            .filter(teams::name_en.ilike(format!("%{}%", name)).or(teams::name_fr.ilike(format!("%{}%", name))))
            .load::<Team>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_org_tier_id(id: &Uuid) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = teams::table
            .filter(teams::org_tier_id.eq(id))
            .load::<Team>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_ids(ids: &Vec<Uuid>) -> Result<Vec<Self>> {

        let mut conn = connection()?;

        let res = teams::table
            .filter(teams::id.eq_any(ids))
            .load::<Team>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(teams::table)
        .filter(teams::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[Object]
impl Team {
    pub async fn id(&self) -> Uuid {
        self.id
    }

    pub async fn organization(&self) -> Result<Organization> {
        Organization::get_by_id(&self.organization_id)
    }

    pub async fn organization_level(&self) -> Result<OrgTier> {
        OrgTier::get_by_id(&self.org_tier_id)
    }

    pub async fn name_english(&self) -> Result<String> {
        Ok(self.name_en.to_owned())
    }

    pub async fn name_french(&self) -> Result<String> {
        Ok(self.name_en.to_owned())
    }

    pub async fn description_english(&self) -> Result<String> {
        Ok(self.name_en.to_owned())
    }

    pub async fn description_french(&self) -> Result<String> {
        Ok(self.name_en.to_owned())
    }

    pub async fn retired_at(&self) -> Result<String> {
        match self.retired_at {
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

    pub async fn occupied_roles(&self) -> Result<Vec<Role>> {
        Role::get_occupied_by_team_id(self.id)
    }

    pub async fn vacant_roles(&self) -> Result<Vec<Role>> {
        Role::get_vacant_by_team_id(self.id)
    }

    pub async fn roles(&self) -> Result<Vec<Role>> {
        Role::get_by_team_id(self.id)
    }

    pub async fn owner(&self) -> Result<Person> {
        let team_ownership = TeamOwnership::get_by_team_id(&self.id).unwrap();

        Person::get_by_id(&team_ownership.person_id)
    }
    
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub name_en: String,
    pub name_fr: String,

    pub organization_id: Uuid,
    pub org_tier_id: Uuid,
    pub primary_domain: SkillDomain,
    
    pub description_en: String,
    pub description_fr: String,
}

impl NewTeam {

    pub fn new(
        name_en: String,
        name_fr: String,
        organization_id: Uuid,
        org_tier_id: Uuid,
        primary_domain: SkillDomain,
        description_en: String,
        description_fr: String,
    ) -> Self {
        NewTeam {
            name_en,
            name_fr,
            organization_id,
            org_tier_id,
            primary_domain,
            description_en,
            description_fr,
        }
    }
}
