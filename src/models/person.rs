use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, Queryable, ExpressionMethods, PgTextExpressionMethods, BoolExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;

use crate::models::{Organization};

use crate::common_utils::{
    is_analyst, RoleGuard, UserRole, is_admin};

use crate::database::connection;
use crate::schema::*;

use crate::models::{Role, TeamOwnership, Team, OrgTier, OrgOwnership, Capability, Affiliation, LanguageData, 
    Publication};

use super::{Validation, Requirement};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, SimpleObject)]
#[graphql(complex)]
#[diesel(table_name = persons)]
#[diesel(belongs_to(Organization))]
/// Represents a person working in an organization
/// Referenced by Team
/// Referenced by ReportingRelationship
/// Will break out address and contact info soon
pub struct Person {
    pub id: Uuid,

    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
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
    pub country: String,

    pub organization_id: Uuid, // Organization 
    #[graphql(
        guard = "RoleGuard::new(UserRole::Admin)",
        visible = "is_admin",
    )]
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

    pub fn batch_create(persons: Vec<NewPerson>) -> Result<usize> {
        let mut conn = connection()?;

        let res = diesel::insert_into(persons::table)
            .values(persons)
            .execute(&mut conn)?;
        
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

    pub fn get_by_ids(ids: &Vec<Uuid>) -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .filter(persons::id.eq_any(ids))
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    pub fn get_all_ids() -> Result<Vec<Uuid>> {
        let mut conn = connection()?;

        let res = persons::table
            .select(persons::id)
            .load::<Uuid>(&mut conn)?;

        Ok(res)
    }

    pub fn get_count(count: i64) -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .limit(count)
            .load::<Person>(&mut conn)?;

        Ok(res)
    }

    pub fn count() -> Result<i64> {
        let mut conn = connection()?;

        let res = persons::table
            .count()
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Person>> {
        let mut conn = connection()?;

        let res = persons::table
            .filter(persons::family_name.ilike(format!("%{}%", name)).or(persons::given_name.ilike(format!("%{}%", name))))
            .load::<Person>(&mut conn)?;

        Ok(res)
    }
    
    pub fn update(&mut self) -> Result<Self> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self.clone())
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
    /// Returns the person's family or second name
    pub async fn family_name(&self) -> Result<String> {
        Ok(self.family_name.to_owned())
    }
    
    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    /// Returns the persons given or first name
    pub async fn given_name(&self) -> Result<String> {
        Ok(self.given_name.to_owned())
    }

    /// Returns the person's organization
    pub async fn organization(&self) -> Result<Organization> {
        
        Organization::get_by_id(&self.organization_id)
    }

    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    /// Returns active or inactive roles depending on the active boolean of true or false
    pub async fn inactive_roles(&self) -> Result<Vec<Role>> {
        Role::get_by_person_id(self.id, false)
    }

    /// Returns active role
    pub async fn active_roles(&self) -> Result<Vec<Role>> {
        Role::get_by_person_id(self.id, true)
    }

    /// Returns person's affiliations with other organizations
    pub async fn affiliations(&self) -> Result<Vec<Affiliation>> {
        Affiliation::get_by_person_id(self.id)
    }

    /// Returns a vector of the teams owned by this person
    pub async fn owned_teams(&self) -> Result<Vec<Team>> {
        let team_ids = TeamOwnership::get_team_ids_by_owner_id(&self.id).unwrap();

        Team::get_by_ids(&team_ids)
    }

    /// Returns a vector of the organizational tiers owned by this person
    pub async fn owned_org_tiers(&self) -> Result<Vec<OrgTier>> {
        let org_tier_ids = OrgOwnership::get_org_tier_ids_by_owner_id(&self.id).unwrap();

        OrgTier::get_by_ids(&org_tier_ids)
    }

    /*
    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
     */
    /// Returns the persons capabilities
    pub async fn capabilities(&self) -> Result<Vec<Capability>> {
        Capability::get_by_person_id(self.id)
    }

    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
    /// Returns a vector of the validations made by the person. Only available to analyst level access and above.
    pub async fn validations(&self) -> Result<Vec<Validation>> {
        Validation::get_by_validator_id(&self.id)
    }

    pub async fn publications(&self) -> Result<Vec<Publication>> {
        Publication::get_by_contributor_id(&self.id)
    }

    #[graphql(
        guard = "RoleGuard::new(UserRole::Analyst)",
        visible = "is_analyst",
    )]
    /// Returns a vector of the language results for the person
    pub async fn language_data(&self) -> Result<Vec<LanguageData>> {
        LanguageData::get_by_person_id(self.id)
    }

    pub async fn find_matches(&self) -> Result<Vec<Role>> {
        find_roles_by_requirements_met(self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject, Queryable)]
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
    pub country: String,
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
        country: String,
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
            country,
            organization_id,
            peoplesoft_id,
            orcid_id,
        }
    }
}

pub fn find_roles_by_requirements_met(person: &Person) -> Result<Vec<Role>> {

    let capabilities = Capability::get_by_person_id(person.id)?;

    let mut role_ids: Vec<Uuid> = Vec::new();

    for cap in capabilities {

        let reqs = Requirement::get_by_skill_id_and_level(cap.skill_id, cap.validated_level.unwrap())?;

        for r in reqs {
            role_ids.push(r.role_id);
        };
    }

    let id_counts: HashMap<Uuid, i32> =
        role_ids.iter()
            .fold(HashMap::new(), |mut map, id| {
                *map.entry(*id).or_insert(0) += 1;
                map
            });

    let mut validated_ids: Vec<Uuid> = Vec::new();

    for (k, v) in id_counts {
        if v >= 3 {
            validated_ids.push(k);
        }
    };

    Role::get_active_vacant_by_ids(&validated_ids)
}