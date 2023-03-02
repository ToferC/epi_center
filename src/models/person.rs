use std::fmt::Debug;

use chrono::{prelude::*};
use rand::distributions::{Distribution};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, PgTextExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::models::{Organization};

use crate::common_utils::{
    is_analyst, RoleGuard, UserRole};

use crate::database::connection;
use crate::schema::*;

use super::{Role, TeamOwnership, Team, OrgTier, OrgOwnership, Capability, Affiliation};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[diesel(table_name = persons)]
#[diesel(belongs_to(Organization))]
/// Referenced by Team
/// Referenced by ReportingRelationship
pub struct Person {
    pub id: Uuid,
    pub user_id: Uuid,
    #[graphql(skip)]
    pub family_name: String,
    #[graphql(skip)]
    pub given_name: String,

    // contact info - this will be another module - just here for expediency
    pub email: String,
    pub phone: String,
    pub work_address: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,

    pub organization_id: Uuid, // Organization 
    #[graphql(visible = false)]
    pub peoplesoft_id: String,
    pub orcid_id: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}


// Non Graphql
impl Person {
    pub fn create(person: &NewPerson) -> Result<Person> {
        let mut conn = connection()?;
        let res = diesel::insert_into(persons::table)
        .values(person)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
    
    pub fn get_or_create(person: &NewPerson) -> Result<Person> {
        let mut conn = connection()?;
        let res = persons::table
        .filter(persons::family_name.eq(&person.family_name))
        .distinct()
        .first(&mut conn);
        
        let person = match res {
            Ok(p) => p,
            Err(e) => {
                // Person not found
                println!("{:?}", e);
                let p = Person::create(person).expect("Unable to create person");
                p
            }
        };
        Ok(person)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Person> {
        let mut conn = connection()?;

        let res = persons::table
            .filter(persons::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .limit(count)
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .filter(persons::family_name.ilike(format!("%{}%", name)).or(persons::given_name.ilike(format!("%{}%", name))))
            .load::<Person>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&self) -> Result<Self> {
        let mut conn = connection()?;

        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self)
        .get_result(&mut conn)?;
        
        Ok(res)
    }
}

#[ComplexObject]
impl Person {

    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
    pub async fn internal_peoplesoft_id(&self) -> Result<String> {
        Ok(self.peoplesoft_id.to_owned())
    }

    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    pub async fn family_name(&self) -> Result<String> {
        Ok(self.family_name.to_owned())
    }
    
    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    pub async fn given_name(&self) -> Result<String> {
        Ok(self.given_name.to_owned())
    }

    pub async fn organization(&self) -> Result<Organization> {
        
        Organization::get_by_id(&self.organization_id)
    }

    pub async fn roles(&self) -> Result<Vec<Role>> {
        Role::get_by_person_id(self.id)
    }

    pub async fn affiliations(&self) -> Result<Vec<Affiliation>> {
        Affiliation::get_by_person_id(self.id)
    }

    pub async fn owned_teams(&self) -> Result<Vec<Team>> {
        let team_ids = TeamOwnership::get_team_ids_by_owner_id(&self.id).unwrap();

        Team::get_by_ids(&team_ids)

    }

    pub async fn owned_org_tiers(&self) -> Result<Vec<OrgTier>> {
        let org_tier_ids = OrgOwnership::get_org_tier_ids_by_owner_id(&self.id).unwrap();

        OrgTier::get_by_ids(&org_tier_ids)

    }

    pub async fn capabilities(&self) -> Result<Vec<Capability>> {
        Capability::get_by_person_id(self.id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject)]
/// Referenced by Roles, TeamOwnership, OrgOwnership
#[diesel(table_name = persons)]
pub struct NewPerson {
    pub user_id: Uuid,
    pub family_name: String,
    pub given_name: String,
    // contact info - this will be another module - just here for expediency
    pub email: String,
    pub phone: String,
    pub work_address: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
    pub organization_id: Uuid, // Organization
    pub peoplesoft_id: String,
    pub orcid_id: String,
}

impl NewPerson {

    pub fn new(
        user_id: Uuid,
        family_name: String,
        given_name: String,
        email: String,
        phone: String,
        work_address: String,
        city: String,
        province: String,
        postal_code: String,
        organization_id: Uuid, // Organization
        peoplesoft_id: String,
        orcid_id: String,
    ) -> Self {
        NewPerson {
            user_id,
            family_name,
            given_name,
            email,
            phone,
            work_address,
            city,
            province,
            postal_code,
            organization_id,
            peoplesoft_id,
            orcid_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, InputObject)]
#[graphql(complex)]
#[diesel(table_name = persons)]
/// InputObject for Person with Option fields - only include the ones you want to update
pub struct PersonData {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub work_address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,

    pub organization_id: Option<Uuid>, // Organization 
    pub peoplesoft_id: Option<String>,
    pub orcid_id: Option<String>,

    pub updated_at: Option<NaiveDateTime>,
    pub retired_at: Option<NaiveDateTime>,
}
